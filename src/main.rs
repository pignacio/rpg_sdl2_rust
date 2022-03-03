use std::path::Path;

use sdl2::pixels::Color;
use sdl2::render::{Canvas, RenderTarget};
use sdl2::video::Window;

use gfx::texture::TextureLoader;

use crate::data::{Data, GameConfig};
use crate::data::map::{MapData};
use crate::error::Error;
use crate::event::{EventListener, EventResult, GameState, PumpProcessor, QuitListener, InputState, Event};
use crate::gfx::spritesheet::SpriteSheet;
use crate::keymap::hardcoded_keymap;
use crate::point::Point;
use crate::resources::{CachedResources, Resources};
use crate::scene::{main_menu::MainMenu, Scene};

pub mod data;
pub mod error;
pub mod event;
pub mod gfx;
pub mod keymap;
pub mod point;
pub mod resources;
pub mod scene;
pub mod utils;

fn main() {
    run().unwrap();
}

fn run() -> Result<(), Error> {
    let sdl2 = sdl2::init()?;
    let timer = sdl2.timer()?;
    let video = sdl2.video()?;
    let window = video
        .window("The rpg", 1024, 768)
        .build()
        .map_err(|e| e.to_string())?;
    let _image = sdl2::image::init(sdl2::image::InitFlag::PNG)?;
    let ttf = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let data_path = Path::new("data");
    let mut config: GameConfig = data::load_file(data_path.join("config.json"))?;
    println!("{:?}", config);
    data::write_file(data_path.join("config.bin"), &config)?;
    // data::write_file(data_path.join("config.json"), &config)?;

    config.reroot(data_path);
    let pump = sdl2.event_pump()?;
    let mut canvas = window.into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let creator = canvas.texture_creator();
    let loader = TextureLoader::new(&creator);
    let resources = CachedResources::new(loader, &ttf);
    let mut state = GameState::new(resources);

    let mut listeners: Vec<Box<dyn EventListener<Window>>> = Vec::new();
    listeners.push(Box::new(QuitListener {}));

    let mut scene_stack: SceneStack<Window> = SceneStack {
        global_listeners: listeners,
        stack: Vec::new(),
    };
    let thebox = Box::new(MainMenu::new(config.font.load(&mut state.resources)?, config.map));
    scene_stack.stack.push(thebox);
    let mut frame_count = 0;
    let mut last_frames = [0u32; 500];
    let mut last_ticks = timer.ticks();
    let key_map = hardcoded_keymap();
    let mut pump_processor = PumpProcessor::new(pump, key_map);
    while state.running {
        let current_ticks = timer.ticks();
        state.ticks_to_process = current_ticks - last_ticks;
        if frame_count > 0 {
            last_frames[frame_count % last_frames.len()] = state.ticks_to_process;
        }
        last_ticks = current_ticks;

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        pump_processor.process_events(&mut state, &mut scene_stack);

        scene_stack.draw(&mut canvas, &mut state.resources)?;

        canvas.present();
        frame_count += 1;
        if frame_count % last_frames.len() == 0 {
            let sum: u32 = last_frames.iter().sum();
            let max = last_frames.iter().max();
            let fps = last_frames.len() as u32 * 1000 / sum;
            println!("Last {} frames took {} ms. Biggest frame: {} ms. Avg FPS: {}", last_frames.len(), sum, max.unwrap_or(&0), fps);
        }
    }

    Ok(())
}

struct SceneStack<'ttf, T: RenderTarget> {
    pub global_listeners: Vec<Box<dyn EventListener<'ttf, T>>>,
    pub stack: Vec<Box<dyn Scene<'ttf, T> + 'ttf>>,
}

impl<'ttf, T: RenderTarget> SceneStack<'ttf, T> {
    fn process(&mut self, result: Option<EventResult<'ttf, T>>) {
        match result {
            Some(EventResult::PushScene(scene)) => { self.stack.push(scene); }
            Some(EventResult::PopScene) => { self.stack.pop(); }
            _ => {}
        };
    }
}

impl<'ttf, T: RenderTarget> SceneStack<'ttf, T> {
    fn active_scene_mut(&mut self) -> &mut dyn Scene<'ttf, T> {
        self.stack.last_mut().unwrap().as_mut()
    }
}


impl<'ttf, T: RenderTarget> EventListener<'ttf, T> for SceneStack<'ttf, T> {
    fn batch_start(&mut self, state: &mut GameState<'ttf>, input: &InputState) -> Option<EventResult<'ttf, T>> {
        for listener in self.global_listeners.iter_mut() {
            listener.batch_start(state, input);
        }
        let result = self.active_scene_mut().batch_start(state, input);
        self.process(result);
        None
    }

    fn process_event(&mut self, state: &mut GameState<'ttf>, event: &Event) -> Option<EventResult<'ttf, T>> {
        for listener in self.global_listeners.iter_mut() {
            listener.process_event(state, event);
        }
        let result = self.active_scene_mut().process_event(state, event);
        self.process(result);
        None
    }

    fn batch_end(&mut self, state: &mut GameState<'ttf>, input: &InputState) -> Option<EventResult<'ttf, T>> {
        for listener in self.global_listeners.iter_mut() {
            listener.batch_end(state, input);
        }
        let result = self.active_scene_mut().batch_end(state, input);
        self.process(result);
        None
    }
}

impl<'ttf, T: RenderTarget> Scene<'ttf, T> for SceneStack<'ttf, T> {
    fn draw(&mut self, canvas: &mut Canvas<T>, resources: &mut dyn Resources<'ttf>) -> Result<(), Error> {
        self.active_scene_mut().draw(canvas, resources)
    }
}