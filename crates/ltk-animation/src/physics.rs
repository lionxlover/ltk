//! 2D physics: gravity, friction, elastic collision for UI elements.

use ltk_core::geometry::{Point, Size, Rect};

#[derive(Debug, Clone)]
pub struct PhysicsBody {
    pub position:  Point,
    pub velocity:  Point,
    pub mass:      f32,
    pub restitution: f32,   // bounciness 0.0–1.0
    pub friction:  f32,
    pub is_static: bool,
}

impl PhysicsBody {
    pub fn new(pos: Point, mass: f32) -> Self {
        Self { position: pos, velocity: Point::ZERO, mass, restitution: 0.3, friction: 0.8, is_static: false }
    }

    pub fn apply_force(&mut self, force: Point, dt: f32) {
        if self.is_static { return; }
        let ax = force.x / self.mass;
        let ay = force.y / self.mass;
        self.velocity.x += ax * dt;
        self.velocity.y += ay * dt;
    }

    pub fn apply_gravity(&mut self, g: f32, dt: f32) {
        if !self.is_static { self.velocity.y += g * dt; }
    }

    pub fn integrate(&mut self, dt: f32) {
        if self.is_static { return; }
        self.position.x += self.velocity.x * dt;
        self.position.y += self.velocity.y * dt;
        self.velocity.x *= 1.0 - self.friction * dt;
        self.velocity.y *= 1.0 - self.friction * dt;
    }

    pub fn bounce_floor(&mut self, floor_y: f32, half_h: f32) {
        if self.position.y + half_h >= floor_y {
            self.position.y = floor_y - half_h;
            self.velocity.y = -self.velocity.y * self.restitution;
        }
    }
}
