#[cfg(all(not(feature = "dim2"), not(feature = "dim3")))]
compile_error!("Features dim2 or dim3 must be enabled!");
#[cfg(all(feature = "dim2", feature = "dim3"))]
compile_error!("Features dim2 and dim3 can't be enabled at same time!");

#[cfg(feature = "dim2")]
extern crate parry2d as parry;
#[cfg(feature = "dim3")]
extern crate parry3d as parry;

mod math;

use bevy::prelude::App;

fn main() {
    let mut app = App::new();
}
