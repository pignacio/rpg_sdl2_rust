use std::rc::Rc;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;
use crate::{Error, EventListener, EventResult, GameState, Scene, TextureLoader};
use crate::gfx::Texture;
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
    texture_loader: TextureLoader<WindowContext>,
    selected_option: i32,
    character: Rc<Texture<'ttf>>,
}

impl<'ttf> MainMenu<'ttf> {
    pub fn new(font: Rc<Font<'ttf, 'static>>, texture_loader: TextureLoader<WindowContext>, character: Rc<Texture<'ttf>>) -> Self {
        MainMenu { font, texture_loader, selected_option: 0, character}
    }

    fn selected_option(&self) -> &MenuOption {
        return &MENU_OPTIONS[crate::utils::positive_mod(self.selected_option, MENU_OPTIONS.len())];
    }
}

impl<'ttf, T: RenderTarget> EventListener<'ttf, T> for MainMenu<'ttf> {
    fn process_event(&mut self, state: &mut GameState, event: &Event) -> Option<EventResult<'ttf, T>> {
        match event {
            Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                self.selected_option -= 1;
            }
            Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                self.selected_option += 1;
            }
            Event::KeyDown { keycode: Some(Keycode::Return | Keycode::KpEnter), .. } => {
                match *self.selected_option() {
                    MenuOption::START => { return Some(EventResult::PushScene(Box::new(MapScene::new(Rc::clone(&self.character)))));}
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
    fn draw(&mut self, canvas: &mut Canvas<T>) -> Result<(), Error> {
        for (index, option) in MENU_OPTIONS.iter().enumerate() {
            let surface = self.font.render(option.text()).blended(if option == self.selected_option() { Color::RED } else { Color::WHITE })?;
            let texture = self.texture_loader.texture_from_surface(surface)?;
            canvas.copy(texture.texture(), None, Rect::new(300, 300 + 50 * (index as i32), texture.width(), texture.height()))?;
        }
        Ok(())
    }
}