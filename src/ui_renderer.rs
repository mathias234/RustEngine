// an immediate mode ui renderer
extern crate glium;

use glium::Surface;
use math_helper;
use resource_manager::*;
use rusttype::{point, FontCollection, PositionedGlyph, Scale};
use shader;

pub struct UIContext {
    pub style: UIStyle,
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

enum UIType {
    Quad,
    Button,
    Text,
}

struct UIElement {
    ui_type: UIType,
    style: UIStyle,
    texture: Resource,
    center_x: f32,
    center_y: f32,
    width: f32,
    height: f32,

    // text spesific
    text: Vec<char>,
}

#[derive(Copy, Clone)]
pub struct UIStyle {
    quad_color: [f32; 4],
    font_color: [f32; 4],

    font_size: i32,
    font_resolution: i32,
}

#[allow(dead_code)]
impl UIContext {
    pub fn new(display: &glium::Display, width: f32, height: f32) -> UIContext {
        UIContext {
            style: UIStyle {
                quad_color: [1.0, 1.0, 1.0, 1.0],
                font_color: [1.0, 1.0, 1.0, 1.0],
                font_size: 14,
                font_resolution: 1,
            },
            elements: Vec::new(),
            program: shader::load(
                &display,
                include_bytes!("../res/ui_basic.vs"),
                include_bytes!("../res/ui_basic.fs"),
            ),
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

    pub fn set_quad_color(&mut self, color: [f32; 4]) {
        self.style.quad_color = color;
    }

    pub fn set_font_color(&mut self, color: [f32; 4]) {
        self.style.font_color = color;
    }

    pub fn set_font_size(&mut self, size: i32) {
        self.style.font_size = size;
    }

    pub fn set_font_res(&mut self, res: i32) {
        self.style.font_resolution = res;
    }

    pub fn screen_resize(&mut self, win_width: f32, win_height: f32) {
        self.win_width = win_width;
        self.win_height = win_height;
    }

    pub fn render_text(&mut self, text: &str, center_x: f32, center_y: f32) {
        let mut chars: Vec<char> = Vec::new();

        for ch in text.chars() {
            chars.push(ch);
        }

        self.elements.push(UIElement {
            style: self.style,
            ui_type: UIType::Text,
            texture: 0,
            center_x: center_x,
            center_y: center_y,
            width: 0.0,
            height: 0.0,
            // text spesific
            text: chars,
        })
    }

    pub fn render_quad(
        &mut self,
        texture: Resource,
        center_x: f32,
        center_y: f32,
        width: f32,
        height: f32,
    ) {
        self.elements.push(UIElement {
            style: self.style,
            ui_type: UIType::Quad,
            texture: texture,
            center_x: center_x,
            center_y: center_y,
            width: width,
            height: height,

            // text spesific
            text: Vec::new(),
        })
    }

    pub fn render_button(
        &mut self,
        texture: Resource,
        text: &str,
        center_x: f32,
        center_y: f32,
        width: f32,
        height: f32,
    ) -> bool {
        let mut chars: Vec<char> = Vec::new();

        for ch in text.chars() {
            chars.push(ch);
        }

        self.elements.push(UIElement {
            style: self.style,
            ui_type: UIType::Button,
            texture: texture,
            center_x: center_x,
            center_y: center_y,
            width: width,
            height: height,

            // text spesific
            text: chars,
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
        resources: &mut ResourceContext,
        target: &mut glium::Frame,
        display: &glium::Display,
    ) {
        // self.elements.reverse();
        for i in 0..self.elements.len() {
            let element = &self.elements[i];
            match element.ui_type {
                UIType::Quad => draw_quad(self, &element, resources, target, display, false),
                UIType::Button => draw_button(self, &element, resources, target, display),
                UIType::Text => draw_text(self, &element, resources, target, display),
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

fn draw_text(
    context: &UIContext,
    element: &UIElement,
    resources: &mut ResourceContext,
    target: &mut glium::Frame,
    display: &glium::Display,
) {
    let font_data = include_bytes!("../res/fonts/Arial.ttf");
    let collection = FontCollection::from_bytes(font_data as &[u8]).unwrap_or_else(|e| {
        panic!("Error constructing a Font Collection from bytes: {}", e);
    });

    let font = collection.into_font().unwrap_or_else(|e| {
        panic!("Error turning font collection into a font: {}", e);
    });

    let height: i32 = element.style.font_size * element.style.font_resolution;
    //let pixel_height = height.ceil() as usize;

    let scale = Scale::uniform(height as f32);

    let v_metrics = font.v_metrics(scale);
    let offset = point(0.0, v_metrics.ascent);

    println!("Offset: {}", offset.y);

    let mut text: String = "".to_owned();

    for i in 0..element.text.len() {
        text += &element.text[i].to_string();
    }

    let glyphs: Vec<PositionedGlyph> = font.layout(&text, scale, offset).collect();
    let width = glyphs
        .iter()
        .rev()
        .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
        .next()
        .unwrap_or(0.0)
        .ceil() as usize;

    let mut i = 0;

    for g in glyphs {
        if let Some(bb) = g.pixel_bounding_box() {
            let mut tex: Resource;

            if !resources
                .get_glyph(
                    element.text[i],
                    element.style.font_size,
                    element.style.font_resolution,
                )
                .is_some()
            {
                let mut pixels = vec![0.0 as f32; (bb.width() as usize * bb.height() as usize) * 4];

                g.draw(|x, y, v| {
                    let idx: usize = (x + y * bb.width() as u32) as usize * 4;
                    pixels[idx] = 1 as f32;
                    pixels[idx + 1] = 1 as f32;
                    pixels[idx + 2] = 1 as f32;
                    pixels[idx + 3] = v as f32;
                });

                let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(
                    &pixels,
                    (bb.width() as u32, bb.height() as u32),
                );

                let tex_srgb = glium::texture::SrgbTexture2d::new(display, raw_image);

                tex = resources.alloc_tex(tex_srgb.unwrap());

                resources.store_glyph(
                    element.text[i],
                    element.style.font_size,
                    element.style.font_resolution,
                    tex,
                );
            } else {
                tex = resources
                    .get_glyph(
                        element.text[i],
                        element.style.font_size,
                        element.style.font_resolution,
                    )
                    .unwrap();
            }

            let min_x = bb.min.x as f32;
            let min_y = -bb.min.y as f32;

            let max_x = bb.max.x as f32;
            let max_y = -bb.max.y as f32;

            let mut center_x = lerp(min_x, max_x, 0.5);
            let mut center_y = lerp(min_y, max_y, 0.5);

            // move text left so its centered in the middle not to the left side
            center_x = center_x - width as f32 / 2.0;

            // move text down so its centered in the middle of the text not the bottom
            center_y = center_y + (height as f32 / 2.0) as f32;

            center_x = center_x / element.style.font_resolution as f32;
            center_y = center_y / element.style.font_resolution as f32;

            let ui = UIElement {
                style: element.style,
                ui_type: UIType::Quad,
                texture: tex,
                center_x: element.center_x + center_x,
                center_y: element.center_y + center_y,
                width: (bb.width() as f32 / 2.0) / element.style.font_resolution as f32,
                height: (bb.height() as f32 / 2.0) / element.style.font_resolution as f32,

                text: Vec::new(),
            };

            draw_quad(context, &ui, resources, target, display, true);
        }
        i += 1;
    }
}

fn lerp(a: f32, b: f32, amt: f32) -> f32 {
    return a + amt * (b - a);
}

fn draw_button(
    context: &UIContext,
    element: &UIElement,
    resources: &mut ResourceContext,
    target: &mut glium::Frame,
    display: &glium::Display,
) {
    draw_quad(context, element, resources, target, display, false);
    draw_text(context, element, resources, target, display);
}

fn draw_quad(
    context: &UIContext,
    element: &UIElement,
    resources: &mut ResourceContext,
    target: &mut glium::Frame,
    display: &glium::Display,
    text_quad: bool,
) {
    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: false,
            ..Default::default()
        },
        backface_culling: glium::BackfaceCullingMode::CullingDisabled,
        blend: glium::draw_parameters::Blend::alpha_blending(),
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
    )
    .unwrap();

    let ortho_matrix = math_helper::ortho_matrix(
        0.0,
        context.win_width as f32,
        0.0,
        context.win_height as f32,
        -1.0,
        1.0,
    );

    let mut texture = resources.get_tex_ref(element.texture).sampled();

    if text_quad {
        texture = texture.wrap_function(glium::uniforms::SamplerWrapFunction::Clamp);
    }

    let uniforms = uniform! {
        ortho_matrix: ortho_matrix,
        ui_texture: texture,
        color: {
            if text_quad {
                element.style.font_color
            }
            else {
                element.style.quad_color
            }
        }
    };

    target
        .draw(
            &vertex_buffer,
            &index_buffer,
            &context.program,
            &uniforms,
            &params,
        )
        .unwrap();
}
