use std::rc::Rc;

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};

use crate::{Error, EventListener, EventResult, GameState, InputState, Scene};
use crate::gfx::spritesheet::SpriteSheet;
use crate::gfx::texture::Texture;
use crate::gfx::tileset::Tileset;

pub struct MapScene<'tx> {
    character: Rc<Texture<'tx>>,
    sprites: Rc<SpriteSheet<'tx>>,
    tiles: Tileset<'tx>,
    x: f32,
    y: f32,
    sprite_x: u32,
    sprite_y: u32,
}

const LAYER_1: [[u32; 20]; 20] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
];

const LAYER_2: [[u32; 20]; 20] = [
    [143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143],
    [143, 143, 2, 3, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143],
    [143, 143, 143, 143, 143, 3, 4, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143],
    [143, 143, 143, 143, 143, 23, 24, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143],
    [143, 143, 143, 143, 143, 43, 44, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143],
    [143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143],
    [143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 40, 41, 42, 43, 143, 143, 143, 143, 143, 143],
    [143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 48, 49, 50, 51, 143, 143, 143, 143, 143, 143],
    [143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 56, 57, 58, 59, 143, 143, 143, 143, 143, 143],
    [143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 64, 65, 66, 67, 143, 143, 143, 143, 143, 143],
    [143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 72, 73, 74, 75, 143, 143, 143, 143, 143, 143],
    [143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143],
    [143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143],
    [143, 143, 143, 143, 104, 105, 106, 107, 108, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143],
    [143, 143, 143, 143, 112, 113, 114, 115, 116, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143],
    [143, 143, 143, 143, 120, 121, 122, 123, 124, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143],
    [143, 143, 143, 143, 128, 129, 130, 131, 132, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143],
    [143, 143, 143, 143, 136, 137, 138, 139, 140, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143],
    [143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143],
    [143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143, 143]
];

impl<'tx> MapScene<'tx> {
    pub fn new(character: Rc<Texture<'tx>>, sprites: Rc<SpriteSheet<'tx>>) -> Self {
        MapScene { character, sprites: sprites.clone(), tiles: Tileset::new(sprites), x: 0., y: 0., sprite_x: 0, sprite_y: 0 }
    }

    fn print<T: RenderTarget>(&self, layer: [[u32; 20]; 20], canvas: &mut Canvas<T>) -> Result<(), Error> {
        for (y, row) in layer.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                let dst = Rect::new(
                    (x as u32 * self.tiles.tile_width()) as i32,
                    (y as u32 * self.tiles.tile_height()) as i32,
                    self.tiles.tile_width(),
                    self.tiles.tile_height(),
                );

                self.tiles.get_tile(*value)
                    .and_then(|tile_rect| {
                        canvas.copy(tile_rect.texture().texture(), tile_rect.rect(), dst)?;
                        Ok(())
                    })?;
            }
        }
        Ok(())
    }
}

impl<'tx, T: RenderTarget> EventListener<'tx, T> for MapScene<'tx> {
    fn process_event(&mut self, _state: &mut GameState, event: &Event) -> Option<EventResult<'tx, T>> {
        match event {
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => Some(EventResult::PopScene),
            Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                if self.sprite_x > 0 { self.sprite_x -= 1; }
                None
            }
            Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                self.sprite_x += 1;
                None
            }
            Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                self.sprite_y += 1;
                None
            }
            Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                if self.sprite_y > 0 { self.sprite_y -= 1; }
                None
            }
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
        self.print(LAYER_1, canvas)?;
        self.print(LAYER_2, canvas)?;

        let src = Rect::new(0, 0, 16, 32);
        let dst = Rect::new(self.x as i32, self.y as i32, 32, 64);
        canvas.copy(self.character.texture(), Some(src), Some(dst))?;
        Ok(())
    }
}