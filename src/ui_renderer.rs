// an immediate mode ui renderer
extern crate glium;
use glium::Surface;
use math_helper;
use resource_manager::*;
use shader;

pub enum UIType {
    Quad,
    Button,
}

pub struct UIElement {
    ui_type: UIType,
    texture: usize,
    center_x: f32,
    center_y: f32,
    width: f32,
    height: f32,
}

pub struct UIContext {
    pub win_width: f32,
    pub win_height: f32,

    pub left_mouse_down: bool,
    pub right_mouse_down: bool,

    pub last_left_state: bool,
    pub last_right_state: bool,

    pub mouse_x: f32,
    pub mouse_y: f32,

    elements: Vec<UIElement>,
    program: glium::Program,
}

#[allow(dead_code)]
impl UIContext {
    pub fn new(display: &glium::Display, width: f32, height: f32) -> UIContext {
        UIContext {
            elements: Vec::new(),
            program: shader::load(&display, "res/ui_basic"),
            win_width: width,
            win_height: height,
            left_mouse_down: false,
            right_mouse_down: false,
            last_left_state: false,
            last_right_state: false,
            mouse_x: 0.0,
            mouse_y: 0.0,
        }
    }

    pub fn screen_resize(&mut self, win_width: f32, win_height: f32) {
        self.win_width = win_width;
        self.win_height = win_height;
    }

    pub fn render_quad(
        &mut self,
        texture: usize,
        center_x: f32,
        center_y: f32,
        width: f32,
        height: f32,
    ) {
        self.elements.push(UIElement {
            ui_type: UIType::Quad,
            texture: texture,
            center_x: center_x,
            center_y: center_y,
            width: width,
            height: height,
        })
    }

    pub fn render_button(
        &mut self,
        texture: usize,
        center_x: f32,
        center_y: f32,
        width: f32,
        height: f32,
    ) -> bool {
        self.elements.push(UIElement {
            ui_type: UIType::Button,
            texture: texture,
            center_x: center_x,
            center_y: center_y,
            width: width,
            height: height,
        });

        let mouse_x = self.mouse_x;
        let mouse_y = (self.mouse_y - self.win_height).abs(); // flip y since the window is top left, but the ui is bottom left

        let mut result = false;

        if self.left_mouse_down && !self.last_left_state {
            // check if we are inside the box
            if mouse_x > -width + center_x
                && mouse_x < width + center_x
                && mouse_y > -height + center_y
                && mouse_y < height + center_y
            {
                result = true;
            }
        }

        self.last_left_state = self.left_mouse_down;

        result
    }

    pub fn draw_frame(
        &mut self,
        resources: &ResourceContext,
        target: &mut glium::Frame,
        display: &glium::Display,
    ) {
        self.elements.reverse();
        for i in 0..self.elements.len() {
            let element = &self.elements[i];
            match element.ui_type {
                UIType::Quad => draw_quad(self, element, resources, target, display),
                UIType::Button => draw_button(self, element, resources, target, display),
            }
        }

        self.elements.clear();
    }
}

#[derive(Copy, Clone)]
pub struct UIVertex {
    position: [f32; 3],
    texcoords: [f32; 2],
}

implement_vertex!(UIVertex, position, texcoords);

fn draw_button(
    context: &UIContext,
    element: &UIElement,
    resources: &ResourceContext,
    target: &mut glium::Frame,
    display: &glium::Display,
) {
    draw_quad(context, element, resources, target, display);
}

fn draw_quad(
    context: &UIContext,
    element: &UIElement,
    resources: &ResourceContext,
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
            position: [-width + center_x, -height + center_y, 0.0],
            texcoords: [0.0, 0.0],
        },
        UIVertex {
            position: [width + center_x, -height + center_y, 0.0],
            texcoords: [1.0, 0.0],
        },
        UIVertex {
            position: [width + center_x, height + center_y, 0.0],
            texcoords: [1.0, 1.0],
        },
        UIVertex {
            position: [-width + center_x, height + center_y, 0.0],
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

    let texture = resources.get_tex_ref(element.texture);

    let uniforms = uniform! {
        ortho_matrix: ortho_matrix,
        ui_texture: texture,
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
