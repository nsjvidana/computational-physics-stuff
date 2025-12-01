use bevy::prelude::App;
use crate::math::*;

pub const COULOMB_CONSTANT: Real = 8987551784.952723;

pub fn visualize_electric_fields(app: &mut App) {

}

/// Electric field at location `r`
fn e(r: Vector<Real>, charges: Vec<PointCharge>) -> Vector<Real> {
    let mut e = Vector::zeros();
    for PointCharge { q: q_i, r: r_i } in charges.iter() {
        e += COULOMB_CONSTANT * q_i * (r - r_i) / (r - r_i).norm().powi(3);
    }
    e
}

/// A charge at a point in space
struct PointCharge {
    /// The charge of the particle
    q: Real,
    /// The charge's location
    r: Vector<Real>,
}