use kiss3d::scene::SceneNode;
use na::{Translation3, Vector3, UnitQuaternion, Point3};

pub struct NodeGroup {
    group: Vec<Node>
}

/* impl NodeGroup {
    pub fn add_node(&mut self, group: &mut SceneNode) {
        self.group.push(Node::init_node(group: &mut SceneNode, x: i32, y: i32, z: i32, r: i32))
    }
} */

pub struct Node {
    x: f32,
    y: f32,
    pub z: f32,
    r: f32,
    scenenode: SceneNode,

}

impl Node {

    pub fn init_node(group: &mut SceneNode, x:i32, y:i32, z:i32, r:i32) -> Node{
        let mut node = group.add_sphere(0.1f32);

        node.set_texture_from_memory(include_bytes!("./data/stone.png"), "stone");

        return Node{ 
            x:(x) as f32,
            y: (y) as f32,
            z:(z) as f32,
            r:(r) as f32,
            scenenode: node};
    }

    pub fn move_x(&mut self, new_x:f32) {
        self.x = self.x + new_x;
        self.set_translation();
    }
    pub fn move_y(&mut self, new_y:f32) {
        self.y = self.y + new_y;
        self.set_translation();
    }
    pub fn move_z(&mut self, new_z:f32) {
        self.z = self.z + new_z;
        self.set_translation();
    }

    pub fn rot_y(&mut self, new_y:f32) {
    let rotx = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), 0.014);
    self.scenenode.set_local_rotation(rotx);
    }

    fn set_translation(&mut self) {
        let t1 = Translation3::new(self.x, self.y, self.z);
        self.scenenode.set_local_translation(t1);
        println!("x: {} | y: {} | z: {}", self.x, self.y, self.z);
    }
}

/* pub fn changeDirection(scene_node: &mut Node, x: f32) {
    
    //let t1 = Translation3::new((x*1.0) as f32,0.0, 0.0);
    let rotx = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), 0.014);
    scene_node.scenenode.set_local_rotation(x);
 }
 */
pub fn add_sphereblock(scene_node: &mut SceneNode, a: i32) -> SceneNode {

    let mut g1 = scene_node.add_group();
    let a2 = a / 2;
    for z in 0..a {
        for x in 0..a {
            for y in 0..a {                
                add_sphere(&mut g1, x - a2, y - a2, z - a2);
            }
        }
    }

    return g1
}

fn add_sphere(scene_node: &mut SceneNode, x: i32, y: i32, z: i32) -> SceneNode {



    let mut c = scene_node.add_sphere(0.1f32);
    let t1 = Translation3::new((x*2) as f32, (y*2) as f32, (z*2) as f32);

    c.set_local_translation(t1);

    return c
}


