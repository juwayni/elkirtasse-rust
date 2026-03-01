use crate::models::{Book, Category, Chapter, Page};
use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::path::Path;
use std::fs;
use epub::doc::EpubDoc;

#[derive(Debug, Deserialize)]
struct GroupXml {
    #[serde(rename = "root")]
    roots: Vec<RootCategory>,
}

#[derive(Debug, Deserialize)]
struct RootCategory {
    #[serde(rename = "@Name")]
    name: String,
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "Item", default)]
    items: Vec<SubCategoryXml>,
}

#[derive(Debug, Deserialize)]
struct SubCategoryXml {
    #[serde(rename = "@Name")]
    name: String,
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "bk", default)]
    books: Vec<BookXml>,
}

#[derive(Debug, Deserialize)]
struct BookXml {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@aut")]
    author: String,
    #[serde(rename = "@betaka")]
    betaka: String,
}

pub fn parse_group_xml(xml_content: &str) -> Result<Vec<Category>> {
    let group: GroupXml = from_str(xml_content)?;
    let categories = group.roots.into_iter().map(|r| Category {
        id: r.id,
        name: r.name,
        sub_categories: r.items.into_iter().map(|s| Category {
            id: s.id,
            name: s.name,
            sub_categories: vec![],
            books: s.books.into_iter().map(|b| Book {
                id: b.id,
                name: b.name,
                title: "".to_string(),
                author: b.author,
                betaka: b.betaka,
                path: None,
            }).collect(),
        }).collect(),
        books: vec![],
    }).collect();
    Ok(categories)
}

#[derive(Debug, Deserialize)]
struct BookRoot {
    #[serde(rename = "book", default)]
    pages: Vec<PageXml>,
}

#[derive(Debug, Deserialize)]
struct PageXml {
    id: String,
    nass: String,
    part: String,
    page: String,
}

pub fn parse_book_xml(xml_content: &str) -> Result<Vec<Page>> {
    let root: BookRoot = from_str(xml_content)?;
    Ok(root.pages.into_iter().map(|p| Page {
        id: p.id,
        nass: p.nass,
        part: p.part,
        page: p.page,
    }).collect())
}

#[derive(Debug, Deserialize)]
struct TitleRoot {
    #[serde(rename = "title", default)]
    chapters: Vec<ChapterXml>,
}

#[derive(Debug, Deserialize)]
struct ChapterXml {
    id: String,
    tit: String,
    lvl: String,
}

pub fn parse_title_xml(xml_content: &str) -> Result<Vec<Chapter>> {
    let root: TitleRoot = from_str(xml_content)?;
    Ok(root.chapters.into_iter().map(|c| Chapter {
        id: c.id,
        title: c.tit,
        level: c.lvl.parse().unwrap_or(1),
    }).collect())
}

pub fn parse_epub<P: AsRef<Path>>(path: P) -> Result<(Vec<Page>, Vec<Chapter>)> {
    let mut doc = EpubDoc::new(path)?;
    let mut pages = Vec::new();
    let mut chapters = Vec::new();

    let toc = doc.toc.clone();
    for (i, item) in toc.iter().enumerate() {
        chapters.push(Chapter {
            id: item.label.clone(),
            title: item.label.clone(),
            level: 1,
        });
    }

    let n_pages = doc.get_num_pages();
    for i in 0..n_pages {
        doc.set_current_page(i);
        if let Ok(content) = doc.get_current_with_epub_uris() {
            pages.push(Page {
                id: i.to_string(),
                nass: String::from_utf8_lossy(&content).to_string(),
                page: (i + 1).to_string(),
                part: "1".to_string(),
            });
        }
    }

    Ok((pages, chapters))
}

pub fn parse_text_file<P: AsRef<Path>>(path: P) -> Result<(Vec<Page>, Vec<Chapter>)> {
    let content = fs::read_to_string(path)?;
    let mut pages = Vec::new();

    // Split into pseudo-pages for better reading experience
    let page_size = 3000;
    for (i, chunk) in content.chars().collect::<Vec<char>>().chunks(page_size).enumerate() {
        pages.push(Page {
            id: i.to_string(),
            nass: format!("<pre style='white-space: pre-wrap;'>{}</pre>", chunk.iter().collect::<String>()),
            page: (i + 1).to_string(),
            part: "1".to_string(),
        });
    }

    Ok((pages, vec![]))
}
