use jetdb::Database;
use anyhow::{Result, anyhow};
use std::path::Path;
use crate::models::{Book, Page};

pub fn normalize_arabic(text: &str) -> String {
    let mut normalized = text.to_string();
    let diacritics = [
        "\u{064B}", "\u{064C}", "\u{064D}", "\u{064E}",
        "\u{064F}", "\u{0650}", "\u{0651}", "\u{0652}",
    ];
    for d in diacritics.iter() {
        normalized = normalized.replace(d, "");
    }
    normalized = normalized.replace("أ", "ا");
    normalized = normalized.replace("إ", "ا");
    normalized = normalized.replace("آ", "ا");
    normalized
}

pub struct MdbBook {
    pub metadata: Book,
    pub pages: Vec<Page>,
}

pub fn read_mdb_book<P: AsRef<Path>>(path: P) -> Result<MdbBook> {
    let db = Database::open(&path)?;

    // In Maktaba Shamela .bok files:
    // Table 'Main' usually contains book metadata
    // Table 'book' or 'Book' usually contains the text content

    let mut pages = Vec::new();

    let table_name = if db.has_table("book") { "book" } else if db.has_table("Book") { "Book" } else { "" };

    if !table_name.is_empty() {
        if let Some(table) = db.get_table(table_name) {
            let columns = table.columns();
            let nass_idx = columns.iter().position(|c| c.name().to_lowercase() == "nass").unwrap_or(1);
            let page_idx = columns.iter().position(|c| c.name().to_lowercase() == "page").unwrap_or(2);
            let part_idx = columns.iter().position(|c| c.name().to_lowercase() == "part").unwrap_or(3);

            for (i, row) in table.rows().enumerate() {
                let nass = row.get_value(nass_idx).and_then(|v| v.as_string()).unwrap_or_default();
                let page_num = row.get_value(page_idx).and_then(|v| v.as_string()).unwrap_or_else(|| (i+1).to_string());
                let part_num = row.get_value(part_idx).and_then(|v| v.as_string()).unwrap_or_else(|| "1".to_string());

                pages.push(Page {
                    id: i.to_string(),
                    nass,
                    page: page_num,
                    part: part_num,
                });
            }
        }
    } else {
        return Err(anyhow!("Neither table 'book' nor 'Book' found in MDB file"));
    }

    let mut title = "Imported MDB Book".to_string();
    let mut author = "Unknown".to_string();
    let mut betaka = "".to_string();

    if let Some(main_table) = db.get_table("Main") {
        let cols = main_table.columns();
        let title_idx = cols.iter().position(|c| c.name().to_lowercase() == "bk").unwrap_or(1);
        let auth_idx = cols.iter().position(|c| c.name().to_lowercase() == "auth").unwrap_or(2);
        let betaka_idx = cols.iter().position(|c| c.name().to_lowercase() == "betaka").unwrap_or(3);

        if let Some(row) = main_table.rows().next() {
            title = row.get_value(title_idx).and_then(|v| v.as_string()).unwrap_or(title);
            author = row.get_value(auth_idx).and_then(|v| v.as_string()).unwrap_or(author);
            betaka = row.get_value(betaka_idx).and_then(|v| v.as_string()).unwrap_or(betaka);
        }
    }

    Ok(MdbBook {
        metadata: Book {
            id: format!("mdb_{}", path.as_ref().file_stem().unwrap().to_string_lossy()),
            name: path.as_ref().file_name().unwrap().to_string_lossy().to_string(),
            title: "Imported MDB Book".to_string(),
            author: "Unknown".to_string(),
            betaka: "".to_string(),
            path: Some(path.as_ref().to_string_lossy().to_string()),
        },
        pages,
    })
}
