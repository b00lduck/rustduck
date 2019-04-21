extern crate kiss3d;
extern crate nalgebra as na;

//use na::{Vector3, UnitQuaternion, Point3};
use kiss3d::window::Window;
use kiss3d::light::Light;
//use kiss3d::camera::FirstPerson;

mod cubeblock;
mod target;
mod eventhandler;

mod playfield;
mod map;

fn main() {

    let map = map::generator::Map::new();
   
    let mut window = Window::new("p1mm3lTr0n");
    
    let mut maingroup = window.add_group();

    //let mut node = target::add_nodeGroup(&mut maingroup);
    let mut node = target::Node::init_node(&mut maingroup, 0,0,-5,1);
    node.set_size(1.0);

    playfield::add_playfield(&mut maingroup, &map);

    window.set_light(Light::StickToCamera);

    let mut counter = 0.001_f32;

    while window.render() { 
        let sin_value = counter.sin();
        //println!("sin: {} | counter: {}", sin_value ,counter );
        node.set_size(sin_value.sin()*1.1);
        counter = counter + 0.2;

        eventhandler::move_event(&mut window, &mut node);
    }
  
}

