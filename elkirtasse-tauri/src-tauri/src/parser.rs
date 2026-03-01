use crate::models::{Book, Category, Chapter, Page};
use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use anyhow::Result;

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
