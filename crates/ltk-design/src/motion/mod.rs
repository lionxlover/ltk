//! Motion tokens: duration scale and named easing curves.

use std::fmt;
use serde::{Deserialize, Serialize};

/// Named duration tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DurationToken {
    Instant,  // 0ms   — no animation
    UltraFast,// 50ms  — micro-interactions (checkbox, radio)
    Fast,     // 150ms — hover states, button press
    Base,     // 250ms — standard transitions
    Moderate, // 350ms — larger elements, menus
    Slow,     // 500ms — page transitions, reveals
    XSlow,    // 700ms — complex orchestrated sequences
    Spring,   // physics-based (no fixed duration)
}

impl DurationToken {
    /// Milliseconds (None = physics-based spring).
    pub fn ms(self) -> Option<f32> {
        use DurationToken::*;
        match self {
            Instant   => Some(0.0),
            UltraFast => Some(50.0),
            Fast      => Some(150.0),
            Base      => Some(250.0),
            Moderate  => Some(350.0),
            Slow      => Some(500.0),
            XSlow     => Some(700.0),
            Spring    => None,
        }
    }
}

/// An easing curve.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Easing {
    /// Constant velocity.
    Linear,
    /// CSS ease (slight ease-in, moderate ease-out).
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    /// Cubic Bézier with four control points.
    CubicBezier(f32, f32, f32, f32),
    /// Spring physics (mass, stiffness, damping, initial_velocity).
    Spring { mass: f32, stiffness: f32, damping: f32, initial_velocity: f32 },
    /// Step function (jump-start, jump-end, jump-both, jump-none).
    Steps(u32, StepPosition),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StepPosition { Start, End, Both, None }

impl Easing {
    /// Standard spring: slightly overshoots then settles.
    pub fn spring_standard() -> Self {
        Self::Spring { mass: 1.0, stiffness: 200.0, damping: 20.0, initial_velocity: 0.0 }
    }
    /// Soft spring: no overshoot, smooth deceleration.
    pub fn spring_soft() -> Self {
        Self::Spring { mass: 1.0, stiffness: 180.0, damping: 26.0, initial_velocity: 0.0 }
    }
    /// Bouncy spring: exaggerated overshoot.
    pub fn spring_bouncy() -> Self {
        Self::Spring { mass: 1.0, stiffness: 280.0, damping: 14.0, initial_velocity: 0.0 }
    }

    pub const SNAP:        Self = Self::CubicBezier(0.4, 0.0, 0.2, 1.0);
    pub const DECELERATE:  Self = Self::CubicBezier(0.0, 0.0, 0.2, 1.0);
    pub const ACCELERATE:  Self = Self::CubicBezier(0.4, 0.0, 1.0, 1.0);
    pub const SHARP:       Self = Self::CubicBezier(0.4, 0.0, 0.6, 1.0);

    /// Evaluate easing at t ∈ [0,1]. (Spring requires simulation; returns 0 here.)
    pub fn evaluate(&self, t: f32) -> f32 {
        match self {
            Self::Linear    => t,
            Self::Ease      => Self::cubic(t, 0.25, 0.1, 0.25, 1.0),
            Self::EaseIn    => Self::cubic(t, 0.42, 0.0, 1.0,  1.0),
            Self::EaseOut   => Self::cubic(t, 0.0,  0.0, 0.58, 1.0),
            Self::EaseInOut => Self::cubic(t, 0.42, 0.0, 0.58, 1.0),
            Self::CubicBezier(x1, y1, x2, y2) => Self::cubic(t, *x1, *y1, *x2, *y2),
            Self::Steps(n, pos) => {
                let step = (t * *n as f32).floor() as u32;
                match pos {
                    StepPosition::Start => ((step + 1).min(*n)) as f32 / *n as f32,
                    _                   => step as f32 / *n as f32,
                }
            }
            Self::Spring { .. } => t, // placeholder; real impl uses RK4 integrator
        }
    }

    fn cubic(t: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
        // Newton-Raphson approximation of cubic Bézier Y(X)
        let cx = 3.0 * x1;
        let bx = 3.0 * (x2 - x1) - cx;
        let ax = 1.0 - cx - bx;
        let cy = 3.0 * y1;
        let by = 3.0 * (y2 - y1) - cy;
        let ay = 1.0 - cy - by;
        let sample_curve_y = |t: f32| ((ay * t + by) * t + cy) * t;
        let sample_curve_x = |t: f32| ((ax * t + bx) * t + cx) * t;
        let sample_curve_dx= |t: f32| (3.0 * ax * t + 2.0 * bx) * t + cx;
        let mut t2 = t;
        for _ in 0..8 {
            let x2 = sample_curve_x(t2) - t;
            if x2.abs() < 1e-6 { break; }
            let dx = sample_curve_dx(t2);
            if dx.abs() < 1e-8 { break; }
            t2 -= x2 / dx;
        }
        sample_curve_y(t2.clamp(0.0, 1.0))
    }
}

/// Bundle of all motion tokens.
pub struct MotionTokens {
    pub reduce_motion: bool,
}

impl MotionTokens {
    pub fn duration(&self, token: DurationToken) -> Option<f32> {
        if self.reduce_motion {
            match token {
                DurationToken::Instant | DurationToken::UltraFast | DurationToken::Fast => Some(0.0),
                _ => Some(0.0),
            }
        } else {
            token.ms()
        }
    }

    pub fn easing(&self, name: &str) -> Easing {
        if self.reduce_motion { return Easing::Linear; }
        match name {
            "spring"      => Easing::spring_standard(),
            "spring-soft" => Easing::spring_soft(),
            "snap"        => Easing::SNAP,
            "decelerate"  => Easing::DECELERATE,
            "accelerate"  => Easing::ACCELERATE,
            "ease-in-out" => Easing::EaseInOut,
            _             => Easing::Ease,
        }
    }
}
