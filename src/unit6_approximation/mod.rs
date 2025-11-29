use bevy::math::Vec2;
use bevy::prelude::{Color, Resource};
use crate::math::{Line, Real, Vect};

pub mod aitken;

#[derive(Resource, Default)]
pub struct DrawData {
    /// Points to draw a circle around.
    pub points: Vec<(Vect, Color)>,
    /// Line segments to draw (if any).
    pub lines: Vec<(Line, Color)>,

    /// Points used in drawing the curve.
    pub curve_points: Vec<Vec2>,
    /// Domain of the function to draw. Used in positioning the camera.
    pub domain: [Real; 2],
}