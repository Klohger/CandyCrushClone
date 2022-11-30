pub mod KeyState;
macro_rules! init_keys {
    ($map:expr, $( $x:ident ),* ) => {
        {
            $(
                $map.insert(VirtualKeyCode::$x, 0);
            )*
        }
    };
}
macro_rules! init_key_map {
    () => {{
        let mut keys = HashMap::new();
        init_keys!(
            keys,
            Key1,
            Key2,
            Key3,
            Key4,
            Key5,
            Key6,
            Key7,
            Key8,
            Key9,
            Key0,
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
            P,
            Q,
            R,
            S,
            T,
            U,
            V,
            W,
            X,
            Y,
            Z,
            Escape,
            F1,
            F2,
            F3,
            F4,
            F5,
            F6,
            F7,
            F8,
            F9,
            F10,
            F11,
            F12,
            F13,
            F14,
            F15,
            F16,
            F17,
            F18,
            F19,
            F20,
            F21,
            F22,
            F23,
            F24,
            Snapshot,
            Scroll,
            Pause,
            Insert,
            Home,
            Delete,
            End,
            PageDown,
            PageUp,
            Left,
            Up,
            Right,
            Down,
            Back,
            Return,
            Space,
            Compose,
            Caret,
            Numlock,
            Numpad0,
            Numpad1,
            Numpad2,
            Numpad3,
            Numpad4,
            Numpad5,
            Numpad6,
            Numpad7,
            Numpad8,
            Numpad9,
            NumpadAdd,
            NumpadDivide,
            NumpadDecimal,
            NumpadComma,
            NumpadEnter,
            NumpadEquals,
            NumpadMultiply,
            NumpadSubtract,
            AbntC1,
            AbntC2,
            Apostrophe,
            Apps,
            Asterisk,
            At,
            Ax,
            Backslash,
            Calculator,
            Capital,
            Colon,
            Comma,
            Convert,
            Equals,
            Grave,
            Kana,
            Kanji,
            LAlt,
            LBracket,
            LControl,
            LShift,
            LWin,
            Mail,
            MediaSelect,
            MediaStop,
            Minus,
            Mute,
            MyComputer,
            NavigateForward,
            NavigateBackward,
            NextTrack,
            NoConvert,
            OEM102,
            Period,
            PlayPause,
            Plus,
            Power,
            PrevTrack,
            RAlt,
            RBracket,
            RControl,
            RShift,
            RWin,
            Semicolon,
            Slash,
            Sleep,
            Stop,
            Sysrq,
            Tab,
            Underline,
            Unlabeled,
            VolumeDown,
            VolumeUp,
            Wake,
            WebBack,
            WebFavorites,
            WebForward,
            WebHome,
            WebRefresh,
            WebSearch,
            WebStop,
            Yen,
            Copy,
            Paste,
            Cut
        );
        keys
    }};
}

use std::{self, collections::HashMap};
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};

use self::KeyState::set_bit;

#[derive(Clone, PartialEq)]
pub struct Input {
    keys: HashMap<VirtualKeyCode, KeyState::KeyState>,
    pub mouse_delta: (f64, f64),
    /// scroll kinda sucks :)
    pub scroll_delta: (f32, f32),
}


impl Input {
    pub fn new() -> Self {
        return Self {
            keys: init_key_map!(),
            mouse_delta: (0.0, 0.0),
            scroll_delta: (0.0, 0.0),
        };
    }
    pub fn key_state(&self, key: VirtualKeyCode) -> KeyState::KeyState {
        return self.keys[&key];
    }

    pub fn poll_keys(&mut self, input: KeyboardInput) {
        if let Some(virtual_keycode) = &input.virtual_keycode {
            let key = self.keys.get_mut(virtual_keycode).unwrap();
            if KeyState::held(*key) != (input.state == ElementState::Pressed) {
                *key = set_bit(*key, 1, 1);
            }

            *key = set_bit(*key, 0, (input.state == ElementState::Pressed) as usize);
        }
    }

    pub fn poll_mouse(&mut self, input: (f64, f64)) {
        self.mouse_delta.0 += input.0;
        self.mouse_delta.1 += input.1;
    }
    pub fn poll_scroll(&mut self, input: (f32, f32)) {
        self.scroll_delta = input;
    }

    pub fn update_has_ran(&mut self) {
        for key in self.keys.iter_mut() {
            *key.1 = KeyState::clear_bit(*key.1, 1);
        }
        self.mouse_delta = (0.0, 0.0);
    }
}

impl Default for Input {
    fn default() -> Self {
        return Input::new();
    }
}