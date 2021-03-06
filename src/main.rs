extern crate kiss3d;
extern crate nalgebra as na;

//use na::{Vector3, UnitQuaternion, Point3};
use kiss3d::window::Window;
use kiss3d::light::Light;
//use kiss3d::camera::FirstPerson;
use std::time::{SystemTime};

mod cubeblock;
mod target;
mod eventhandler;

mod playfield;
mod map;
mod postproc;

fn main() {

    let map = map::generator::Map::new();
   
    let mut window = Window::new("p1mm3lTr0n");
    
    let mut maingroup = window.add_group();

    let mut sobel = postproc::sobel::MySobel::new(10.0);

    //let mut node = target::add_nodeGroup(&mut maingroup);
    let mut node = target::Node::init_node(&mut maingroup, 0,0,-5,1.0);

    playfield::add_playfield(&mut maingroup, &map);

    window.set_light(Light::StickToCamera);
    window.set_framerate_limit(Some(60));

    let mut frames_rendered = 0usize;
  
    let mut now = SystemTime::now();

    while !window.should_close() {
        
        match now.elapsed() {
            Ok(elapsed) => {                
                let time_gone = elapsed.as_micros();
                now = SystemTime::now();
                move_step(time_gone, &mut node);
                window.render_with_effect(&mut sobel);
                frames_rendered = frames_rendered + 1;               
                eventhandler::move_event(&mut window, &mut node);
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }

    }


 

  
}

fn move_step(time_gone:u128,  node:&mut target::Node) {
    let step = time_gone as f32 /1000000.0;
        //println!("Time gone: {} | step: {}", time_gone, step);
        node.pulse(step*1.0);
        node.move_one_step();

}
