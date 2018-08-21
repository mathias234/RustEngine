extern crate glium;
extern crate tobj;
use camera::*;
use resource_manager::ResourceContext;

pub struct RenderContext {
    pub clear_r: f32,
    pub clear_g: f32,
    pub clear_b: f32,
    pub gameobjects: Vec<usize>,
    pub camera: CameraState,
}

#[allow(dead_code)]
impl RenderContext {
    pub fn new() -> RenderContext {
        RenderContext {
            camera: CameraState::new(1024, 768),
            clear_r: 0.0,
            clear_g: 0.0,
            clear_b: 0.0,
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
}

pub fn init_renderer() -> RenderContext {
    RenderContext::new()
}

pub fn update_renderer(
    context: &mut RenderContext,
    resources: &mut ResourceContext,
    target: &mut glium::Frame,
) {
    use glium::Surface;

    target.clear_color_and_depth(
        (context.clear_r, context.clear_g, context.clear_b, 1.0),
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
        let program = resources.get_shader_ref(gobj.shader_program);

        let model = resources.get_model_ref(gobj.model);

        let model_matrix = gobj.get_model_matrix();

        let uniforms = uniform! {
            persp_matrix: pers_mat,
            view_matrix: view_mat,
            model_matrix: model_matrix,
            view_pos: context.camera.position.raw(),
            light_dir: (-0.5, -1.0, 0.0f32),
            ambient_light: 0.4 as f32,
            diffuse: resources.get_tex_ref(gobj.texture),
            normal_map: resources.get_tex_ref(gobj.normal_map),
        };

        target
            .draw(
                &model.vertex_buffer,
                &model.index_buffer,
                &program,
                &uniforms,
                &params,
            )
            .unwrap();
    }
}
