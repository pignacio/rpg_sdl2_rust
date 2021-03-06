use std::rc::Rc;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::RenderTarget;
use sdl2::ttf::Font;

use crate::{Error, Event, EventListener, EventResult, GameState, MapData, Resources, Scene, SpriteSheet};
use crate::gfx::animation::BasicCharAnimation;
use crate::gfx::renderer::Renderer;
use crate::keymap::Action;
use crate::point::IntPoint;
use crate::scene::map::MapScene;

#[derive(PartialEq)]
enum MenuOption {
    START,
    SETTINGS,
    QUIT,
}

impl MenuOption {
    fn text(&self) -> &str {
        match self {
            MenuOption::START => "Start!",
            MenuOption::SETTINGS => "Settings",
            MenuOption::QUIT => "Quit",
        }
    }
}

const MENU_OPTIONS: [MenuOption; 3] = [MenuOption::START, MenuOption::SETTINGS, MenuOption::QUIT];

pub struct MainMenu<'ttf> {
    font: Rc<Font<'ttf, 'static>>,
    map_data: MapData,
    selected_option: i32,
}

impl<'ttf> MainMenu<'ttf> {
    pub fn new(font: Rc<Font<'ttf, 'static>>, map_data: MapData) -> Self {
        MainMenu { font, map_data, selected_option: 0 }
    }

    fn selected_option(&self) -> &MenuOption {
        return &MENU_OPTIONS[crate::utils::positive_mod(self.selected_option, MENU_OPTIONS.len())];
    }
}

impl<'ttf, T: RenderTarget> EventListener<'ttf, T> for MainMenu<'ttf> {
    fn process_event(&mut self, state: &mut GameState<'ttf>, event: &Event) -> Option<EventResult<'ttf, T>> {
        match event {
            Event::ActionDown { action: Action::Up } => {
                self.selected_option -= 1;
            }
            Event::ActionDown { action: Action::Down } => {
                self.selected_option += 1;
            }
            Event::ActionDown { action: Action::Back } => {
                state.running = false;
            }
            Event::ActionDown { action: Action::Select } => {
                match *self.selected_option() {
                    MenuOption::START => {
                        let character = self.map_data.character.load(&mut state.resources).unwrap();
                        let sprite_width = character.width() / 4;
                        let sprite_height = character.height() / 4;
                        let animation = BasicCharAnimation::new(Rc::new(SpriteSheet::new(character, sprite_width, sprite_height)));
                        let tiles = self.map_data.tileset.load(&mut state.resources).unwrap();
                        return Some(EventResult::PushScene(Box::new(MapScene::new(animation, tiles, self.map_data.tiles.to_tiles()))));
                    }
                    MenuOption::QUIT => state.running = false,
                    MenuOption::SETTINGS => println!("No settings for you!"),
                }
            }
            _ => {}
        }
        None
    }
}

impl<'ttf, T: RenderTarget> Scene<'ttf, T> for MainMenu<'ttf> {
    fn draw(&mut self, renderer: &mut Renderer<T>, resources: &mut dyn Resources<'ttf>) -> Result<(), Error> {
        renderer.with_offset(IntPoint::new(300, 300), |renderer| {
            for (index, option) in MENU_OPTIONS.iter().enumerate() {
                let surface = self.font.render(option.text()).blended(if option == self.selected_option() { Color::RED } else { Color::WHITE })?;
                let texture = resources.texture_from_surface(surface)?;
                renderer.copy(&texture, None, Rect::new(0, 50 * (index as i32), texture.width(), texture.height()))?;
            }
            Ok(())
        })
    }
}