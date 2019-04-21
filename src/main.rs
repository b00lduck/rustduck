extern crate kiss3d;
extern crate nalgebra as na;

//use na::{Vector3, UnitQuaternion, Point3};
use kiss3d::window::Window;
use kiss3d::light::Light;
//use kiss3d::camera::FirstPerson;

mod cubeblock;
mod target;
mod eventhandler;

fn main() {
//    let eye = Point3::new(10.0f32, 10.0, 10.0);
//    let at = Point3::origin();
//    let mut first_person = FirstPerson::new(eye, at);

    let mut window = Window::new("p1mm3lTr0n");
    
    let mut maingroup = window.add_group();


    //let mut node = target::add_nodeGroup(&mut maingroup);
    let mut node = target::Node::init_node(&mut maingroup, 0,0,0,1);
  //  let mut cubes = cubeblock::add_cubeblock(&mut maingroup, 3);

    window.set_light(Light::StickToCamera);
    //let rotx = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), 0.014);

    while window.render() { 
//        window.render_with_camera(&mut first_person);
        eventhandler::move_event(&mut window, &mut node);
    }

}

