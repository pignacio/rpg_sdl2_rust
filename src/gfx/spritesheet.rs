use std::borrow::Borrow;
use std::rc::Rc;

use sdl2::rect::Rect;

use crate::Error;
use crate::gfx::texture::Texture;
use crate::gfx::TextureRect;

pub struct SpriteSheet<'tx> {
    sheet: Rc<Texture<'tx>>,
    sprite_width: u32,
    sprite_height: u32,
    sheet_width: u32,
    sheet_height: u32,
}

impl<'tx> SpriteSheet<'tx> {
    pub fn new(sheet: Rc<Texture<'tx>>, sprite_width: u32, sprite_height: u32) -> Self {
        let sheet_width = sheet.width() / sprite_width;
        let sheet_height = sheet.height() / sprite_height;
        println!("Creating spritesheet. total: {},{}, sprite: {},{}, sheet:{},{}",
            sheet.width(), sheet.height(), sprite_width, sprite_height, sheet_width, sheet_height);
        SpriteSheet {
            sheet,
            sprite_width,
            sprite_height,
            sheet_width,
            sheet_height,
        }
    }

    pub fn get_sprite(&self, x: u32, y: u32) -> Result<TextureRect<'tx>, Error> {
        if x > self.sheet_width {
            Err(Error::simple(format!("Asked for x={} > {} in sheet total width {} with sprite width {}", x, self.sheet_width, self.sheet.width(), self.sprite_width)))
        } else if y > self.sheet_height {
            Err(Error::simple(format!("Asked for y={} > {} in sheet total height {} with sprite height {}", y, self.sheet_height, self.sheet.height(), self.sprite_height)))
        } else {
            Ok(TextureRect::new(self.sheet.clone(), Rect::new((x * self.sprite_width) as i32, (y * self.sprite_height) as i32, self.sprite_width, self.sprite_height)))
        }
    }

    pub fn sheet_width(&self) -> u32 {
        self.sheet_width
    }

    pub fn sheet_height(&self) -> u32 {
        self.sheet_height
    }

    pub fn sprite_width(&self) -> u32 {
        self.sprite_width
    }

    pub fn sprite_height(&self) -> u32 {
        self.sprite_height
    }
}