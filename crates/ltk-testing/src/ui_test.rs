//! Simulate user interactions (clicks, keys, gestures) in tests.

use ltk_core::{geometry::Point, id::WidgetId};
use ltk_input::{keyboard::{KeyEvent, KeyCode, KeyState, Modifiers}, mouse::{MouseEvent, MouseEventKind, MouseButton}};

/// Drives simulated input events against a widget tree for testing.
pub struct UiTestSimulator { pub events: Vec<String> }   // log of dispatched events

impl UiTestSimulator {
    pub fn new() -> Self { Self { events: Vec::new() } }

    pub fn click(&mut self, target: WidgetId, at: Point) -> MouseEvent {
        self.events.push(format!("click({target:?}, {at:?})"));
        MouseEvent {
            kind: MouseEventKind::Clicked,
            position: at,
            button: Some(MouseButton::Left),
            delta: None,
            modifiers: Modifiers::empty(),
            timestamp: 0,
        }
    }

    pub fn type_text(&mut self, text: &str) -> Vec<KeyEvent> {
        text.chars().map(|c| {
            self.events.push(format!("key('{c}')"));
            KeyEvent {
                code: KeyCode::Unknown(c as u32),
                state: KeyState::Pressed,
                modifiers: Modifiers::empty(),
                text: Some(c.to_string()),
                repeat: false,
                timestamp: 0,
            }
        }).collect()
    }

    pub fn press_key(&mut self, code: KeyCode, mods: Modifiers) -> KeyEvent {
        self.events.push(format!("press({code:?}, {mods:?})"));
        KeyEvent { code, state: KeyState::Pressed, modifiers: mods, text: None, repeat: false, timestamp: 0 }
    }
}

impl Default for UiTestSimulator { fn default() -> Self { Self::new() } }
