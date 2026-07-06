//! Gamepad input (primarily for accessibility — switch access, etc.).

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadButton {
    A, B, X, Y,
    LeftBumper, RightBumper,
    LeftTrigger, RightTrigger,
    Select, Start, Guide,
    LeftStick, RightStick,
    DPadUp, DPadDown, DPadLeft, DPadRight,
}

#[derive(Debug, Clone)]
pub struct GamepadEvent {
    pub device_id:  u32,
    pub button:     Option<GamepadButton>,
    pub axis:       Option<(GamepadAxis, f32)>,
    pub timestamp:  u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamepadAxis { LeftX, LeftY, RightX, RightY, LeftTrigger, RightTrigger }
