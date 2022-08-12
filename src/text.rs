use std::rc::Rc;

use sdl2::rect::Rect;
use sdl2::render::RenderTarget;

use crate::error::Error;
use crate::gfx::animation::{Drawable, Ticker};
use crate::gfx::renderer::Renderer;
use crate::gfx::texture::Texture;
use crate::gfx::TextureRect;
use crate::point::Point;

#[derive(Debug)]
pub struct BitState {
    pub last_frame: u32,
    pub position: u32,
}

pub struct RenderedTextBit<'sdl> {
    texture: Rc<Texture<'sdl>>,
    states: Vec<BitState>,
}

impl<'sdl> RenderedTextBit<'sdl> {
    pub fn new(texture: Rc<Texture<'sdl>>, states: Vec<BitState>) -> Self {
        debug_assert!(!states.is_empty());
        RenderedTextBit { texture, states }
    }

    pub fn get_frame(&self, frame: u32) -> TextureRect {
        TextureRect::new(self.texture.clone(), Rect::new(0, 0, self.get_width(frame), self.texture.height()))
    }

    fn get_width(&self, frame: u32) -> u32 {
        for state in &self.states {
            if state.last_frame >= frame {
                return state.position;
            }
        }
        return self.width();
    }

    pub fn height(&self) -> u32 {
        return self.texture.height();
    }

    pub fn width(&self) -> u32 {
        return self.texture.width();
    }

    pub fn last_frame(&self) -> u32 {
        return self.states.last().map(|s| s.last_frame).unwrap();
    }
}

pub struct RenderedTextLine<'sdl> {
    bits: Vec<RenderedTextBit<'sdl>>,
    height: u32,
    frame: u32,
}

impl<'sdl> RenderedTextLine<'sdl> {
    pub fn new(bits: Vec<RenderedTextBit<'sdl>>) -> Self {
        debug_assert!(!bits.is_empty());
        let height = bits.iter().map(|b| b.height()).max().unwrap();
        RenderedTextLine {
            bits,
            height,
            frame: 0,
        }
    }
}

impl<'sdl, T: RenderTarget> Drawable<T> for RenderedTextLine<'sdl> {
    fn draw_at(&self, _renderer: &mut Renderer<T>, dest: Point<i32>) -> Result<(), Error> {
        let mut remaining_frames = self.frame;
        let height = self.height as i32;
        let mut current_x = dest.x;
        for bit in &self.bits {
            if remaining_frames > bit.last_frame() {
                let rect = bit.get_frame(bit.last_frame() + 1);
                let src = rect.rect();
                _renderer.copy(rect.texture(), src, Rect::new(current_x, dest.y + height - src.height() as i32, src.width(), src.height()))?;

                current_x += src.width() as i32;
                remaining_frames -= bit.last_frame();
            } else {
                let rect = bit.get_frame(remaining_frames);
                let src = rect.rect();
                _renderer.copy(rect.texture(), src, Rect::new(current_x, dest.y + height - src.height() as i32, src.width(), src.height()))?;
                break;
            }
        }
        Ok(())
    }
}

impl<'sdl> Ticker for RenderedTextLine<'sdl> {
    fn advance(&mut self, ticks: u32) {
        self.frame += ticks;
        if self.frame > 5000 {
            self.reset();
        }
    }

    fn reset(&mut self) {
        self.frame = 0;
    }
}