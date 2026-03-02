use serde::{Deserialize, Serialize};
use quick_xml::de::from_str;
use anyhow::Result;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Narrator {
    #[serde(rename = "@ID")]
    pub id: String,
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "Bio")]
    pub bio: String,
}

pub fn parse_narrators(xml: &str) -> Result<Vec<Narrator>> {
    #[derive(Debug, Deserialize)]
    struct Root {
        #[serde(rename = "Narrator", default)]
        narrators: Vec<Narrator>,
    }
    let root: Root = from_str(xml)?;
    Ok(root.narrators)
}
