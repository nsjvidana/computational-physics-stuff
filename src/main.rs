#[cfg(all(not(feature = "dim2"), not(feature = "dim3")))]
compile_error!("Features dim2 or dim3 must be enabled!");
#[cfg(all(feature = "dim2", feature = "dim3"))]
compile_error!("Features dim2 and dim3 can't be enabled at same time!");

#[cfg(feature = "dim2")]
extern crate parry2d as parry;
#[cfg(feature = "dim3")]
extern crate parry3d as parry;

mod math;

use bevy::DefaultPlugins;
use bevy::prelude::{App, ButtonInput, Camera3d, Color, Commands, Gizmos, KeyCode, Res, ResMut, Resource, Startup, Transform, Update, Vec2, Vec3};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use crate::math::*;

const DRAW_STEPS: usize = 10;
const X: f32 = 1.715;

/// The function being interpolated
fn f(x: f32) -> f32 {
    f32::sin(x)
}

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        NoCameraPlayerPlugin,
    ))
        .add_systems(Startup, startup)
        .add_systems(Update, update);

    // Compute points for drawing the function
    let f_domain: [f32; 2] = [1.2, 2.2];
    let mut points = Vec::with_capacity(DRAW_STEPS);
    let step_len = (f_domain[1] - f_domain[0]) / DRAW_STEPS as f32;
    for i in 0..DRAW_STEPS {
        let x = f_domain[0] + step_len * i as f32;
        let y = f(x);
        points.push(Vec2::new(x, y));
    }
    app.insert_resource(FunctionDrawData {
        domain: f_domain,
        points,
    });

    // Compute the known points that will be used in the interpolation
    let original = vec![
        Vec2::new(1.42, f(1.42)),
        Vec2::new(1.64, f(1.64)),
        Vec2::new(1.80, f(1.80)),
        Vec2::new(1.93, f(1.93)),
        Vec2::new(2.0, f(2.0)),
    ];
    app.insert_resource(InterpolationData {
        f: original.clone(),
        original,
    });

    app.run();
}

fn startup(drawing: Res<FunctionDrawData>, mut commands: Commands) {
    let cam_x = (drawing.domain[0] + drawing.domain[1]) / 2.;
    let mut cam_y = 0.;
        for pt in drawing.points.iter() {
            cam_y += pt.y;
        }
        cam_y /= DRAW_STEPS as f32;
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(cam_x, cam_y, 4.))
            .looking_at(Vec3::new(cam_x, cam_y, 0.), Vec3::Y),
        FlyCam
    ));
}

fn update(
    drawing: Res<FunctionDrawData>,
    mut interpol: ResMut<InterpolationData>,
    keys: Res<ButtonInput<KeyCode>>,
    mut gizmos: Gizmos,
) {
    // Draw the function
    for i in 1..DRAW_STEPS {
        gizmos.line_2d(drawing.points[i-1], drawing.points[i], Color::WHITE);
    }

    // Draw current interpolation step
    for pt in interpol.f.iter() {
        gizmos.circle_2d(Iso::from_translation(*pt), 0.01, Color::linear_rgb(0., 1., 0.));
    }
    //Draw actual solution
    gizmos.circle_2d(Iso::from_xy(X, f(X)), 0.01, Color::linear_rgb(1., 0., 0.));

    if !keys.just_pressed(KeyCode::KeyT) { return; }
    let n = interpol.original.len()-1;
    let f = &mut interpol.f;
    for i in 0..n {
        for j in 0..n-i {
            let new_f = (X - f[j].x) / (f[i+j+1].x - f[j].x) * f[j+1].y
                + (X - f[i+j+1].x) / (f[j].x - f[i+j+1].x) * f[j].y;
            f[j].y = new_f;
        }
    }
    *f = vec![Vec2::new(X, f[0].y)];
}

fn lin_interpol(x: f32, p1: Vec2, p2: Vec2) -> f32 {
    let df = p2.y - p1.y;
    p1.y + (x - p1.x)/(p2.x - p1.x) * df
}

#[derive(Resource)]
struct FunctionDrawData {
    points: Vec<Vec2>,
    /// Domain of the function to draw
    domain: [f32; 2],
}

#[derive(Resource)]
struct InterpolationData {
    /// Original given points. Used for drawing only.
    original: Vec<Vec2>,
    /// Points the current iteration of Aitken's Method will be computing with
    f: Vec<Vec2>,
}
