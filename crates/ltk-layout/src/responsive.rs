//! Responsive layout: breakpoints and adaptive rules.
use ltk_core::geometry::Size;

/// A named width breakpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Breakpoint { Xs, Sm, Md, Lg, Xl, Xl2 }

impl Breakpoint {
    pub fn min_width(self) -> f32 {
        match self {
            Self::Xs  => 0.0,
            Self::Sm  => 640.0,
            Self::Md  => 768.0,
            Self::Lg  => 1024.0,
            Self::Xl  => 1280.0,
            Self::Xl2 => 1536.0,
        }
    }
    pub fn for_width(w: f32) -> Self {
        if      w >= 1536.0 { Self::Xl2 }
        else if w >= 1280.0 { Self::Xl }
        else if w >= 1024.0 { Self::Lg }
        else if w >= 768.0  { Self::Md }
        else if w >= 640.0  { Self::Sm }
        else                { Self::Xs }
    }
}

/// A set of breakpoints with associated values of type `T`.
pub struct BreakpointSet<T> {
    pub xs:  T,
    pub sm:  Option<T>,
    pub md:  Option<T>,
    pub lg:  Option<T>,
    pub xl:  Option<T>,
    pub xl2: Option<T>,
}

impl<T: Clone> BreakpointSet<T> {
    pub fn all(value: T) -> Self {
        Self { xs: value, sm: None, md: None, lg: None, xl: None, xl2: None }
    }
    pub fn resolve(&self, bp: Breakpoint) -> &T {
        let opts = [
            (Breakpoint::Xl2, &self.xl2),
            (Breakpoint::Xl,  &self.xl),
            (Breakpoint::Lg,  &self.lg),
            (Breakpoint::Md,  &self.md),
            (Breakpoint::Sm,  &self.sm),
        ];
        for (threshold, val) in &opts {
            if bp >= *threshold { if let Some(v) = val { return v; } }
        }
        &self.xs
    }
}

/// A responsive rule: apply a style/layout change at a breakpoint.
#[derive(Debug, Clone)]
pub struct ResponsiveRule {
    pub breakpoint: Breakpoint,
    pub key:        String,
    pub value:      String,
}
