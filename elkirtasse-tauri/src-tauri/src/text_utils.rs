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
    // Table 'book' usually contains the text content

    let mut pages = Vec::new();

    // Attempt to read 'book' table
    if let Some(table) = db.get_table("book") {
        for (i, row) in table.rows().enumerate() {
            // Conceptually mapping columns (id, nass, page, part)
            // Note: jetdb row access depends on its API specifics for column indexing/naming
            let nass = row.get_value(1).and_then(|v| v.as_string()).unwrap_or_default();
            let page_num = row.get_value(2).and_then(|v| v.as_string()).unwrap_or_else(|| i.to_string());
            let part_num = row.get_value(3).and_then(|v| v.as_string()).unwrap_or_else(|| "1".to_string());

            pages.push(Page {
                id: i.to_string(),
                nass,
                page: page_num,
                part: part_num,
            });
        }
    } else {
        return Err(anyhow!("Table 'book' not found in MDB file"));
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
