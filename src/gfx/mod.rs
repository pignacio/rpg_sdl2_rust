use std::rc::Rc;

use sdl2::rect::Rect;

use crate::gfx::texture::Texture;

pub mod animation;
pub mod renderer;
pub mod spritesheet;
pub mod texture;
pub mod tileset;


pub struct TextureRect<'tx> {
    texture: Rc<Texture<'tx>>,
    rect: Rect,
}

impl<'tx> TextureRect<'tx> {
    fn new(texture: Rc<Texture<'tx>>, rect: Rect) -> Self {
        TextureRect { texture, rect }
    }

    pub fn texture(&self) -> &Texture<'tx> {
        &self.texture
    }

    pub fn rect(&self) -> Rect {
        self.rect
    }

    pub fn width(&self) -> u32 {
        self.rect.width()
    }

    pub fn height(&self) -> u32 {
        self.rect.height()
    }
}