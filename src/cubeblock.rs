use kiss3d::scene::SceneNode;
use na::Translation3;


pub fn add_cubeblock(scene_node: &mut SceneNode, a: i32) -> SceneNode {

    let mut g1 = scene_node.add_group();
    let a2 = a / 2;
    for z in 0..a {
        for x in 0..a {
            for y in 0..a {                
                add_cube(&mut g1, x - a2, y - a2, z - a2);
            }
        }
    }

    return g1
}

fn add_cube(scene_node: &mut SceneNode, x: i32, y: i32, z: i32) -> SceneNode {

    let mut c = scene_node.add_cube(1.0, 1.0, 1.0);
    let t1 = Translation3::new((x*2) as f32, (y*2) as f32, (z*2) as f32);

    c.set_local_translation(t1);

    return c
}