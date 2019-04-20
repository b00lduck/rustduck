extern crate kiss3d;
extern crate nalgebra as na;

use na::{Vector3, UnitQuaternion};
use kiss3d::window::Window;
use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::event::{Action, WindowEvent, Key};

mod cubeblock;

fn main() {
    let mut window = Window::new("p1mm3lTr0n");
    
    let mut maingroup = window.add_group();

    let mut cubes = cubeblock::add_cubeblock(&mut maingroup, 3);

    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
    let rotx = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), 0.014);

    while window.render() { 

        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::Key(button, Action::Press, _) => {
                    match button {
                        //Key::W => window.caprepend_to_local_rotation(&rotx),
  //                      Key::Z => for mut x in &mut cs {
//                             x.prepend_to_local_rotation(&rotx);
    //                    }
                        _ => {},
                    }
                    event.inhibited = true; // override the default keyboard handler
                }
                _ => {}
            }
        }
        cubes.prepend_to_local_rotation(&rot);
    }

}

