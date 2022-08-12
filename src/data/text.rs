use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::data::Data;
use crate::data::font::FontData;

#[derive(Debug, Serialize, Deserialize)]
pub enum TextBitData {
    Text { font: Option<FontData>, text: String },
    FontChange { font: FontData },
}

impl Data for TextBitData {
    fn reroot(&mut self, base_path: &Path) {
        match self {
            TextBitData::Text { font, .. } => font.reroot(base_path),
            TextBitData::FontChange { font } => font.reroot(base_path),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextLineData {
    pub font: Option<FontData>,
    pub parts: Vec<TextBitData>,
}


impl Data for TextLineData {
    fn reroot(&mut self, base_path: &Path) {
        self.font.reroot(base_path);
        self.parts.iter_mut().for_each(|part| part.reroot(base_path));
    }
}