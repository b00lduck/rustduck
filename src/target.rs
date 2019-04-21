use kiss3d::scene::SceneNode;
use na::{Translation3, Vector3, UnitQuaternion};

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
    z_angle: f32,
    scenenode: SceneNode,

}

impl Node {

    pub fn init_node(group: &mut SceneNode, x:i32, y:i32, z:i32, r:i32) -> Node{
        let mut node = Node{ x:(x) as f32,y: (y) as f32,z:(z) as f32,r:(r) as f32,z_angle: 0.0,scenenode: group.add_sphere(0.1f32)};
        node.scenenode.set_color(1.0,0.0,0.0);
        node.set_translation();
        return node
    }

    pub fn move_one_step(&mut self,step:f32) {
        let new_x = self.x + self.z_angle.sin()*step;
        let new_y = self.y + self.z_angle.cos()*step;
        self.x = new_x;
        self.y = new_y;
        self.set_translation();
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

    pub fn inc_z_angle(&mut self, step:f32) {
        self.z_angle = self.z_angle + step;
    }

    pub fn rot_z(&mut self, new_angle:f32) {
        self.z_angle = self.z_angle + new_angle;

        // Um das Objekt zu drehen
        //let rot = UnitQuaternion::from_axis_angle(&Vector3::z_axis(), 0.014);
        //self.scenenode.set_local_rotation(rotx);
    }

    pub fn set_size(&mut self, new_size: f32) {
        self.scenenode.set_local_scale(new_size, new_size,new_size)
    }
    fn set_translation(&mut self) {
        let t1 = Translation3::new(self.x, self.y, self.z);
        self.scenenode.set_local_translation(t1);
        println!("x: {} | y: {} | z: {}", self.x, self.y, self.z);
    }
}


