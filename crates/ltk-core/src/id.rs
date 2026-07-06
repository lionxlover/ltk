//! Typed, globally unique identifiers for every framework entity.

use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);
fn next() -> u64 { NEXT_ID.fetch_add(1, Ordering::Relaxed) }

macro_rules! define_id {
    ($(#[$m:meta])* $name:ident) => {
        $(#[$m])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(pub(crate) u64);

        impl $name {
            /// Allocate a new globally unique ID.
            pub fn new() -> Self { Self(next()) }
            /// The numeric value of this ID.
            pub fn value(self) -> u64 { self.0 }
            /// A sentinel "null" ID that is never allocated.
            pub const NULL: Self = Self(0);
            /// Returns true if this is the null sentinel.
            pub fn is_null(self) -> bool { self.0 == 0 }
        }

        impl Default for $name {
            fn default() -> Self { Self::new() }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}({})", stringify!($name), self.0)
            }
        }
    };
}

define_id!(/// Unique identity of a widget/component node.
           WidgetId);
define_id!(/// Unique identity of a render layer.
           LayerId);
define_id!(/// Unique identity of an OS window.
           WindowId);
define_id!(/// Unique identity of a physical display/monitor.
           MonitorId);
define_id!(/// Unique identity of a running animation.
           AnimationId);
define_id!(/// Unique identity of a layout node.
           LayoutNodeId);
define_id!(/// Unique identity of a scene graph node.
           SceneNodeId);
define_id!(/// Unique identity of an event bus subscription.
           SubscriptionId);
define_id!(/// Unique identity of a resource handle.
           ResourceId);
define_id!(/// Unique identity of a plugin instance.
           PluginId);
define_id!(/// Unique identity of a loaded font face.
           FontId);
define_id!(/// Unique identity of a GPU texture.
           TextureId);
define_id!(/// Unique identity of a render surface.
           SurfaceId);
define_id!(/// Unique identity of a state binding.
           BindingId);
define_id!(/// Unique identity of a command in the undo stack.
           CommandId);
define_id!(/// Unique identity of a service registration.
           ServiceId);
