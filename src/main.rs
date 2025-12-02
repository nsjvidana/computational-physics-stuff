#[cfg(all(not(feature = "dim2"), not(feature = "dim3")))]
compile_error!("Features dim2 or dim3 must be enabled!");
#[cfg(all(feature = "dim2", feature = "dim3"))]
compile_error!("Features dim2 and dim3 can't be enabled at same time!");

#[cfg(feature = "dim2")]
extern crate parry2d_f64 as parry;
#[cfg(feature = "dim3")]
extern crate parry3d_f64 as parry;

mod math;
#[cfg(feature = "dim2")]
mod unit6_approximation2;
mod unit1_project;
mod utils;

use crate::math::*;
use bevy::prelude::{App, Camera3d, Color, Commands, Gizmos, PostUpdate, Res, Resource, Startup, Transform, Vec2, Vec3};
use bevy::DefaultPlugins;
use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};
use crate::utils::W;

const DRAW_STEPS: usize = 10;
const X: Real = 1.715;

/// The function being interpolated
fn f(x: Real) -> Real {
    Real::sin(x)
}

fn main() {
    let draw_data = DrawData::default();

    let mut app = App::new();
    app
        .insert_resource(MovementSettings {
            speed: 2.,
            ..Default::default()
        })
        .add_plugins((
            DefaultPlugins,
            NoCameraPlayerPlugin
        ))
        .insert_resource(draw_data)
        .add_systems(Startup, startup)
        .add_systems(PostUpdate, draw);

    unit6_approximation2::aitken2::aitken_demo(&mut app, X);

    app.run();
}

fn startup(d: Res<DrawData>, mut commands: Commands) {
    // Spawning camera at the right position
    let cam_x = (d.domain[0] + d.domain[1]) / 2.;
    let cam_y = d.points.iter().map(|(pt, _)| pt.y).sum::<BevyReal>() / DRAW_STEPS as BevyReal;
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(cam_x, cam_y, 2.))
            .looking_at(Vec3::new(cam_x, cam_y, 0.), Vec3::Y),
        FlyCam
    ));
}

#[derive(Resource, Default)]
pub struct DrawData {
    /// Points to draw a circle/sphere around.
    pub points: Vec<(Vect, Color)>,
    /// Line segments to draw.
    pub lines: Vec<(Line, Color)>,
    /// Arrows to draw
    pub arrows: Vec<(Line, Color)>,

    /// Points used in drawing the curve.
    pub curve_points: Vec<Vec2>,
    /// Domain of the function to draw. Used in positioning the camera.
    pub domain: [BevyReal; 2],
}

fn draw(
    drawing: Res<DrawData>,
    mut gizmos: Gizmos,
) {
    // Draw the function
    for i in 1..DRAW_STEPS {
        #[cfg(feature = "dim2")]
        gizmos.line_2d(drawing.curve_points[i-1], drawing.curve_points[i], Color::WHITE);
    }

    // Draw cicles/spheres around points
    for (p, c) in drawing.points.iter() {
        #[cfg(feature = "dim2")]
        gizmos.circle_2d(*p, 0.01, *c);
        #[cfg(feature = "dim3")]
        gizmos.sphere(*p, 0.01, *c);
    }

    // Draw line segments
    for (seg, c) in drawing.lines.iter() {
        #[cfg(feature = "dim2")]
        gizmos.line_2d(seg.start, seg.end, *c);
        #[cfg(feature = "dim3")]
        gizmos.line(seg.start, seg.end, *c)
    }

    // Draw arrows
    for (arrow, c) in drawing.arrows.iter() {
        #[cfg(feature = "dim2")]
        gizmos.arrow_2d(arrow.start, arrow.end, *c);
        #[cfg(feature = "dim3")]
        gizmos.arrow(arrow.start, arrow.end, *c);
    }
}
