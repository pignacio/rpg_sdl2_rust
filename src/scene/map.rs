use std::rc::Rc;

use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};

use crate::{Error, Event, EventListener, EventResult, GameState, InputState, Point, Resources, Scene};
use crate::gfx::texture::Texture;
use crate::gfx::tileset::Tileset;
use crate::keymap::Action;

pub type Layer<T> = Vec<Vec<T>>;
pub type Tiles<T> = Vec<Layer<T>>;

pub struct MapScene<'tx> {
    character: Rc<Texture<'tx>>,
    tileset: Rc<Tileset<'tx>>,
    tiles: Tiles<u32>,
    character_position: Point<f32>,
    sprite_x: u32,
    sprite_y: u32,
}

pub const LAYER_1: [[u32; 20]; 20] = [
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

pub const LAYER_2: [[u32; 20]; 20] = [
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

pub fn to_vec(layer: &[[u32; 20]; 20]) -> Vec<Vec<u32>> {
    layer.iter().map(|row| row.to_vec()).collect()
}

impl<'tx> MapScene<'tx> {
    pub fn new(character: Rc<Texture<'tx>>, tileset: Rc<Tileset<'tx>>, tiles: Vec<Vec<Vec<u32>>>) -> Self {
        MapScene { character, tileset, tiles, character_position: Point::new(0., 0.), sprite_x: 0, sprite_y: 0 }
    }

    pub fn old_new(character: Rc<Texture<'tx>>, tileset: Rc<Tileset<'tx>>) -> Self {
        let mut tiles = Vec::new();
        tiles.push(to_vec(&LAYER_1));
        tiles.push(to_vec(&LAYER_2));
        MapScene { character, tileset, tiles: tiles, character_position: Point::new(0., 0.), sprite_x: 0, sprite_y: 0 }
    }

    fn print<T: RenderTarget>(&self, layer: &Vec<Vec<u32>>, canvas: &mut Canvas<T>) -> Result<(), Error> {
        for (y, row) in layer.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                let dst = Rect::new(
                    (x as u32 * self.tileset.tile_width()) as i32,
                    (y as u32 * self.tileset.tile_height()) as i32,
                    self.tileset.tile_width(),
                    self.tileset.tile_height(),
                );

                self.tileset.get_tile(*value)
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
            Event::ActionDown { action: Action::Back } => Some(EventResult::PopScene),
            Event::ActionDown { action: Action::Left } => {
                if self.sprite_x > 0 { self.sprite_x -= 1; }
                None
            }
            Event::ActionDown { action: Action::Right } => {
                self.sprite_x += 1;
                None
            }
            Event::ActionDown { action: Action::Down }  => {
                self.sprite_y += 1;
                None
            }
            Event::ActionDown { action: Action::Up }  => {
                if self.sprite_y > 0 { self.sprite_y -= 1; }
                None
            }
            _ => None,
        }
    }


    fn batch_end(&mut self, state: &mut GameState, input: &InputState) -> Option<EventResult<'tx, T>> {
        let distance: f32 = state.ticks_to_process as f32 * 0.3;
        if input.is_action_pressed(Action::Up) {
            self.character_position.y -= distance;
        }
        if input.is_action_pressed(Action::Down) {
            self.character_position.y += distance;
        }
        if input.is_action_pressed(Action::Left) {
            self.character_position.x -= distance;
        }
        if input.is_action_pressed(Action::Right) {
            self.character_position.x += distance;
        }
        None
    }
}

impl<'tx, T: RenderTarget> Scene<'tx, T> for MapScene<'tx> {
    fn draw(&mut self, canvas: &mut Canvas<T>, _resources: &mut dyn Resources<'tx>) -> Result<(), Error> {
        for layer in &self.tiles {
            self.print(layer, canvas)?;
        }

        let src = Rect::new(0, 0, 16, 32);
        let dst = Rect::new(self.character_position.x as i32, self.character_position.y as i32, 32, 64);
        canvas.copy(self.character.texture(), Some(src), Some(dst))?;
        Ok(())
    }
}