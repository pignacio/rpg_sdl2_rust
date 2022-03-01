use serde::{Serialize, Deserialize};
use std::path::Path;
use sdl2::ttf::Font;
use crate::{Data, Resources};
use crate::data::join_as_string;
use crate::resources::LoadResult;

#[derive(Debug, Serialize, Deserialize)]
pub enum FontData {
    Inline { path: String, size: u16 },
    Ref { id: String },
}

impl FontData {
    pub fn load<'ttf>(&self, resources: &mut dyn Resources<'ttf>) -> LoadResult<Font<'ttf, 'static>> {
        match self {
            FontData::Inline { path, size } => Ok(resources.load_font(path.as_ref(), *size)?),
            FontData::Ref { id } => resources.get_font(id),
        }
    }
}

impl Data for FontData {
    fn reroot(&mut self, base_path: &Path) {
        match self {
            FontData::Inline { path , ..} => {
                *path = join_as_string(base_path, &path);
            },
            FontData::Ref{ .. } => {},
        }
    }
}