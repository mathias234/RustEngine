extern crate glium;
extern crate tobj;
use camera::*;
use resource_manager::ResourceContext;

pub struct RenderContext {
    pub clear_color: [f32; 3],
    pub gameobjects: Vec<usize>,
    pub camera: CameraState,
}

#[allow(dead_code)]
impl RenderContext {
    pub fn new(win_width: i32, win_height: i32) -> RenderContext {
        RenderContext {
            camera: CameraState::new(win_width, win_height),
            clear_color: [0.0, 0.0, 0.0],
            gameobjects: Vec::new(),
        }
    }

    pub fn get_gameobject(&mut self, resources: &mut ResourceContext, name: String) -> usize {
        for i in 0..self.gameobjects.len() {
            if resources.get_gameobject_ref(self.gameobjects[i]).name == name {
                return self.gameobjects[i];
            }
        }

        self.gameobjects[0]
    }

    pub fn resized(&mut self, width: i32, height: i32) {
        self.camera.resize(width, height);
    }
}

pub fn render(
    context: &mut RenderContext,
    resources: &mut ResourceContext,
    target: &mut glium::Frame,
) {
    use glium::Surface;

    target.clear_color_and_depth(
        (
            context.clear_color[0],
            context.clear_color[1],
            context.clear_color[2],
            1.0,
        ),
        1.0,
    );

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        backface_culling: glium::BackfaceCullingMode::CullCounterClockwise,
        ..Default::default()
    };

    let pers_mat = context.camera.get_perspective();
    let view_mat = context.camera.get_view();

    for i in 0..context.gameobjects.len() {
        let gobj = resources.get_gameobject_ref(context.gameobjects[i]);
        let material = gobj.material;
        let program = resources.get_shader_ref(material.shader_prog);

        let model = resources.get_model_ref(gobj.model);

        let model_matrix = gobj.get_model_matrix();

        let uniforms = uniform! {
            persp_matrix: pers_mat,
            view_matrix: view_mat,
            model_matrix: model_matrix,
            view_pos: context.camera.position.raw(),
            light_dir: (-0.5, -1.0, 0.0f32),
            ambient_light: 0.4 as f32,
            diffuse: resources.get_tex_ref(material.diffuse_tex),
            normal_map: resources.get_tex_ref(material.normal_tex),
            color: material.color,
            tiling: material.tiling,
        };

        target
            .draw(
                &model.vertex_buffer,
                &model.index_buffer,
                &program,
                &uniforms,
                &params,
            ).unwrap();
    }
}
