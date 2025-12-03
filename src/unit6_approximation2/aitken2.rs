use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{App, Color, Gizmos, KeyCode, Res, ResMut, Resource, Update};
use parry::na;
use crate::{f, DrawData, DRAW_STEPS, X};
use crate::math::*;
use crate::utils::W;

pub fn aitken_demo(app: &mut App, x: Real) {
    // Compute points for drawing the function
    let f_domain: [BevyReal; 2] = [1.2, 2.2];
    let mut curve_points = Vec::with_capacity(DRAW_STEPS);
    let step_len = (f_domain[1] - f_domain[0]) / DRAW_STEPS as BevyReal;
    for i in 0..DRAW_STEPS {
        let x = f_domain[0] + step_len * i as BevyReal;
        let y = f(x as Real);
        curve_points.push(Vec2::new(x, y as _));
    }

    // Known points to interpolate with
    let knowns = vec![
        Vec2::new(1.42, f(1.42) as _),
        Vec2::new(1.64, f(1.64) as _),
        Vec2::new(1.80, f(1.80) as _),
        Vec2::new(1.93, f(1.93) as _),
        Vec2::new(2.0, f(2.0) as _),
    ];

    let mut draw_data = app.world_mut().resource_mut::<DrawData>();
    draw_data.points.append(&mut knowns.iter()
        .map(|v| (*v, Color::WHITE))
        .collect()
    );
    draw_data.curve_points = curve_points;
    draw_data.domain = f_domain;
    draw_data.points.push((Vec2::new(X as _, f(X) as _), Color::linear_rgb(1., 0., 0.)));

    app
        .insert_resource(Aitken {
            fi: knowns.iter().map(|v| v.y as Real).collect(),
            xi: knowns.iter().map(|v| v.x as Real).collect(),
            x,
            i: 0
        })
        .add_systems(Update, update);
}

fn update(
    mut draw_data: ResMut<DrawData>,
    keys: Res<ButtonInput<KeyCode>>,
    mut aitken: ResMut<Aitken>,
) {
    let i = aitken.i;
    let n = aitken.xi.len()-1;
    if !keys.just_pressed(KeyCode::KeyT) { return; }
    if i >= n {
        println!("Aitken algorithm is already finished.");
        return;
    }

    let x = aitken.x;
    let Aitken {
        fi,
        xi,
        i,
        ..
    } = &mut *aitken;
    // Step the algorithm
    {
        let i = *i;
        for j in 0..(n-i) {
            let x0 = xi[j];
            let y0 = fi[j];
            let x1 = xi[i + j+1];
            let y1 = fi[j+1];
            fi[j] = (x-x0) / (x1 - x0) * y1
                +(x-x1) / (x0 - x1) * y0;
            fi[j] = lin_interpol(x, [x0, y0], [x1, y1]);
        }
    }
    println!("Aitken step {i}");
    // Advance the algorithm by one step.
    *i += 1;
    
    // Draw the resulting points
    let i = *i;
    for j in 0..(n-i) {
        let x0 = xi[j] as BevyReal;
        let y0 = fi[j] as BevyReal;
        let x1 = xi[i + j+1] as BevyReal;
        let y1 = fi[j+1] as BevyReal;

        let color_mul = i as BevyReal / n as BevyReal;
        let color = Color::linear_rgb(0., 1. * color_mul, 1. * color_mul);
        draw_data.lines.push((
            Line::new((x0, y0), (x1, y1)),
            color
        ));
    }
    if i == n {
        println!("Aitken algorithm finished. Drawing approximation result in green.");
        draw_data.points.push((
            Vect::new(x as BevyReal, fi[0] as BevyReal),
            Color::linear_rgb(0., 1., 0.)
        ));
    }
}

fn draw_aitken(
    drawing: Res<DrawData>,
    mut gizmos: Gizmos,
) {
    // Draw the function
    for i in 1..DRAW_STEPS {
        #[cfg(feature = "dim2")]
        gizmos.line_2d(drawing.curve_points[i-1], drawing.curve_points[i], Color::WHITE);
    }
}

#[derive(Resource)]
pub struct Aitken {
    fi: Vec<Real>,
    xi: Vec<Real>,
    x: Real,
    i: usize
}
