use std::rc::Rc;

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;

use crate::{Error, EventListener, GameState, InputState, TextureLoader};
use crate::event::EventResult;
use crate::gfx::Texture;

pub mod map;
pub mod main_menu;

pub trait Scene<'ttf, T: RenderTarget>: EventListener<'ttf, T> {
    fn draw(&mut self, canvas: &mut Canvas<T>) -> Result<(), Error>;
}