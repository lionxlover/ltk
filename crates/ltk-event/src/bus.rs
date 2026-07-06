//! Typed publish/subscribe event bus.

use ltk_core::{callback::Callback, id::SubscriptionId, sync::RwLock};
use std::{any::{Any, TypeId}, collections::HashMap, sync::Arc};

type BoxedCallback = Box<dyn Fn(&dyn Any) + Send + Sync>;

struct Slot { id: SubscriptionId, cb: BoxedCallback }

/// Central typed event bus.  Publish any `'static` event; subscribers receive `&E`.
pub struct EventBus {
    channels: RwLock<HashMap<TypeId, Vec<Slot>>>,
}

impl EventBus {
    pub fn new() -> Arc<Self> {
        Arc::new(Self { channels: RwLock::new(HashMap::new()) })
    }

    /// Subscribe to events of type `E`. Returns an ID; call `unsubscribe` to remove.
    pub fn subscribe<E: 'static>(
        &self,
        cb: impl Fn(&E) + Send + Sync + 'static,
    ) -> SubscriptionId {
        let id = SubscriptionId::new();
        let type_id = TypeId::of::<E>();
        let slot = Slot {
            id,
            cb: Box::new(move |any| {
                if let Some(e) = any.downcast_ref::<E>() { cb(e); }
            }),
        };
        self.channels.write().entry(type_id).or_default().push(slot);
        id
    }

    /// Publish an event to all subscribers of its type.
    pub fn publish<E: 'static>(&self, event: E) {
        let type_id = TypeId::of::<E>();
        if let Some(slots) = self.channels.read().get(&type_id) {
            for slot in slots { (slot.cb)(&event); }
        }
    }

    /// Unsubscribe by ID.
    pub fn unsubscribe(&self, id: SubscriptionId) {
        for slots in self.channels.write().values_mut() {
            slots.retain(|s| s.id != id);
        }
    }
}

impl Default for EventBus { fn default() -> Self { *Arc::try_unwrap(Self::new()).ok().unwrap() } }
