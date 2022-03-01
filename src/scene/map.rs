use std::rc::Rc;

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};

use crate::{Error, EventListener, EventResult, GameState, InputState, Scene};
use crate::gfx::Texture;

pub struct MapScene<'tx> {
    character: Rc<Texture<'tx>>,
    x: f32,
    y: f32,
}

impl<'tx> MapScene<'tx> {
    pub fn new(character: Rc<Texture<'tx>>) -> Self {
        MapScene { character, x: 0., y: 0. }
    }
}

impl<'tx, T: RenderTarget> EventListener<'tx, T> for MapScene<'tx> {
    fn process_event(&mut self, _state: &mut GameState, event: &Event) -> Option<EventResult<'tx, T>> {
        match event {
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => Some(EventResult::PopScene),
            _ => None,
        }
    }


    fn batch_end(&mut self, state: &mut GameState, input: &InputState) -> Option<EventResult<'tx, T>> {
        let distance: f32 = state.ticks_to_process as f32 * 0.3;
        if input.keyboard.is_scancode_pressed(Scancode::Up) {
            self.y -= distance;
        }
        if input.keyboard.is_scancode_pressed(Scancode::Down) {
            self.y += distance;
        }
        if input.keyboard.is_scancode_pressed(Scancode::Left) {
            self.x -= distance;
        }
        if input.keyboard.is_scancode_pressed(Scancode::Right) {
            self.x += distance;
        }
        None
    }
}

impl<'tx, T: RenderTarget> Scene<'tx, T> for MapScene<'tx> {
    fn draw(&mut self, canvas: &mut Canvas<T>) -> Result<(), Error> {
        let src = Rect::new(0, 0, 16, 32);
        let dst = Rect::new(self.x as i32, self.y as i32, 32, 64);
        canvas.copy(self.character.texture(), Some(src), Some(dst))?;
        Ok(())
    }
}