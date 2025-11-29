use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{App, Color, KeyCode, Res, ResMut, Resource, Update};
use crate::math::*;
use crate::unit6_approximation::DrawData;

pub fn aitken_demo(app: &mut App, knowns: Vec<Vect>, x: Real) {
    app
        .insert_resource(Aitken {
            fi: knowns.iter().map(|v| v.y).collect(),
            xi: knowns.iter().map(|v| v.x).collect(),
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
            fi[j] = lin_interpol(x, (x0, y0), (x1, y1));
        }
    }
    println!("Aitken step {i}");
    // Advance the algorithm by one step.
    *i += 1;
    
    // Draw the resulting points
    let i = *i;
    for j in 0..(n-i) {
        let x0 = xi[j];
        let y0 = fi[j];
        let x1 = xi[i + j+1];
        let y1 = fi[j+1];

        let color_mul = i as Real / n as Real;
        let color = Color::linear_rgb(0., 1. * color_mul, 1. * color_mul);
        draw_data.lines.push((
            Line::new((x0, y0), (x1, y1)),
            color
        ));
    }
    if i == n {
        println!("Aitken algorithm finished. Drawing approximation result in green.");
        draw_data.points.push((Vect::new(x, fi[0]), Color::linear_rgb(0., 1., 0.)));
    }
}

#[derive(Resource)]
pub struct Aitken {
    fi: Vec<Real>,
    xi: Vec<Real>,
    x: Real,
    i: usize
}
