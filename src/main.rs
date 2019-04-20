extern crate kiss3d;
extern crate nalgebra as na;

use na::{Vector3, UnitQuaternion};
use kiss3d::window::Window;
use kiss3d::light::Light;

mod cubeblock;
mod keyevents;
mod playfield;

fn main() {
    let mut window = Window::new("p1mm3lTr0n");
    
    let mut maingroup = window.add_group();

    let mut cubes = cubeblock::add_cubeblock(&mut maingroup, 3);
    playfield::add_playfield(&mut maingroup);

    window.set_light(Light::StickToCamera);
    let rotx = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), 0.014);

    while window.render() { 
        keyevents::handle(&mut window, &mut maingroup);
        cubes.prepend_to_local_rotation(&rotx);
    }

}

