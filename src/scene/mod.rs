use sdl2::render::RenderTarget;

use crate::{Error, EventListener};
use crate::gfx::renderer::Renderer;
use crate::resources::Resources;

pub mod map;
pub mod main_menu;

pub trait Scene<'ttf, T: RenderTarget>: EventListener<'ttf, T> {
    fn draw(&mut self, renderer: &mut Renderer<T>, resources: &mut dyn Resources<'ttf>) -> Result<(), Error>;
}