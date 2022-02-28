use sdl2::{pixels::Color, rect::Rect};

use gfx::TextureLoader;
use crate::data::GameConfig;
use crate::error::Error;
use crate::event::{EventListener, GameState, MoveListener, PumpProcessor, QuitListener};

pub mod data;
pub mod error;
pub mod event;
pub mod gfx;

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
    let _image = sdl2::image::init(sdl2::image::InitFlag::PNG);

    let config : GameConfig = data::load_file(&"config.json")?;
    println!("{:?}", config);
    data::write_file(&"config.bin", &config)?;
    let pump = sdl2.event_pump()?;
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let loader = TextureLoader::new(canvas.texture_creator());

    let character = loader.load(&config.character)?;
    let mut state = GameState::new();

    let mut listeners: Vec<Box<dyn EventListener>> = Vec::new();
    listeners.push(Box::new(QuitListener {}));
    listeners.push(Box::new(MoveListener {}));

    let mut pump_processor = PumpProcessor::new(pump, listeners);

    let mut last_ticks = timer.ticks();

    while state.running {
        let current_ticks = timer.ticks();
        state.ticks_to_process = current_ticks - last_ticks;
        last_ticks = current_ticks;

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        pump_processor.process_batch(&mut state);


        let src = Rect::new(0, 0, 16, 32);
        let dst = Rect::new(state.x as i32, state.y as i32, 32, 64);
        canvas.copy(character.texture(), Some(src), Some(dst))?;

        canvas.present();
    }

    Ok(())
}