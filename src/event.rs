use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::{KeyboardState, Keycode, Scancode};

pub struct PumpProcessor {
    pump: EventPump,
    listeners: Vec<Box<dyn EventListener>>,
}

impl PumpProcessor {
    pub fn new(pump: EventPump, listeners: Vec<Box<dyn EventListener>>) -> Self {
        PumpProcessor { pump, listeners }
    }

    pub fn process_batch(&mut self, state: &mut GameState) {
        let start_state = InputState::from(&self.pump);
        for listener in &self.listeners {
            listener.batch_start(state, &start_state);
        }

        for event in self.pump.poll_iter() {
            for listener in &self.listeners {
                listener.process_event(state, &event);
            }
        }

        let end_state = InputState::from(&self.pump);
        for listener in &self.listeners {
            listener.batch_end(state, &end_state);
        }
    }
}

pub enum EventResult {}

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
    pub x: f32,
    pub y: f32,
    pub ticks_to_process: u32,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            running: true,
            x: 0.,
            y: 0.,
            ticks_to_process: 0,
        }
    }
}

pub trait EventListener {
    fn batch_start(&self, _state: &mut GameState, _input: &InputState) -> Option<EventResult> { None }
    fn process_event(&self, _state: &mut GameState, _event: &Event) -> Option<EventResult> { None }
    fn batch_end(&self, _state: &mut GameState, _input: &InputState) -> Option<EventResult> { None }
}

pub struct QuitListener {}

impl EventListener for QuitListener {
    fn process_event(&self, state: &mut GameState, event: &Event) -> Option<EventResult> {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape | Keycode::Q),
                ..
            } => {
                state.running = false;
                None
            }
            _ => None
        }
    }
}

pub struct MoveListener {}

impl EventListener for MoveListener {
    fn batch_end(&self, state: &mut GameState, input: &InputState) -> Option<EventResult> {
        let distance: f32 = state.ticks_to_process as f32 * 0.3;
        if input.keyboard.is_scancode_pressed(Scancode::Up) {
            state.y -= distance;
        }
        if input.keyboard.is_scancode_pressed(Scancode::Down) {
            state.y += distance;
        }
        if input.keyboard.is_scancode_pressed(Scancode::Left) {
            state.x -= distance;
        }
        if input.keyboard.is_scancode_pressed(Scancode::Right) {
            state.x += distance;
        }
        None
    }
}