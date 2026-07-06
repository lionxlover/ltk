//! wl_seat: keyboard, pointer, touch capability negotiation.

#[derive(Debug, Clone, Copy, Default)]
pub struct SeatCapabilities { pub pointer: bool, pub keyboard: bool, pub touch: bool }

pub struct WaylandSeat { pub name: String, pub capabilities: SeatCapabilities }

impl WaylandSeat {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), capabilities: SeatCapabilities::default() }
    }
}
