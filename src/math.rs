pub use self::math::*;
pub use parry::math::*;

#[cfg(feature = "dim2")]
mod math {
    use bevy::math::{Isometry2d, Vec2};

    pub type Vect = Vec2;
    pub type Iso = Isometry2d;
}


#[cfg(feature = "dim3")]
mod math {
    use bevy::math::{Isometry3d, Vec3};

    pub type Vect = Vec3;
    pub type Iso = Isometry3d;
}
