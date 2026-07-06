//! 2D and 3D geometric primitives (logical-pixel based).

use std::ops::{Add, Sub, Mul, Neg};

// ── Point ────────────────────────────────────────────────────────────

/// A 2-D point in logical-pixel space.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point { pub x: f32, pub y: f32 }

impl Point {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    #[inline] pub fn new(x: f32, y: f32) -> Self { Self { x, y } }
    #[inline] pub fn distance(self, other: Self) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
    #[inline] pub fn lerp(self, other: Self, t: f32) -> Self {
        Self { x: self.x + (other.x - self.x) * t, y: self.y + (other.y - self.y) * t }
    }
    #[inline] pub fn to_physical(self, scale: f32) -> PhysicalPoint {
        PhysicalPoint { x: (self.x * scale).round() as i32, y: (self.y * scale).round() as i32 }
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self { Self { x: self.x + rhs.x, y: self.y + rhs.y } }
}
impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self { Self { x: self.x - rhs.x, y: self.y - rhs.y } }
}
impl Neg for Point {
    type Output = Self;
    fn neg(self) -> Self { Self { x: -self.x, y: -self.y } }
}

/// A point in physical (device) pixel space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PhysicalPoint { pub x: i32, pub y: i32 }

impl PhysicalPoint {
    pub fn to_logical(self, scale: f32) -> Point {
        Point { x: self.x as f32 / scale, y: self.y as f32 / scale }
    }
}

// ── Size ─────────────────────────────────────────────────────────────

/// A 2-D size in logical pixels.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Size { pub width: f32, pub height: f32 }

impl Size {
    pub const ZERO: Self     = Self { width: 0.0, height: 0.0 };
    pub const INFINITY: Self = Self { width: f32::INFINITY, height: f32::INFINITY };

    #[inline] pub fn new(w: f32, h: f32) -> Self { Self { width: w, height: h } }
    #[inline] pub fn area(self) -> f32 { self.width * self.height }
    #[inline] pub fn is_empty(self) -> bool { self.width <= 0.0 || self.height <= 0.0 }
    #[inline] pub fn ceil(self) -> Self {
        Self { width: self.width.ceil(), height: self.height.ceil() }
    }
    #[inline] pub fn floor(self) -> Self {
        Self { width: self.width.floor(), height: self.height.floor() }
    }
    #[inline] pub fn min(self, other: Self) -> Self {
        Self { width: self.width.min(other.width), height: self.height.min(other.height) }
    }
    #[inline] pub fn max(self, other: Self) -> Self {
        Self { width: self.width.max(other.width), height: self.height.max(other.height) }
    }
    #[inline] pub fn to_physical(self, scale: f32) -> PhysicalSize {
        PhysicalSize { width: (self.width * scale).ceil() as u32, height: (self.height * scale).ceil() as u32 }
    }
}

impl Mul<f32> for Size {
    type Output = Self;
    fn mul(self, s: f32) -> Self { Self { width: self.width * s, height: self.height * s } }
}

/// A size in physical (device) pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PhysicalSize { pub width: u32, pub height: u32 }

impl PhysicalSize {
    pub fn to_logical(self, scale: f32) -> Size {
        Size { width: self.width as f32 / scale, height: self.height as f32 / scale }
    }
}

// ── Rect ─────────────────────────────────────────────────────────────

/// An axis-aligned rectangle in logical pixels.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect { pub origin: Point, pub size: Size }

impl Rect {
    pub const ZERO: Self = Self { origin: Point::ZERO, size: Size::ZERO };

    #[inline] pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { origin: Point::new(x, y), size: Size::new(w, h) }
    }
    #[inline] pub fn from_points(min: Point, max: Point) -> Self {
        Self { origin: min, size: Size::new(max.x - min.x, max.y - min.y) }
    }
    #[inline] pub fn x(self) -> f32 { self.origin.x }
    #[inline] pub fn y(self) -> f32 { self.origin.y }
    #[inline] pub fn width(self) -> f32 { self.size.width }
    #[inline] pub fn height(self) -> f32 { self.size.height }
    #[inline] pub fn min_x(self) -> f32 { self.origin.x }
    #[inline] pub fn min_y(self) -> f32 { self.origin.y }
    #[inline] pub fn max_x(self) -> f32 { self.origin.x + self.size.width }
    #[inline] pub fn max_y(self) -> f32 { self.origin.y + self.size.height }
    #[inline] pub fn center(self) -> Point {
        Point::new(self.origin.x + self.size.width * 0.5, self.origin.y + self.size.height * 0.5)
    }
    #[inline] pub fn contains(self, p: Point) -> bool {
        p.x >= self.min_x() && p.x <= self.max_x() && p.y >= self.min_y() && p.y <= self.max_y()
    }
    #[inline] pub fn intersects(self, other: Self) -> bool {
        self.min_x() < other.max_x() && self.max_x() > other.min_x()
        && self.min_y() < other.max_y() && self.max_y() > other.min_y()
    }
    #[inline] pub fn intersection(self, other: Self) -> Option<Self> {
        let x0 = self.min_x().max(other.min_x());
        let y0 = self.min_y().max(other.min_y());
        let x1 = self.max_x().min(other.max_x());
        let y1 = self.max_y().min(other.max_y());
        if x1 > x0 && y1 > y0 { Some(Self::from_points(Point::new(x0,y0), Point::new(x1,y1))) }
        else { None }
    }
    #[inline] pub fn union(self, other: Self) -> Self {
        Self::from_points(
            Point::new(self.min_x().min(other.min_x()), self.min_y().min(other.min_y())),
            Point::new(self.max_x().max(other.max_x()), self.max_y().max(other.max_y())),
        )
    }
    #[inline] pub fn inset(self, insets: Insets) -> Self {
        Self::new(
            self.x() + insets.left,
            self.y() + insets.top,
            self.width() - insets.left - insets.right,
            self.height() - insets.top - insets.bottom,
        )
    }
    #[inline] pub fn is_empty(self) -> bool { self.size.is_empty() }
}

// ── Insets ───────────────────────────────────────────────────────────

/// Padding/margin/border values on all four sides.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Insets { pub top: f32, pub right: f32, pub bottom: f32, pub left: f32 }

impl Insets {
    pub const ZERO: Self = Self { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 };

    #[inline] pub fn all(v: f32) -> Self { Self { top: v, right: v, bottom: v, left: v } }
    #[inline] pub fn horizontal(h: f32) -> Self { Self { left: h, right: h, ..Self::ZERO } }
    #[inline] pub fn vertical(v: f32) -> Self { Self { top: v, bottom: v, ..Self::ZERO } }
    #[inline] pub fn xy(h: f32, v: f32) -> Self { Self { top: v, bottom: v, left: h, right: h } }
    #[inline] pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self { Self { top, right, bottom, left } }
    #[inline] pub fn horizontal_sum(self) -> f32 { self.left + self.right }
    #[inline] pub fn vertical_sum(self) -> f32 { self.top + self.bottom }
}

// ── Transform2D ──────────────────────────────────────────────────────

/// Column-major 3×3 affine transform matrix for 2D operations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform2D {
    // [a, b, c, d, tx, ty] in CSS transform notation
    pub a: f32, pub b: f32,
    pub c: f32, pub d: f32,
    pub tx: f32, pub ty: f32,
}

impl Transform2D {
    pub const IDENTITY: Self = Self { a: 1.0, b: 0.0, c: 0.0, d: 1.0, tx: 0.0, ty: 0.0 };

    pub fn translation(tx: f32, ty: f32) -> Self { Self { tx, ty, ..Self::IDENTITY } }
    pub fn scale(sx: f32, sy: f32) -> Self { Self { a: sx, d: sy, ..Self::IDENTITY } }
    pub fn rotation(angle_rad: f32) -> Self {
        let (s, c) = angle_rad.sin_cos();
        Self { a: c, b: s, c: -s, d: c, tx: 0.0, ty: 0.0 }
    }
    pub fn transform_point(self, p: Point) -> Point {
        Point { x: self.a * p.x + self.c * p.y + self.tx, y: self.b * p.x + self.d * p.y + self.ty }
    }
    pub fn then(self, other: Self) -> Self {
        Self {
            a:  self.a * other.a  + self.b * other.c,
            b:  self.a * other.b  + self.b * other.d,
            c:  self.c * other.a  + self.d * other.c,
            d:  self.c * other.b  + self.d * other.d,
            tx: self.tx * other.a + self.ty * other.c + other.tx,
            ty: self.tx * other.b + self.ty * other.d + other.ty,
        }
    }
    pub fn is_identity(self) -> bool { self == Self::IDENTITY }
}

impl Default for Transform2D {
    fn default() -> Self { Self::IDENTITY }
}

// ── SizeConstraint ───────────────────────────────────────────────────

/// Layout measurement constraint (min/max/definite).
#[derive(Debug, Clone, Copy)]
pub struct SizeConstraint {
    pub min:      Size,
    pub max:      Size,
    pub definite: Option<Size>,
}

impl SizeConstraint {
    pub const UNBOUNDED: Self = Self {
        min: Size::ZERO,
        max: Size::INFINITY,
        definite: None,
    };

    pub fn tight(size: Size) -> Self { Self { min: size, max: size, definite: Some(size) } }
    pub fn loose(max: Size) -> Self  { Self { min: Size::ZERO, max, definite: None } }
    pub fn clamp(self, size: Size) -> Size { size.max(self.min).min(self.max) }
}
