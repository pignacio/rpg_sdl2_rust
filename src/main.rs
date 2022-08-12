use std::path::Path;
use std::rc::Rc;

use sdl2::pixels::Color;
use sdl2::render::RenderTarget;
use sdl2::ttf::Font;
use sdl2::video::Window;

use gfx::texture::TextureLoader;

use crate::data::{Data, GameConfig};
use crate::data::map::MapData;
use crate::data::text::{TextBitData, TextLineData};
use crate::error::Error;
use crate::event::{Event, EventListener, EventResult, GameState, InputState, PumpProcessor, QuitListener};
use crate::gfx::animation::{Drawable, Ticker};
use crate::gfx::renderer::{BackBuffer, Renderer};
use crate::gfx::spritesheet::SpriteSheet;
use crate::gfx::texture::Texture;
use crate::keymap::hardcoded_keymap;
use crate::point::Point;
use crate::resources::{CachedResources, Resources};
use crate::scene::{main_menu::MainMenu, Scene};
use crate::text::{BitState, RenderedTextBit, RenderedTextLine};

pub mod data;
pub mod direction;
pub mod error;
pub mod event;
pub mod gfx;
pub mod keymap;
pub mod point;
pub mod resources;
pub mod scene;
pub mod text;
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
    let canvas = window.into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let creator = canvas.texture_creator();
    let loader = TextureLoader::new(&creator);
    let mut back_buffer = BackBuffer::new(canvas, &creator)?;
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

    let mut text_line = if let Some(line_data) = config.text_line { Some(load_text_line(line_data, &mut state.resources)?) } else { None };

    while state.running {
        let current_ticks = timer.ticks();
        state.ticks_to_process = current_ticks - last_ticks;
        if frame_count > 0 {
            last_frames[frame_count % last_frames.len()] = state.ticks_to_process;
        }
        last_ticks = current_ticks;


        pump_processor.process_events(&mut state, &mut scene_stack);
        text_line.as_mut().map(|line| {
            line.advance(state.ticks_to_process);
        });
        back_buffer.render_and_flip(false, |renderer| {
            renderer.set_draw_color(Color::BLACK);
            renderer.clear();
            scene_stack.draw(renderer, &mut state.resources)?;

            if let Some(line) = &text_line {
                line.draw_at(renderer, Point::new(50, 50))?;
            }
            Ok(())
        })?;

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

fn load_text_line<'sdl>(data: TextLineData, resources: &mut dyn Resources<'sdl>) -> Result<RenderedTextLine<'sdl>, Error> {
    let font = data.font.unwrap().load(resources)?;
    let bits: Result<Vec<RenderedTextBit>, Error> = data.parts.iter().map(|bit_data| {
        match bit_data {
            TextBitData::Text { text, .. } => Some(text),
            _ => None,
        }
    }).flatten()
        .map(|text| {
            let texture = render(&font, text, resources)?;
            // let width = texture.width();
            let states = build_states(&font, text, 100)?;
            println!("states for '{}': {:?}", text, states);
            Ok(RenderedTextBit::new(Rc::new(texture), states))
        })
        .collect();

    Ok(RenderedTextLine::new(bits?))
}

fn render<'sdl>(font: &Rc<Font>, text: &str, resources: & dyn Resources<'sdl>) -> Result<Texture<'sdl>, Error>{
    font.render(text).blended(Color::WHITE)
        .map_err(Error::from)
        .and_then(|s| resources.texture_from_surface(s))
}

fn build_states(font: &Rc<Font>, text: &str, letter_duration: u32) -> Result<Vec<BitState>, Error> {
    if text.is_empty() {
        Ok(vec![BitState { last_frame: letter_duration, position: 0 }])
    } else {
        (0..text.len()).map(|index| {
            let (width, _height) = font.size_of(&text[0..(index)])?;
            Ok(BitState { last_frame: letter_duration * (index + 1) as u32, position: width })
        })
            .collect()
    }
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
    fn draw(&mut self, renderer: &mut Renderer<T>, resources: &mut dyn Resources<'ttf>) -> Result<(), Error> {
        self.active_scene_mut().draw(renderer, resources)
    }
}
