pub use self::math::*;
pub use parry::math::*;

pub type Real = parry::math::SimdReal;

#[cfg(feature = "dim2")]
mod math {
    use bevy::math::{Isometry2d, Vec2};

    pub type Vect = Vec2;
    pub type Iso = Isometry2d;

    pub fn lin_interpol(x: f32, p0: impl Into<Vect>, p1: impl Into<Vect>) -> f32 {
        let p0 = p0.into();
        let p1 = p1.into();
        p0.y + (x - p0.x)/(p1.x - p0.x) * (p1.y - p0.y)
    }
}

#[cfg(feature = "dim3")]
mod math {
    use bevy::math::{Isometry3d, Vec3};

    pub type Vect = Vec3;
    pub type Iso = Isometry3d;
}

#[derive(Default)]
pub struct Line {
    pub start: Vect,
    pub end: Vect,
}

impl Line {
    pub fn new(start: impl Into<Vect>, end: impl Into<Vect>) -> Self {
        Self {
            start: start.into(),
            end: end.into(),
        }
    }
}
