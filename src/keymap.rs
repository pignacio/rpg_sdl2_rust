use std::collections::HashMap;

use sdl2::keyboard::{Keycode, Scancode};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Action {
    Select,
    Back,
    Up,
    Down,
    Left,
    Right,
}

pub struct KeyMap {
    key_to_action: HashMap<Scancode, Action>,
    action_to_keys: HashMap<Action, Vec<Scancode>>,
}

fn invert(key_to_action: &HashMap<Scancode, Action>) -> HashMap<Action, Vec<Scancode>> {
    let mut action_to_keys: HashMap<Action, Vec<Scancode>> = HashMap::new();
    for (key, action) in key_to_action {
        action_to_keys.entry(*action).or_insert_with(|| Vec::new()).push(*key);
    }
    action_to_keys
}

impl KeyMap {
    pub fn new(key_to_action: HashMap<Scancode, Action>) -> Self {
        let action_to_keys = invert(&key_to_action);
        KeyMap {
            key_to_action,
            action_to_keys,
        }
    }

    pub fn get_action(&self, key: &Scancode) -> Option<&Action> {
        self.key_to_action.get(key)
    }

    pub fn get_keys(&self, action: &Action) -> Option<&Vec<Scancode>> {
        self.action_to_keys.get(action)
    }
}

pub fn hardcoded_keymap() -> KeyMap {
    let keycode_map = [
        (Keycode::Up, Action::Up),
        (Keycode::Down, Action::Down),
        (Keycode::Left, Action::Left),
        (Keycode::Right, Action::Right),
        (Keycode::W, Action::Up),
        (Keycode::S, Action::Down),
        (Keycode::A, Action::Left),
        (Keycode::D, Action::Right),
        (Keycode::Return, Action::Select),
        (Keycode::Escape, Action::Back),
    ];

    KeyMap::new(keycode_map.iter()
        .map(|(key, action)| (Scancode::from_keycode(*key).unwrap(), *action))
        .collect())
}

// let KEY_MAP: HashMap<Keycode, Action> = HashMap::from([
//     (Keycode::Up, Action::Up),
//     (Keycode::Down, Action::Down),
//     (Keycode::Left, Action::Left),
//     (Keycode::Right, Action::Right),
//     (Keycode::Return, Action::Select),
//     (Keycode::Escape, Action::Back),
// ]);