use std::path::Path;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use crate::gfx::texture::Texture;
use crate::gfx::tileset::Tileset;

use crate::resources::Resources;
use crate::{Error, SpriteSheet};
use crate::data::{Data, join_as_string};

type LoadResult<T> = Result<Rc<T>, Error>;

#[derive(Debug, Serialize, Deserialize)]
pub enum TextureData {
    Inline { path: String },
    Ref { id: String },
}

impl TextureData {
    pub fn load<'ttf>(&self, resources: &mut dyn Resources<'ttf>) -> LoadResult<Texture<'ttf>> {
        match self {
            TextureData::Inline { path } => Ok(resources.load_texture(path.as_ref())?),
            TextureData::Ref { id } => resources.get_texture(id),
        }
    }
}

impl Data for TextureData {
    fn reroot(&mut self, base_path: &Path) {
        match self {
            TextureData::Inline { path } => {
                *path = join_as_string(base_path, &path);
            },
            TextureData::Ref{ .. } => {},
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TilesetData {
    Inline { texture: TextureData, width: u32, height: u32 },
    Ref { id: String },
}

impl TilesetData {
    pub fn load<'ttf>(&self, resources: &mut dyn Resources<'ttf>) -> LoadResult<Tileset<'ttf>>{
        match self {
            TilesetData::Inline { texture, width, height } => {
                let texture = texture.load(resources)?;
                let sheet = SpriteSheet::new(texture, *width, *height);
                Ok(Rc::new(Tileset::new(Rc::new(sheet))))
            }
            TilesetData::Ref { id } => {
                resources.get_tileset(&id)
            }
        }
    }
}

impl Data for TilesetData {
    fn reroot(&mut self, base_path: &Path) {
        match self {
            TilesetData::Inline { texture, ..} => texture.reroot(base_path),
            TilesetData::Ref {..} => {},
        }
    }
}

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