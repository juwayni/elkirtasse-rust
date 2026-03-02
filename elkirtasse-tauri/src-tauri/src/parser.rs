use crate::models::{Book, Category, Chapter, Page};
use quick_xml::de::from_str;
use quick_xml::se::to_string;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::path::Path;
use std::fs;
use epub::doc::EpubDoc;

#[derive(Debug, Deserialize, Serialize)]
pub struct GroupXml {
    #[serde(rename = "root")]
    pub roots: Vec<RootCategory>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RootCategory {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "Item", default)]
    pub items: Vec<SubCategoryXml>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubCategoryXml {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "bk", default)]
    pub books: Vec<BookXml>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BookXml {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@aut")]
    pub author: String,
    #[serde(rename = "@betaka")]
    pub betaka: String,
}

pub fn add_book_to_group_xml(xml_content: &str, category_id: &str, book: BookXml) -> Result<String> {
    let mut group: GroupXml = from_str(xml_content)?;
    let mut found = false;
    for root in &mut group.roots {
        for sub in &mut root.items {
            if sub.id == category_id {
                sub.books.push(book.clone());
                found = true;
                break;
            }
        }
        if found { break; }
    }

    // If category not found, add to the first subcategory of the first root
    if !found && !group.roots.is_empty() && !group.roots[0].items.is_empty() {
        group.roots[0].items[0].books.push(book);
    }

    let new_xml = to_string(&group)?;
    Ok(new_xml)
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
                progress: None,
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
