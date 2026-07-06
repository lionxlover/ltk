//! Named service discovery + dependency injection container.

use std::{any::{Any, TypeId}, collections::HashMap, sync::Arc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ServiceId(u64);
static NEXT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
impl ServiceId { pub fn new() -> Self { Self(NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)) } }

/// A simple DI container keyed by type.
pub struct ServiceRegistry {
    services: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl ServiceRegistry {
    pub fn new() -> Self { Self { services: HashMap::new() } }

    pub fn register<T: Send + Sync + 'static>(&mut self, service: T) {
        self.services.insert(TypeId::of::<T>(), Arc::new(service));
    }

    pub fn resolve<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        self.services.get(&TypeId::of::<T>())
            .and_then(|s| s.clone().downcast::<T>().ok())
    }

    pub fn is_registered<T: 'static>(&self) -> bool {
        self.services.contains_key(&TypeId::of::<T>())
    }
}

impl Default for ServiceRegistry { fn default() -> Self { Self::new() } }
