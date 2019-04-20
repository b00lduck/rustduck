
use na::{Translation3, Vector3, UnitQuaternion};
use kiss3d::window::Window;
use kiss3d::scene::SceneNode;
use kiss3d::event::{Action, WindowEvent, Key};


pub fn handle(window :&mut Window, scene_node: &mut SceneNode) {

    let rotx = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), 0.014);
    let rotxneg = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), -0.014);

    while window.render() { 

        for mut event in window.events().iter() {
            match event.value {
                WindowEvent::Key(button, Action::Press, _) => {
                    match button {
                        Key::W => scene_node.prepend_to_local_translation(&(Translation3::new(0.0f32, 0.0f32, -0.5f32))),
                        Key::S => scene_node.prepend_to_local_translation(&(Translation3::new(0.0f32, 0.0f32, 0.5f32))),
                        Key::A => scene_node.prepend_to_local_rotation(&rotx),
                        Key::D => scene_node.prepend_to_local_rotation(&rotxneg),
                        _ => {},
                    }
                    event.inhibited = true; // override the default keyboard handler
                }
                _ => {}
            }
        }
    }

}

