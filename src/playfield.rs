//extern crate rand;

use kiss3d::scene::SceneNode;
use na::Translation3;

use map::generator::Map;

pub fn add_playfield(scene_node: &mut SceneNode, map: &Map) -> SceneNode {

    let mut g1 = scene_node.add_group();
    for x in 0..map.size.x {
        for y in 0..map.size.y {                      
            if map.state[y as usize][x as usize] == 0 {
                add_cube(&mut g1, x - map.size.x/2, y - map.size.y/2, -4);
            }
        }
    }    

    return g1
}

fn add_cube(scene_node: &mut SceneNode, x: i16, y: i16, z: i16) -> SceneNode {

    let mut c = scene_node.add_cube(1.0, 1.0, 1.0);
    let t1 = Translation3::new((x) as f32, (y) as f32, (z) as f32);

    c.set_local_translation(t1);

    return c
}