#[cfg(all(not(feature = "dim2"), not(feature = "dim3")))]
compile_error!("Features dim2 or dim3 must be enabled!");
#[cfg(all(feature = "dim2", feature = "dim3"))]
compile_error!("Features dim2 and dim3 can't be enabled at same time!");

#[cfg(feature = "dim2")]
extern crate parry2d as parry;
#[cfg(feature = "dim3")]
extern crate parry3d as parry;

mod math;
mod unit6_approximation;

use bevy::DefaultPlugins;
use bevy::prelude::{App, ButtonInput, Camera3d, Color, Commands, Gizmos, KeyCode, PostUpdate, Res, ResMut, Resource, Startup, Transform, Update, Vec2, Vec3};
use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};
use crate::math::*;
use crate::unit6_approximation::aitken::aitken_demo;
use crate::unit6_approximation::DrawData;

const DRAW_STEPS: usize = 10;
const X: f32 = 1.715;

/// The function being interpolated
fn f(x: f32) -> f32 {
    f32::sin(x)
}

fn main() {
    // Compute points for drawing the function
    let f_domain: [f32; 2] = [1.2, 2.2];
    let mut curve_points = Vec::with_capacity(DRAW_STEPS);
    let step_len = (f_domain[1] - f_domain[0]) / DRAW_STEPS as f32;
    for i in 0..DRAW_STEPS {
        let x = f_domain[0] + step_len * i as f32;
        let y = f(x);
        curve_points.push(Vec2::new(x, y));
    }

    // Known points to interpolate with
    let knowns = vec![
        Vec2::new(1.42, f(1.42)),
        Vec2::new(1.64, f(1.64)),
        Vec2::new(1.80, f(1.80)),
        Vec2::new(1.93, f(1.93)),
        Vec2::new(2.0, f(2.0)),
    ];

    let mut app = App::new();

    let mut draw_data = DrawData {
        points: knowns.iter()
            .map(|v| (*v, Color::WHITE))
            .collect(),
        curve_points,
        domain: f_domain,
        ..Default::default()
    };
    draw_data.points.push((Vect::new(X, f(X)), Color::linear_rgb(1., 0., 0.)));

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

    aitken_demo(&mut app, knowns, X);

    app.run();
}

fn startup(d: Res<DrawData>, mut commands: Commands) {
    // Spawning camera at the right position
    let cam_x = (d.domain[0] + d.domain[1]) / 2.;
    let mut cam_y = d.points.iter().map(|(pt, _)| pt.y).sum::<f32>() / DRAW_STEPS as f32;
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(cam_x, cam_y, 2.))
            .looking_at(Vec3::new(cam_x, cam_y, 0.), Vec3::Y),
        FlyCam
    ));
}

fn draw(
    drawing: Res<DrawData>,
    mut gizmos: Gizmos,
) {
    // Draw the function
    for i in 1..DRAW_STEPS {
        gizmos.line_2d(drawing.curve_points[i-1], drawing.curve_points[i], Color::WHITE);
    }

    // Draw cicles around points
    for (p, c) in drawing.points.iter() {
        gizmos.circle_2d(*p, 0.01, *c);
    }

    // Draw line segments
    for (seg, c) in drawing.lines.iter() {
        gizmos.line_2d(seg.start, seg.end, *c);
    }
}
