use std::ops::Deref;
pub use self::math::*;
pub use parry::math::*;
use crate::utils::W;

pub type Real = SimdReal;
pub type BevyReal = f32;

#[cfg(feature = "dim2")]
mod math {
    use bevy::math::{Isometry2d, Vec2};
    use crate::math::*;

    pub type Vect = Vec2;
    pub type Iso = Isometry2d;

    pub fn lin_interpol(x: Real, p0: impl Into<Vector<Real>>, p1: impl Into<Vector<Real>>) -> Real {
        let p0 = p0.into();
        let p1 = p1.into();
        p0.y + (x - p0.x)/(p1.x - p0.x) * (p1.y - p0.y)
    }

    /// Convert a flat index into a 2D Vector.
    pub fn flat_idx_to_vector(i: usize, grid_dimensions: Vector<usize>) -> Vector<usize> {
        Vector::new(i % grid_dimensions.x, i / grid_dimensions.y)
    }
}

#[cfg(feature = "dim3")]
mod math {
    use bevy::math::{Isometry3d, Vec3};
    use crate::math::*;

    pub type Vect = Vec3;
    pub type Iso = Isometry3d;

    /// Convert a flat index into a vector whose dimensions are as follows:
    /// `x`: row index of x-y plane
    /// `y`: column index of x-y plane
    /// `z`: index of the x-y plane that the x and y indices are on
    pub fn flat_idx_to_vector(i: usize, grid_dimensions: Vector<usize>) -> Vector<usize> {
        Vector::new(
            i % grid_dimensions.x,
            (i / grid_dimensions.y) % grid_dimensions.y,
            i / (grid_dimensions.x * grid_dimensions.y)
        )
    }
}


impl Into<Vector<Real>> for W<(Real, Real)> {
    fn into(self) -> Vector<Real> {
        let (x, y) = *self;
        #[cfg(feature = "dim2")]
        return Vector::new(x, y);

        #[cfg(feature = "dim3")]
        Vector::new(x, y, 0.)
    }
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
