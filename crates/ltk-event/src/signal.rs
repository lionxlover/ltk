//! Fine-grained typed signal/slot connections (one-to-many).

use ltk_core::sync::RwLock;
use std::sync::Arc;

type SlotFn<A> = Box<dyn Fn(&A) + Send + Sync>;

/// A unique handle for a slot connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SlotHandle(u64);

static NEXT_SLOT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
fn next_slot() -> SlotHandle { SlotHandle(NEXT_SLOT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)) }

struct Connection<A> { handle: SlotHandle, slot: SlotFn<A> }

/// A typed signal that dispatches to all connected slots when emitted.
pub struct Signal<A = ()> {
    connections: Arc<RwLock<Vec<Connection<A>>>>,
}

impl<A: 'static> Signal<A> {
    pub fn new() -> Self { Self { connections: Arc::new(RwLock::new(Vec::new())) } }

    /// Connect a slot. Returns a handle; keep it alive to stay connected.
    pub fn connect(&self, slot: impl Fn(&A) + Send + Sync + 'static) -> SlotHandle {
        let handle = next_slot();
        self.connections.write().push(Connection { handle, slot: Box::new(slot) });
        handle
    }

    /// Disconnect a slot by handle.
    pub fn disconnect(&self, handle: SlotHandle) {
        self.connections.write().retain(|c| c.handle != handle);
    }

    /// Emit the signal — calls all connected slots synchronously.
    pub fn emit(&self, arg: &A) {
        let conns = self.connections.read();
        for c in conns.iter() { (c.slot)(arg); }
    }

    pub fn connection_count(&self) -> usize { self.connections.read().len() }
}

impl<A> Default for Signal<A> { fn default() -> Self { Self::new() } }
impl<A> Clone for Signal<A> { fn clone(&self) -> Self { Self { connections: Arc::clone(&self.connections) } } }
