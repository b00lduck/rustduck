extern crate kiss3d;
extern crate nalgebra as na;

//use na::{Vector3, UnitQuaternion, Point3};
use kiss3d::window::Window;
use kiss3d::light::Light;
//use kiss3d::camera::FirstPerson;
use std::time::{SystemTime};
use kiss3d::text::Font;
use na::{Point2, Point3};

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

    let mut sobel = postproc::sobel::MySobel::new(5.0);

    //let mut node = target::add_nodeGroup(&mut maingroup);
    let mut node = target::Node::init_node(&mut maingroup, 0,0,0,1);

    playfield::add_playfield(&mut maingroup, &map);

    window.set_light(Light::StickToCamera);
    window.set_framerate_limit(Some(60));

    let mut frames_rendered = 0usize;
  
    let mut now = SystemTime::now();

    let font = Font::from_bytes(include_bytes!("./data/topaz.ttf")).unwrap_or(Font::default());

    while !window.should_close() {
        
        match now.elapsed() {
            Ok(elapsed) => {                
                let time_gone = elapsed.as_micros();
                now = SystemTime::now();
                // move(time_gone)
 
                //window.render_with_effect(&mut sobel);
                window.draw_text(
                    &format!("fps: {}", 1.0/(time_gone as f32/1000000.0)),
                    &Point2::new(30.0,30.0),
                    40.0,
                    &font,
                    &Point3::new(1.0, 1.0, 1.0),
                );
                frames_rendered = frames_rendered + 1;               
                window.draw_text(
                    &format!("frame: {}", frames_rendered),
                    &Point2::new(30.0,90.0),
                    40.0,
                    &font,
                    &Point3::new(1.0, 1.0, 1.0),
                );                
                window.render();
  
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }

    }
 

  
}

