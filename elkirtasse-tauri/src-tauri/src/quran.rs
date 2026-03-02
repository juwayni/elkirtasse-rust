use serde::{Deserialize, Serialize};
use quick_xml::de::from_str;
use anyhow::Result;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QuranNode {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "Item", default)]
    pub children: Vec<QuranNode>,
    #[serde(rename = "bk", default)]
    pub leaves: Vec<QuranLeaf>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct QuranLeaf {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Surah {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "Item", default)]
    pub ayas: Vec<Aya>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Aya {
    #[serde(rename = "@ID")]
    pub id: String,
    #[serde(rename = "$value")]
    pub text: String,
}

pub fn parse_ajzaa(xml: &str) -> Result<Vec<QuranNode>> {
    #[derive(Debug, Deserialize)]
    struct Root {
        #[serde(rename = "root", default)]
        nodes: Vec<QuranNode>,
    }
    let root: Root = from_str(xml)?;
    Ok(root.nodes)
}

pub fn parse_curan(xml: &str) -> Result<Vec<Surah>> {
    #[derive(Debug, Deserialize)]
    struct Root {
        #[serde(rename = "Department", default)]
        surahs: Vec<Surah>,
    }
    let root: Root = from_str(xml)?;
    Ok(root.surahs)
}
