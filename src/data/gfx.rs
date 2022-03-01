use serde::{Serialize, Deserialize};
use std::path::Path;
use std::rc::Rc;
use crate::{Data, Resources, SpriteSheet};
use crate::data::join_as_string;
use crate::gfx::texture::Texture;
use crate::gfx::tileset::Tileset;
use crate::resources::LoadResult;

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
