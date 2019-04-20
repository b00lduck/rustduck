use kiss3d::resource::{Effect, Material, Mesh, ShaderAttribute, ShaderUniform};
use kiss3d::scene::SceneNode;
use na::Translation3;
use na::{Isometry3, Matrix3, Matrix4, Point3, Vector3};
use std::cell::RefCell;
use std::rc::Rc;
use kiss3d::camera::Camera;
use kiss3d::context::Context;
use kiss3d::light::Light;
use kiss3d::scene::ObjectData;

pub fn add_cubeblock(scene_node: &mut SceneNode, a: i32) -> SceneNode {

    let material = Rc::new(RefCell::new(
        Box::new(NormalMaterial::new()) as Box<Material + 'static>
    ));

    let mut g1 = scene_node.add_group();
    let a2 = a / 2;
    for z in 0..a {
        for x in 0..a {
            for y in 0..a {
                add_cube(&mut g1, 0, x - a2, y - a2, z - a2);
                add_cube(&mut g1, 1, x - a2, y - a2, z - a2);
            }
        }
    }

    g1.set_material(material);

    return g1
}

fn add_cube(scene_node: &mut SceneNode, n: i32, x: i32, y: i32, z: i32) -> SceneNode {

    let mut c = scene_node.add_cube(1.0, 1.0, 1.0);
    let t1 = Translation3::new((x*2) as f32, (y*2) as f32, (z*2) as f32);
    //let t2 = Translation3::new(-4.5f32, -0.5f32, -0.5f32);

    if n == 0 {
        c.set_color(0.0, 1.0, 0.5);
        c.set_points_size(5.0);
        c.set_lines_width(1.0);    
        c.set_surface_rendering_activation(false);
    } else {
        c.set_color(0.7, 0.2, 0.1);
    }

    c.set_local_translation(t1);
    //c.append_translation(&t2);

    return c
}


// A material that draws normals
pub struct NormalMaterial {
    shader: Effect,
    position: ShaderAttribute<Point3<f32>>,
    normal: ShaderAttribute<Vector3<f32>>,
    view: ShaderUniform<Matrix4<f32>>,
    proj: ShaderUniform<Matrix4<f32>>,
    transform: ShaderUniform<Matrix4<f32>>,
    scale: ShaderUniform<Matrix3<f32>>,
}

impl NormalMaterial {
    pub fn new() -> NormalMaterial {
        let mut shader = Effect::new_from_str(NORMAL_VERTEX_SRC, NORMAL_FRAGMENT_SRC);

        shader.use_program();

        NormalMaterial {
            position: shader.get_attrib("position").unwrap(),
            normal: shader.get_attrib("normal").unwrap(),
            transform: shader.get_uniform("transform").unwrap(),
            scale: shader.get_uniform("scale").unwrap(),
            view: shader.get_uniform("view").unwrap(),
            proj: shader.get_uniform("proj").unwrap(),
            shader: shader,
        }
    }
}

impl Material for NormalMaterial {
    fn render(
        &mut self,
        pass: usize,
        transform: &Isometry3<f32>,
        scale: &Vector3<f32>,
        camera: &mut Camera,
        _: &Light,
        _: &ObjectData,
        mesh: &mut Mesh,
    ) {
        self.shader.use_program();
        self.position.enable();
        self.normal.enable();

        /*
         *
         * Setup camera and light.
         *
         */
        camera.upload(pass, &mut self.proj, &mut self.view);

        /*
         *
         * Setup object-related stuffs.
         *
         */
        let formated_transform = transform.to_homogeneous();
        let formated_scale = Matrix3::from_diagonal(&Vector3::new(scale.x, scale.y, scale.z));

        self.transform.upload(&formated_transform);
        self.scale.upload(&formated_scale);

        mesh.bind_coords(&mut self.position);
        mesh.bind_normals(&mut self.normal);
        mesh.bind_faces();

        Context::get().draw_elements(
            Context::TRIANGLES,
            mesh.num_pts() as i32,
            Context::UNSIGNED_SHORT,
            0,
        );

        mesh.unbind();

        self.position.disable();
        self.normal.disable();
    }
}

static NORMAL_VERTEX_SRC: &'static str = "#version 100
attribute vec3 position;
attribute vec3 normal;
uniform mat4 view;
uniform mat4 proj;
uniform mat4 transform;
uniform mat3 scale;
varying vec3 ls_normal;
void main() {
    ls_normal   = normal;
    gl_Position = proj * view * transform * mat4(scale) * vec4(position, 1.0);
}
";

static NORMAL_FRAGMENT_SRC: &'static str = "#version 100
#ifdef GL_FRAGMENT_PRECISION_HIGH
   precision highp float;
#else
   precision mediump float;
#endif
varying vec3 ls_normal;
void main() {
    gl_FragColor = vec4((ls_normal + 1.0) / 2.0, 1.0);
}
";