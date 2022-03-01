use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::data::{Data};
use crate::data::gfx::{TextureData, TilesetData};

#[derive(Debug, Serialize, Deserialize)]
pub struct MapData {
    pub tileset: TilesetData,
    pub character: TextureData,
    pub tiles:  Vec<Vec<Vec<u32>>>,
}

impl Data for MapData {
    fn reroot(&mut self, base_path: &Path) {
        self.tileset.reroot(base_path);
        self.character.reroot(base_path);
    }
}