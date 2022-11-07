macro_rules! init_keys {
    ($map:expr, $( $x:ident ),* ) => {
        {
            $(
                $map.insert(VirtualKeyCode::$x, false);
            )*
        }
    };
}
macro_rules! init_key_map {
    () => {
        {
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
                    Cut);
            keys
        }
    };
}

use std::{self, collections::HashMap};
use glium::glutin;
use winit::event::{VirtualKeyCode, KeyboardInput, ElementState};

pub struct Input {
    keys: HashMap<VirtualKeyCode, bool>,

    
}

impl Input {
    pub fn new() -> Self {
        return Self {
            keys: init_key_map!(),
        };
    }
    pub fn is_key_held(&self, key : VirtualKeyCode) -> bool {
        return self.keys[&key];
    }
    /// because you can't update data in hashmaps because reasons i have to do this mess
    pub fn update(&mut self, key : KeyboardInput) {
        *self.keys.get_mut(&key.virtual_keycode.unwrap()).unwrap() = key.state == ElementState::Pressed;
    }
}
