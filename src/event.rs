use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::{KeyboardState, Keycode};
use sdl2::render::RenderTarget;

use crate::Scene;

pub struct PumpProcessor {
    pump: EventPump,
}

impl PumpProcessor {
    pub fn new(pump: EventPump) -> Self {
        PumpProcessor { pump }
    }

    pub fn process_events<'ttf, T: RenderTarget, L: EventListener<'ttf, T>>(&mut self, state: &mut GameState, listener: &mut L) {
        let start_state = InputState::from(&self.pump);
        listener.batch_start(state, &start_state);

        for event in self.pump.poll_iter() {
            listener.process_event(state, &event);
        }

        let end_state = InputState::from(&self.pump);
        listener.batch_end(state, &end_state);
    }
}

pub enum EventResult<'ttf, T: RenderTarget> {
    PushScene(Box<dyn Scene<'ttf, T> + 'ttf>),
    PopScene,
}

pub struct InputState<'r> {
    pub keyboard: KeyboardState<'r>,
}

impl<'r> InputState<'r> {
    fn from(pump: &'r EventPump) -> InputState<'r> {
        InputState {
            keyboard: pump.keyboard_state()
        }
    }
}

pub struct GameState {
    pub running: bool,
    pub ticks_to_process: u32,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            running: true,
            ticks_to_process: 0,
        }
    }
}

pub trait EventListener<'ttf, T: RenderTarget> {
    fn batch_start(&mut self, _state: &mut GameState, _input: &InputState) -> Option<EventResult<'ttf, T>> { None }
    fn process_event(&mut self, _state: &mut GameState, _event: &Event) -> Option<EventResult<'ttf, T>> { None }
    fn batch_end(&mut self, _state: &mut GameState, _input: &InputState) -> Option<EventResult<'ttf, T>> { None }
}

pub struct QuitListener {}

impl<'ttf, T: RenderTarget> EventListener<'ttf, T> for QuitListener {
    fn process_event(&mut self, state: &mut GameState, event: &Event) -> Option<EventResult<'ttf, T>> {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Q),
                ..
            } => {
                state.running = false;
                None
            }
            _ => None
        }
    }
}