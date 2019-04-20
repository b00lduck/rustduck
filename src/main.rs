extern crate kiss3d;
extern crate nalgebra as na;

use na::{Vector3, UnitQuaternion, Translation3};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;

fn main() {
    let mut window = Window::new("Kiss3d: wireframe");

    let mut cs: [SceneNode; 6] = [
        cube(&mut window, 0, 0),
        cube(&mut window, 1, 0),
        cube(&mut window, 0, 1),
        cube(&mut window, 1, 1),
        cube(&mut window, 0, 2),
        cube(&mut window, 1, 2)
    ];

    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    while window.render() { 
        for mut x in &mut cs {
            x.prepend_to_local_rotation(&rot);
        }
    }
}

fn cube(window: &mut Window, n: i32, x: i32) -> SceneNode {

    let mut c = window.add_cube(1.0, 1.0, 1.0);
    let transe = Translation3::new((x*2) as f32, 0.0f32, 0.0f32);

    if n == 0 {
        c.set_color(0.0, 1.0, 0.5);
        c.set_points_size(5.0);
        c.set_lines_width(1.0);    
        c.set_surface_rendering_activation(false);
    } else {
        c.set_color(0.7, 0.2, 0.1);
    }

    c.set_local_translation(transe);

    return c
}