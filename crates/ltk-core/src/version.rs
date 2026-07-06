//! SemVer type and compatibility checks.

use std::fmt;

/// A semantic version number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version { pub major: u32, pub minor: u32, pub patch: u32 }

impl Version {
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self { Self { major, minor, patch } }

    /// Returns true if `self` is backward-compatible with `required`.
    /// Compatible means: same major, self.minor >= required.minor.
    pub fn is_compatible_with(self, required: Self) -> bool {
        self.major == required.major && self >= required
    }

    pub fn parse(s: &str) -> Option<Self> {
        let mut p = s.split('.');
        let major = p.next()?.parse().ok()?;
        let minor = p.next()?.parse().ok()?;
        let patch = p.next()?.parse().ok()?;
        Some(Self::new(major, minor, patch))
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}
