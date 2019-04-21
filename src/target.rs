use kiss3d::scene::SceneNode;
//use na::{Translation3, Vector3, UnitQuaternion};
use na::{Translation3};

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
    z_angle: f32,
    size: f32,
    speed: f32,
    scenenode: SceneNode,
    pulse: f32,
}

impl Node {

    pub fn init_node(group: &mut SceneNode, x:i32, y:i32, z:i32, size:f32) -> Node{
        let mut scene_node = group.add_sphere(0.1f32);

        scene_node.set_texture_from_memory(include_bytes!("./data/stone.png"), "stone");

        let mut node = Node{ 
            x:(x) as f32,
            y: (y) as f32,
            z:(z) as f32,
            z_angle: 0.0,
            size: size,
            speed: 0.0,
            scenenode: scene_node,
            pulse: 0.1};
            node.set_translation();
            return node
    }

    pub fn move_one_step(&mut self) {
        let new_x = self.x + self.z_angle.sin()*self.speed;
        let new_y = self.y + self.z_angle.cos()*self.speed;
        self.x = new_x;
        self.y = new_y;
        self.set_translation();
    }

    pub fn inc_speed(&mut self, add_speed:f32) {
        self.speed = self.speed + add_speed;
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

    pub fn pulse(&mut self, step: f32) {
        self.pulse = self.pulse + step*2.0;


        self.size = self.pulse.sin().sin() * 0.5 + 2.0; 
        self.scenenode.set_local_scale(self.size,self.size,self.size);
    }
    fn set_translation(&mut self) {
        let t1 = Translation3::new(self.x, self.y, self.z);
        self.scenenode.set_local_translation(t1);
        //println!("x: {} | y: {} | z: {}", self.x, self.y, self.z);
    }
}


