use std::ops::Sub;
use bevy::prelude::{App, Color, Commands, Component, Query, ResMut, Startup, Update};
use crate::DrawData;
use crate::math::*;
use crate::utils::W;

pub const COULOMB_CONSTANT: Real = 8987551784.952723;

const BOX_SIZE: f32 = 3.;
const VECTOR_FIELD_RESOLUTION: usize = 20;
const VECTOR_FIELD_INCREMENT: f32 = BOX_SIZE / (VECTOR_FIELD_RESOLUTION - 1) as f32;

pub fn visualize_electric_fields(app: &mut App) {
    app
        .add_systems(Startup, setup_particles)
        .add_systems(Update, draw_electric_field);
}

fn setup_particles(mut commands: Commands) {
    commands.spawn(PointCharge::new(0.1, W((-1., 0.)).into()));
    commands.spawn(PointCharge::new(-0.1, W(( 1., 0.)).into()));
}

fn draw_electric_field(
    charges: Query<&PointCharge>,
    mut draw_data: ResMut<DrawData>,
) {
    draw_data.arrows.clear();
    for i in 0..VECTOR_FIELD_RESOLUTION.pow(DIM as _) {
        let mut r = flat_idx_to_vector(i, Vector::from_element(VECTOR_FIELD_RESOLUTION))
            .map(|v| v as Real * VECTOR_FIELD_INCREMENT as Real)
            .sub(Vector::from_element(BOX_SIZE as Real / 2.));
        let e_dir = e(r, charges.iter()).normalize() * VECTOR_FIELD_INCREMENT as Real / 2.;

        draw_data.arrows.push((
            Line::new(W(r), W(r + e_dir)),
            Color::WHITE
        ));
    }
}

/// Electric field at location `r`
fn e<'a>(r: Vector<Real>, charges: impl Iterator<Item = &'a PointCharge>) -> Vector<Real> {
    let mut e = Vector::zeros();
    for PointCharge { q: q_i, r: r_i } in charges {
        e += COULOMB_CONSTANT * q_i * (r - r_i) / (r - r_i).norm().powi(3);
    }
    e
}

/// A charge at a point in space
#[derive(Component)]
struct PointCharge {
    /// The charge of the particle
    q: Real,
    /// The charge's location
    r: Vector<Real>,
}

impl PointCharge {
    pub fn new(q: Real, r: Vector<Real>) -> Self {
        Self {
            q,
            r: r.into()
        }
    }
}