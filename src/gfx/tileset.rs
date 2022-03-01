use std::fmt::Debug;
use std::rc::Rc;

use crate::{Error, SpriteSheet};
use crate::gfx::TextureRect;

pub struct Tileset<'tx> {
    sheet: Rc<SpriteSheet<'tx>>,
}

impl<'tx> Tileset<'tx> {
    pub fn new(sheet: Rc<SpriteSheet<'tx>>) -> Self {
        Tileset { sheet }
    }

    pub fn get_tile<T: TryInto<u32> + Debug + Copy>(&self, index: T) -> Result<TextureRect<'tx>, Error> {
        let u32_index = index.try_into().map_err(|_| Error::simple(format!("Could not convert index {:?} into u32", index)))?;
        self.sheet.get_sprite(u32_index % self.sheet.sheet_width(), u32_index / self.sheet.sheet_width())
    }

    pub fn tile_width(&self) -> u32 {
        self.sheet.sprite_width()
    }

    pub fn tile_height(&self) -> u32 {
        self.sheet.sprite_height()
    }
}