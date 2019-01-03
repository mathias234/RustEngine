extern crate glium;
extern crate tobj;
use assets;
use camera::*;
use resource_manager::*;
use shader;

pub struct RenderContext {
    pub clear_color: [f32; 3],
    pub gameobjects: Vec<Resource>,
    pub camera: CameraState,

    picking_attachments: Option<(
        glium::texture::UnsignedTexture2d,
        glium::framebuffer::DepthRenderBuffer,
    )>,
    picking_pbo: glium::texture::pixel_buffer::PixelBuffer<u32>,

    picking_program: glium::Program,
}

#[allow(dead_code)]
impl RenderContext {
    pub fn new(win_width: i32, win_height: i32, display: &glium::Display) -> RenderContext {
        let mut picking_attachments: Option<(
            glium::texture::UnsignedTexture2d,
            glium::framebuffer::DepthRenderBuffer,
        )> = None;

        let picking_pbo: glium::texture::pixel_buffer::PixelBuffer<u32> =
            glium::texture::pixel_buffer::PixelBuffer::new_empty(display, 1);

        let picking_program = shader::load(
            &display,
            &assets::get_asset("./res/picking.vs"),
            &assets::get_asset("./res/picking.fs"),
        );

        RenderContext {
            camera: CameraState::new(win_width, win_height),
            clear_color: [0.0, 0.0, 0.0],
            gameobjects: Vec::new(),

            picking_attachments: picking_attachments,
            picking_pbo: picking_pbo,
            picking_program: picking_program,
        }
    }

    pub fn get_gameobject(&mut self, resources: &mut ResourceContext, name: String) -> Resource {
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
    display: &glium::Display,
    cursor_position: Option<(i32, i32)>,
) {
    use glium::Surface;

    let picked_object = { context.picking_pbo.read().map(|d| d[0]).unwrap_or(0) };

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

    if context.picking_attachments.is_none()
        || (
            context.picking_attachments.as_ref().unwrap().0.get_width(),
            context
                .picking_attachments
                .as_ref()
                .unwrap()
                .0
                .get_height()
                .unwrap(),
        ) != target.get_dimensions()
    {
        let (width, height) = target.get_dimensions();
        context.picking_attachments = Some((
            glium::texture::UnsignedTexture2d::empty_with_format(
                display,
                glium::texture::UncompressedUintFormat::U32,
                glium::texture::MipmapsOption::NoMipmap,
                width,
                height,
            )
            .unwrap(),
            glium::framebuffer::DepthRenderBuffer::new(
                display,
                glium::texture::DepthFormat::F32,
                width,
                height,
            )
            .unwrap(),
        ))
    }

    //clearing the picking texture
    if let Some((ref picking_texture, ref _depth_buffer)) = context.picking_attachments {
        picking_texture
            .main_level()
            .first_layer()
            .into_image(None)
            .unwrap()
            .raw_clear_buffer([0u32, 0, 0, 0]);
    }

    for i in 0..context.gameobjects.len() {
        let gobj = resources.get_gameobject_ref(context.gameobjects[i]);
        let mut material = gobj.material;
        let program = resources.get_shader_ref(material.shader_prog);

        let model = resources.get_model_ref(gobj.model);

        let model_matrix = gobj.get_model_matrix();

        if picked_object - 1 == i as u32 {
            material.color = [0.0, 1.0, 0.0];
        }

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

        // add 1 to id so we know that 0 is nothing
        let picking_uniform = uniform! {
            persp_matrix: pers_mat,
            view_matrix: view_mat,
            model_matrix: model_matrix,
            id: (i + 1) as u32
        };

        if let Some((ref picking_texture, ref depth_buffer)) = context.picking_attachments {
            let mut picking_target = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(
                display,
                picking_texture,
                depth_buffer,
            )
            .unwrap();
            picking_target.clear_depth(1.0);

            picking_target
                .draw(
                    &model.vertex_buffer,
                    &model.index_buffer,
                    &context.picking_program,
                    &picking_uniform,
                    &params,
                )
                .unwrap();
        }

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

    if let (Some(cursor), Some(&(ref picking_texture, _))) =
        (cursor_position, context.picking_attachments.as_ref())
    {
        let read_target = glium::Rect {
            left: (cursor.0 - 1) as u32,
            bottom: picking_texture.get_height().unwrap() - std::cmp::max(cursor.1 - 1, 0) as u32,
            width: 1,
            height: 1,
        };

        if read_target.left < picking_texture.get_width()
            && read_target.bottom < picking_texture.get_height().unwrap()
        {
            picking_texture
                .main_level()
                .first_layer()
                .into_image(None)
                .unwrap()
                .raw_read_to_pixel_buffer(&read_target, &context.picking_pbo);
        } else {
            context.picking_pbo.write(&[0]);
        }
    } else {
        context.picking_pbo.write(&[0]);
    }
}
