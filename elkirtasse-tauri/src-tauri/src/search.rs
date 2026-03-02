use tantivy::schema::*;
use tantivy::{Index, IndexWriter, ReloadPolicy, Term};
use anyhow::Result;
use std::path::Path;
use crate::models::SearchResult;

pub struct SearchEngine {
    pub index: Index,
    pub schema: Schema,
}

impl SearchEngine {
    pub fn new<P: AsRef<Path>>(index_path: P) -> Result<Self> {
        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("book_id", STRING | STORED);
        schema_builder.add_text_field("title", TEXT | STORED);
        schema_builder.add_text_field("author", TEXT | STORED);
        schema_builder.add_text_field("content", TEXT | STORED);
        schema_builder.add_text_field("part", STRING | STORED);
        schema_builder.add_text_field("page", STRING | STORED);
        let schema = schema_builder.build();

        let index = if index_path.as_ref().exists() {
            Index::open_in_dir(index_path)?
        } else {
            std::fs::create_dir_all(&index_path)?;
            Index::create_in_dir(index_path, schema.clone())?
        };

        Ok(Self { index, schema })
    }

    pub fn index_page_with_writer(&self, writer: &mut IndexWriter, book_id: &str, title: &str, author: &str, content: &str, part: &str, page: &str) -> Result<()> {
        let book_id_field = self.schema.get_field("book_id").unwrap();
        let title_field = self.schema.get_field("title").unwrap();
        let author_field = self.schema.get_field("author").unwrap();
        let content_field = self.schema.get_field("content").unwrap();
        let part_field = self.schema.get_field("part").unwrap();
        let page_field = self.schema.get_field("page").unwrap();

        writer.add_document(tantivy::doc!(
            book_id_field => book_id,
            title_field => title,
            author_field => author,
            content_field => content,
            part_field => part,
            page_field => page,
        ))?;
        Ok(())
    }

    pub fn search(&self, query_str: &str, book_filter: Option<Vec<String>>) -> Result<Vec<SearchResult>> {
        let reader = self.index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()?;

        let searcher = reader.searcher();
        let content_field = self.schema.get_field("content").unwrap();
        let book_id_field = self.schema.get_field("book_id").unwrap();

        let query_parser = tantivy::query::QueryParser::for_index(&self.index, vec![content_field]);
        let base_query = query_parser.parse_query(query_str)?;

        let query: Box<dyn tantivy::query::Query> = if let Some(book_ids) = book_filter {
            let mut filter_queries: Vec<Box<dyn tantivy::query::Query>> = Vec::new();
            for id in book_ids {
                filter_queries.push(Box::new(tantivy::query::TermQuery::new(
                    Term::from_field_text(book_id_field, &id),
                    IndexRecordOption::Basic,
                )));
            }
            let filter_query = Box::new(tantivy::query::BooleanQuery::new(
                filter_queries.into_iter().map(|q| (tantivy::query::Occur::Should, q)).collect()
            ));
            Box::new(tantivy::query::BooleanQuery::new(vec![
                (tantivy::query::Occur::Must, base_query),
                (tantivy::query::Must, filter_query),
            ]))
        } else {
            base_query
        };

        let top_docs = searcher.search(&query, &tantivy::collector::TopDocs::with_limit(100))?;

        let mut results = Vec::new();
        for (_score, doc_address) in top_docs {
            let retrieved_doc: tantivy::TantivyDocument = searcher.doc(doc_address)?;

            let content = retrieved_doc.get_first(content_field).and_then(|v| v.as_str()).unwrap_or("");
            // Basic snippet with search terms bolded
            let snippet = self.generate_highlighted_snippet(content, query_str);

            results.push(SearchResult {
                book_id: retrieved_doc.get_first(self.schema.get_field("book_id").unwrap()).and_then(|v| v.as_str()).unwrap_or("").to_string(),
                book_title: retrieved_doc.get_first(self.schema.get_field("title").unwrap()).and_then(|v| v.as_str()).unwrap_or("").to_string(),
                author: retrieved_doc.get_first(self.schema.get_field("author").unwrap()).and_then(|v| v.as_str()).unwrap_or("").to_string(),
                part: retrieved_doc.get_first(self.schema.get_field("part").unwrap()).and_then(|v| v.as_str()).unwrap_or("").to_string(),
                page: retrieved_doc.get_first(self.schema.get_field("page").unwrap()).and_then(|v| v.as_str()).unwrap_or("").to_string(),
                snippet,
            });
        }

        Ok(results)
    }

    fn generate_highlighted_snippet(&self, content: &str, query: &str) -> String {
        let terms: Vec<&str> = query.split_whitespace().collect();
        let mut snippet: String = content.chars().take(300).collect();

        // Sanitize snippet before highlighting to prevent XSS
        snippet = ammonia::clean(&snippet);

        for term in terms {
            let re = regex::RegexBuilder::new(&regex::escape(term))
                .case_insensitive(true)
                .build()
                .unwrap();
            snippet = re.replace_all(&snippet, "<strong>$0</strong>").to_string();
        }
        snippet
    }
}
