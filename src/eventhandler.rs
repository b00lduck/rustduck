
//use na::{Translation3, Vector3, UnitQuaternion};
use kiss3d::window::Window;
use kiss3d::event::{Action, WindowEvent, Key};

use target::Node;

pub fn move_event(window :&mut Window, scene_node: &mut Node) {

//    let rotx = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), 0.014);
//    let rotxneg = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), -0.014);


        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::Key(button, Action::Press, _) => {
                    match button {
                        Key::W => scene_node.move_z(0.1),
                        Key::S => scene_node.move_z(-0.1),
                        Key::A => scene_node.move_x(0.1),
                        Key::D => scene_node.move_x(-0.1),
                        Key::Q => scene_node.move_y(0.1),
                        Key::E => scene_node.move_y(-0.1),
//                        Key::A => scene_node.rot_y(0.1),
//                        Key::D => scene_node.rot_y(-0.1),
                        _ => {},
                    }
                    event.inhibited = true;

                     // override the default keyboard handler
                }
                _ => {}
            }
        }
    

}

