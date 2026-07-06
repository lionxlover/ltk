//! Keyboard event types with cross-platform key codes.

use bitflags::bitflags;

/// Physical key code (layout-independent).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum KeyCode {
    // Alphanumeric
    A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z,
    Digit0,Digit1,Digit2,Digit3,Digit4,Digit5,Digit6,Digit7,Digit8,Digit9,
    // Function keys
    F1,F2,F3,F4,F5,F6,F7,F8,F9,F10,F11,F12,
    // Control
    Enter,Escape,Space,Tab,Backspace,Delete,Insert,
    Home,End,PageUp,PageDown,
    ArrowUp,ArrowDown,ArrowLeft,ArrowRight,
    // Modifiers
    ShiftLeft,ShiftRight,CtrlLeft,CtrlRight,AltLeft,AltRight,
    SuperLeft,SuperRight,CapsLock,NumLock,ScrollLock,
    // Punctuation
    Minus,Equal,BracketLeft,BracketRight,Backslash,Semicolon,Quote,Comma,Period,Slash,Grave,
    // Numpad
    Numpad0,Numpad1,Numpad2,Numpad3,Numpad4,
    Numpad5,Numpad6,Numpad7,Numpad8,Numpad9,
    NumpadAdd,NumpadSub,NumpadMul,NumpadDiv,NumpadDecimal,NumpadEnter,
    // Media
    MediaPlay,MediaStop,MediaNext,MediaPrev,VolumeUp,VolumeDown,VolumeMute,
    // Other
    PrintScreen,Pause,Menu,Unknown(u32),
}

bitflags! {
    /// Modifier keys held during a key event.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct Modifiers: u16 {
        const SHIFT   = 0x0001;
        const CTRL    = 0x0002;
        const ALT     = 0x0004;
        const SUPER   = 0x0008;
        const CAPS    = 0x0010;
        const NUM     = 0x0020;
        const ALTGR   = 0x0040;
    }
}

impl Modifiers {
    pub fn is_plain(&self) -> bool { self.is_empty() }
    pub fn ctrl_or_cmd(&self) -> bool { self.contains(Self::CTRL) || self.contains(Self::SUPER) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyState { Pressed, Released, Repeat }

/// A single keyboard event.
#[derive(Debug, Clone)]
pub struct KeyEvent {
    pub code:      KeyCode,
    pub state:     KeyState,
    pub modifiers: Modifiers,
    /// Logical text produced by this keypress (e.g. "A", "€", "\n").
    pub text:      Option<String>,
    pub repeat:    bool,
    pub timestamp: u64,
}

impl KeyEvent {
    pub fn is_press(&self)   -> bool { self.state == KeyState::Pressed  }
    pub fn is_release(&self) -> bool { self.state == KeyState::Released }
    pub fn has_text(&self)   -> bool { self.text.as_ref().map_or(false, |t| !t.is_empty()) }
}
