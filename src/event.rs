use sdl2::event::Event as SdlEvent;
use sdl2::EventPump;
use sdl2::keyboard::{KeyboardState, Keycode};
use sdl2::render::RenderTarget;
use sdl2::video::WindowContext;

use crate::{CachedResources, Scene};
use crate::keymap::{Action, KeyMap};

pub struct PumpProcessor {
    pump: EventPump,
    key_map: KeyMap,
}

#[derive(Debug, Clone)]
pub enum Event {
    Sdl(SdlEvent),
    ActionDown { action: Action },
    ActionUp { action: Action },
}

impl PumpProcessor {
    pub fn new(pump: EventPump, key_map: KeyMap) -> Self {
        PumpProcessor { pump, key_map }
    }

    pub fn process_events<'ttf, T: RenderTarget, L: EventListener<'ttf, T>>(&mut self, state: &mut GameState<'ttf>, listener: &mut L) {
        let start_state = self.get_input_state();
        listener.batch_start(state, &start_state);

        for sdl_event in self.pump.poll_iter() {
            let event = match sdl_event {
                SdlEvent::KeyDown { scancode: Some(scancode), .. } => self.key_map.get_action(&scancode).map(|action| Event::ActionDown { action: *action }),
                SdlEvent::KeyUp { scancode: Some(scancode), .. } => self.key_map.get_action(&scancode).map(|action| Event::ActionUp { action: *action }),
                _ => Some(Event::Sdl(sdl_event)),
            };
            event.and_then(|e| listener.process_event(state, &e));
        }

        let end_state = self.get_input_state();
        listener.batch_end(state, &end_state);
    }

    fn get_input_state(&self) -> InputState {
        InputState::new(SdlInputState::from(&self.pump), &self.key_map)
    }
}

pub enum EventResult<'ttf, T: RenderTarget> {
    PushScene(Box<dyn Scene<'ttf, T> + 'ttf>),
    PopScene,
}

pub struct SdlInputState<'r> {
    pub keyboard: KeyboardState<'r>,
}

impl<'r> SdlInputState<'r> {
    fn from(pump: &'r EventPump) -> SdlInputState<'r> {
        SdlInputState {
            keyboard: pump.keyboard_state()
        }
    }
}

pub struct InputState<'r> {
    sdl_state: SdlInputState<'r>,
    key_map: &'r KeyMap,
}

impl<'r> InputState<'r> {
    pub fn new(sdl_state: SdlInputState<'r>, key_map: &'r KeyMap) -> Self {
        InputState { sdl_state, key_map }
    }

    pub fn is_action_pressed(&self, action: Action) -> bool {
        self.key_map.get_keys(&action)
            .map(|keys| keys.iter().any(|key| self.sdl_state.keyboard.is_scancode_pressed(*key)))
            .unwrap_or(false)
    }
}


pub struct GameState<'tx> {
    pub running: bool,
    pub ticks_to_process: u32,
    pub resources: CachedResources<'tx, WindowContext>,
}

impl<'tx> GameState<'tx> {
    pub fn new(resources: CachedResources<'tx, WindowContext>) -> Self {
        GameState {
            running: true,
            ticks_to_process: 0,
            resources,
        }
    }
}

pub trait EventListener<'ttf, T: RenderTarget> {
    fn batch_start(&mut self, _state: &mut GameState<'ttf>, _input: &InputState) -> Option<EventResult<'ttf, T>> { None }
    fn process_event(&mut self, _state: &mut GameState<'ttf>, _event: &Event) -> Option<EventResult<'ttf, T>> { None }
    fn batch_end(&mut self, _state: &mut GameState<'ttf>, _input: &InputState) -> Option<EventResult<'ttf, T>> { None }
}

pub struct QuitListener {}

impl<'ttf, T: RenderTarget> EventListener<'ttf, T> for QuitListener {
    fn process_event(&mut self, state: &mut GameState<'ttf>, event: &Event) -> Option<EventResult<'ttf, T>> {
        match event {
            Event::Sdl(SdlEvent::Quit { .. }
                       | SdlEvent::KeyDown {
                keycode: Some(Keycode::Q),
                ..
            }) => {
                state.running = false;
                None
            }
            _ => None
        }
    }
}