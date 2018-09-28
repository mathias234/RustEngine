// an immediate mode ui renderer
extern crate glium;
use glium::Surface;
use math_helper;
use shader;

pub enum UIType {
    Quad,
}

pub struct UIElement {
    ui_type: UIType,
    center_x: f32,
    center_y: f32,
    width: f32,
    height: f32,
}

pub struct UIContext {
    win_width: i32,
    win_height: i32,
    elements: Vec<UIElement>,
    program: glium::Program,
}

impl UIContext {
    pub fn new(display: &glium::Display, width: i32, height: i32) -> UIContext {
        UIContext {
            elements: Vec::new(),
            program: shader::load(&display, "res/ui_basic"),
            win_width: width,
            win_height: height,
        }
    }

    pub fn screen_resize(&mut self, win_width: i32, win_height: i32) {
        self.win_width = win_width;
        self.win_height = win_width;
    }

    pub fn render_quad(&mut self, center_x: f32, center_y: f32, width: f32, height: f32) {
        self.elements.push(UIElement {
            ui_type: UIType::Quad,
            center_x: center_x,
            center_y: center_y,
            width: width,
            height: height,
        })
    }

    pub fn draw_frame(&mut self, target: &mut glium::Frame, display: &glium::Display) {
        for i in 0..self.elements.len() {
            let element = &self.elements[i];
            match element.ui_type {
                UIType::Quad => draw_quad(self, element, target, display),
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct UIVertex {
    position: [f32; 3],
    texcoords: [f32; 2],
}

implement_vertex!(UIVertex, position, texcoords);

pub fn draw_quad(
    context: &UIContext,
    element: &UIElement,
    target: &mut glium::Frame,
    display: &glium::Display,
) {
    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        backface_culling: glium::BackfaceCullingMode::CullingDisabled,
        ..Default::default()
    };

    let center_x = element.center_x;
    let center_y = element.center_y;
    let width = element.width;
    let height = element.height;

    let quad_vertices = [
        UIVertex {
            position: [-width, -width, 0.0],
            texcoords: [0.0, 0.0],
        },
        UIVertex {
            position: [width, -width, 0.0],
            texcoords: [1.0, 0.0],
        },
        UIVertex {
            position: [width, width, 0.0],
            texcoords: [1.0, 1.0],
        },
        UIVertex {
            position: [-width, width, 0.0],
            texcoords: [0.0, 1.0],
        },
    ];

    let quad_indices: [u16; 6] = [0, 1, 2, 2, 3, 0];

    let vertex_buffer = glium::VertexBuffer::new(display, &quad_vertices).unwrap();
    let index_buffer = glium::IndexBuffer::new(
        display,
        glium::index::PrimitiveType::TrianglesList,
        &quad_indices,
    ).unwrap();

    let ortho_matrix = math_helper::ortho_matrix(
        0.0,
        context.win_width as f32,
        0.0,
        context.win_height as f32,
        -1.0,
        1.0,
    );

    let model_matrix = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];

    let uniforms = uniform! {
        ortho_matrix: ortho_matrix,
    };

    target
        .draw(
            &vertex_buffer,
            &index_buffer,
            &context.program,
            &uniforms,
            &params,
        ).unwrap();
}
