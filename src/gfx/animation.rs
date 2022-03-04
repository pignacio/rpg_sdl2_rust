use std::rc::Rc;

use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};

use crate::{Error, Point, SpriteSheet};
use crate::direction::{CardinalDirection, Direction};

pub trait Ticker {
    fn advance(&mut self, ticks: u32);
    fn reset(&mut self);
}

pub trait Animation<T: RenderTarget>: Ticker {
    fn draw_at(&self, canvas: &mut Canvas<T>, dest: Point<i32>) -> Result<(), Error>;
}


pub trait Oriented {
    fn point_to(&mut self, direction: Direction);
}


pub struct BasicCharAnimation<'sdl> {
    sheet: Rc<SpriteSheet<'sdl>>,
    current_direction: CardinalDirection,
    ticks: u32,
}

impl<'sdl> BasicCharAnimation<'sdl> {
    pub fn new(sheet: Rc<SpriteSheet<'sdl>>) -> Self {
        BasicCharAnimation { sheet, current_direction: CardinalDirection::Down, ticks: 0 }
    }
}

impl<'sdl, T: RenderTarget> Animation<T> for BasicCharAnimation<'sdl> {
    fn draw_at(&self, canvas: &mut Canvas<T>, dest: Point<i32>) -> Result<(), Error> {
        let sprite_x = (self.ticks / 200) % self.sheet.sheet_width();
        let sprite_y = match self.current_direction {
            CardinalDirection::Up => 3,
            CardinalDirection::Down => 0,
            CardinalDirection::Left => 1,
            CardinalDirection::Right => 2,
        };

        let texture_rect = self.sheet.get_sprite(sprite_x, sprite_y)?;
        let dest_rect = Rect::new(dest.x - texture_rect.width() as i32 / 2, dest.y - texture_rect.height() as i32, texture_rect.width(), texture_rect.height());
        Ok(canvas.copy(texture_rect.texture().texture(), texture_rect.rect(), dest_rect)?)
    }
}

impl<'sdl> Ticker for BasicCharAnimation<'sdl> {
    fn advance(&mut self, ticks: u32) {
        self.ticks += ticks;
    }

    fn reset(&mut self) {
        self.ticks = 0;
    }
}

impl<'sdl> Oriented for BasicCharAnimation<'sdl> {
    fn point_to(&mut self, direction: Direction) {
        if !direction.is_close_to(&self.current_direction.to_direction()) {
            self.current_direction = direction.to_cardinal();
        }
    }
}