//! SharedString: a cheaply-cloneable, immutable string.

use std::{fmt, ops::Deref, sync::Arc};

/// An immutable string backed by an `Arc<str>`.
/// Cloning is O(1).
#[derive(Clone, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct SharedString(Arc<str>);

impl SharedString {
    pub fn new(s: impl AsRef<str>) -> Self { Self(s.as_ref().into()) }
    pub fn as_str(&self) -> &str { &self.0 }
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
    pub fn len(&self) -> usize { self.0.len() }
}

impl From<&str>    for SharedString { fn from(s: &str) -> Self { Self::new(s) } }
impl From<String>  for SharedString { fn from(s: String) -> Self { Self(s.into()) } }
impl From<Arc<str>>for SharedString { fn from(s: Arc<str>) -> Self { Self(s) } }
impl Deref         for SharedString { type Target = str; fn deref(&self) -> &str { &self.0 } }
impl fmt::Display  for SharedString { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.0.fmt(f) } }
impl fmt::Debug    for SharedString { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.0.fmt(f) } }
