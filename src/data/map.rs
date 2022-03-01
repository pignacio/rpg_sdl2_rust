use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::data::{Data};
use crate::data::gfx::{TextureData, TilesetData};
use crate::scene::map;


#[derive(Debug, Serialize, Deserialize)]
pub enum LayerData {
    Simple{layer: map::Layer<u32>},
    Condensed{rows: Vec<String>},
}

impl LayerData {
    pub fn to_layer(&self) -> map::Layer<u32> {
        match self {
            LayerData::Simple { layer} => layer.clone(),
            LayerData::Condensed { rows } => rows.iter()
                .map(|row| row.split(",")
                    .map(|x| x.trim().parse::<u32>()
                        .expect(&format!("Could not parse u32 value from '{}'", x)))
                    .collect())
                .collect(),
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub enum TilesData {
    Simple{tiles: map::Tiles<u32>},
    ByLayer{layers: Vec<LayerData>},
}

impl TilesData {
    pub fn to_tiles(&self) -> map::Tiles<u32> {
        match self {
            TilesData::Simple{tiles} => tiles.clone(),
            TilesData::ByLayer { layers} => layers.iter().map(|layer| layer.to_layer()).collect()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapData {
    pub tileset: TilesetData,
    pub character: TextureData,
    pub tiles:  TilesData,
}

impl Data for MapData {
    fn reroot(&mut self, base_path: &Path) {
        self.tileset.reroot(base_path);
        self.character.reroot(base_path);
    }
}