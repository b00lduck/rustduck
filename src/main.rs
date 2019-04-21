extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::post_processing::SobelEdgeHighlight;
//use na::{Vector3, UnitQuaternion, Point3};
use kiss3d::window::Window;
use kiss3d::light::Light;
//use kiss3d::camera::FirstPerson;
use std::time::{SystemTime, UNIX_EPOCH};

mod cubeblock;
mod target;
mod eventhandler;

mod playfield;
mod map;

fn main() {

    let map = map::generator::Map::new();
   
    let mut window = Window::new("p1mm3lTr0n");
    
    let mut maingroup = window.add_group();

    let mut sobel = SobelEdgeHighlight::new(4.0);

    //let mut node = target::add_nodeGroup(&mut maingroup);
    let mut node = target::Node::init_node(&mut maingroup, 0,0,0,1);

    playfield::add_playfield(&mut maingroup, &map);

    window.set_light(Light::StickToCamera);
    window.set_framerate_limit(Some(60));

    let mut frames_rendered = 0usize;
  
    let mut now = SystemTime::now();

    while !window.should_close() {
        
        match now.elapsed() {
            Ok(elapsed) => {                
                let time_gone = elapsed.as_micros();
                println!("{}", time_gone);
                now = SystemTime::now();
                // move(time_gone)
                window.render_with_effect(&mut sobel);
                frames_rendered = frames_rendered + 1;               
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }

    }
 

  
}

