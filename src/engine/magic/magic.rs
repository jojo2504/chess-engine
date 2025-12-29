use std::{fs::{self, File}, io::Read, path::Path};

use serde::Deserialize;

use crate::engine::models::board::Square;

#[derive(Debug, Deserialize)]
pub struct Magic {
    pub square: Square,
    #[serde(rename = "magicNumber")]
    pub magic_number: u64,
    pub mask: u64,
}

impl Magic {
    pub fn load_magic_table(path: &str) -> anyhow::Result<Vec<Magic>> {
        let json_str = fs::read_to_string(path)?;
        let magics: Vec<Magic> = serde_json::from_str(&json_str)?;
        Ok(magics)
    }
}