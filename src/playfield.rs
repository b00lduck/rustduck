extern crate rand;

use kiss3d::scene::SceneNode;
use na::Translation3;


pub fn add_playfield(scene_node: &mut SceneNode) -> SceneNode {

    let size = 32;

    let size2 = size / 2;

    let mut g1 = scene_node.add_group();
    for x in 0..64 {
        for y in 0..64 {                      
                if rand::random() {
                    add_cube(&mut g1, x - size2, y - size2, -4);
                }                
        }
    }    

    return g1
}

fn add_cube(scene_node: &mut SceneNode, x: i32, y: i32, z: i32) -> SceneNode {

    let mut c = scene_node.add_cube(1.0, 1.0, 1.0);
    let t1 = Translation3::new((x) as f32, (y) as f32, (z) as f32);

    c.set_local_translation(t1);

    return c
}