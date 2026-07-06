//! Damped spring physics using 4th-order Runge-Kutta integration.

/// Spring configuration (physical parameters).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpringConfig {
    pub mass:      f32,   // kg (default 1.0)
    pub stiffness: f32,   // N/m (default 200.0)
    pub damping:   f32,   // Ns/m (default 20.0)
}

impl SpringConfig {
    pub const STANDARD: Self = Self { mass: 1.0, stiffness: 200.0, damping: 20.0 };
    pub const SOFT:     Self = Self { mass: 1.0, stiffness: 180.0, damping: 26.0 };
    pub const BOUNCY:   Self = Self { mass: 1.0, stiffness: 280.0, damping: 14.0 };
    pub const STIFF:    Self = Self { mass: 1.0, stiffness: 400.0, damping: 40.0 };

    /// Returns true if the spring is critically or over-damped (no oscillation).
    pub fn is_overdamped(&self) -> bool {
        self.damping * self.damping >= 4.0 * self.mass * self.stiffness
    }

    /// Estimated duration until the spring settles (within 0.1% of target).
    pub fn estimated_duration_ms(&self) -> f32 {
        let omega_n = (self.stiffness / self.mass).sqrt();
        let zeta    = self.damping / (2.0 * (self.mass * self.stiffness).sqrt());
        if zeta >= 1.0 {
            1000.0 * 4.0 / (zeta * omega_n) // overdamped: 4τ
        } else {
            let omega_d = omega_n * (1.0 - zeta * zeta).sqrt();
            1000.0 * (-0.001f32.ln()) / (zeta * omega_n) // underdamped
        }
    }
}

impl Default for SpringConfig { fn default() -> Self { Self::STANDARD } }

/// Running spring simulation state.
#[derive(Debug, Clone, Copy)]
pub struct SpringSim {
    pub config:   SpringConfig,
    pub position: f32,  // current value
    pub velocity: f32,  // current velocity
    pub target:   f32,  // rest position
}

impl SpringSim {
    pub fn new(cfg: SpringConfig, initial: f32, target: f32) -> Self {
        Self { config: cfg, position: initial, velocity: 0.0, target }
    }

    /// Advance the simulation by `dt` seconds using RK4 integration.
    pub fn step(&mut self, dt: f32) {
        let k = self.config.stiffness;
        let d = self.config.damping;
        let m = self.config.mass;
        let target = self.target;

        // RK4: state = (position, velocity)
        let accel = |p: f32, v: f32| -> f32 {
            (-k * (p - target) - d * v) / m
        };

        let (p0, v0) = (self.position, self.velocity);
        let a0 = accel(p0, v0);

        let (p1, v1) = (p0 + v0 * dt * 0.5, v0 + a0 * dt * 0.5);
        let a1 = accel(p1, v1);

        let (p2, v2) = (p0 + v1 * dt * 0.5, v0 + a1 * dt * 0.5);
        let a2 = accel(p2, v2);

        let (p3, v3) = (p0 + v2 * dt, v0 + a2 * dt);
        let a3 = accel(p3, v3);

        self.position += (dt / 6.0) * (v0 + 2.0*v1 + 2.0*v2 + v3);
        self.velocity += (dt / 6.0) * (a0 + 2.0*a1 + 2.0*a2 + a3);
    }

    /// Returns true when the spring has effectively settled.
    pub fn is_at_rest(&self, threshold: f32) -> bool {
        (self.position - self.target).abs() < threshold
        && self.velocity.abs() < threshold
    }
}

/// A handle returned when a spring animation is started.
#[derive(Debug, Clone)]
pub struct SpringHandle {
    pub id: ltk_core::id::AnimationId,
}
