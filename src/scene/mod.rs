use sdl2::render::{Canvas, RenderTarget};

use crate::{Error, EventListener};
use crate::resources::Resources;

pub mod map;
pub mod main_menu;

pub trait Scene<'ttf, T: RenderTarget>: EventListener<'ttf, T> {
    fn draw(&mut self, canvas: &mut Canvas<T>, resources: &mut dyn Resources<'ttf>) -> Result<(), Error>;
}