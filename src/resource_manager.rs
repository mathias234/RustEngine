extern crate glium;
use model::Model;
pub struct ResourceContext {
    texture_resources: Vec<glium::texture::SrgbTexture2d>,
    shader_resources: Vec<glium::Program>,
    model_resources: Vec<Model>,
}

impl ResourceContext {
    pub fn new() -> ResourceContext {
        ResourceContext {
            texture_resources: Vec::new(),
            shader_resources: Vec::new(),
            model_resources: Vec::new(),
        }
    }

    pub fn alloc_tex(&mut self, tex: glium::texture::SrgbTexture2d) -> usize {
        self.texture_resources.push(tex);

        return self.texture_resources.len() - 1;
    }

    pub fn get_tex_ref(&self, id: usize) -> &glium::texture::SrgbTexture2d {
        return &self.texture_resources[id];
    }

    pub fn alloc_shader(&mut self, shader: glium::Program) -> usize {
        self.shader_resources.push(shader);

        return self.shader_resources.len() - 1;
    }

    pub fn get_shader_ref(&self, id: usize) -> &glium::Program {
        return &self.shader_resources[id];
    }

    pub fn alloc_model(&mut self, model: Model) -> usize {
        self.model_resources.push(model);

        return self.model_resources.len() - 1;
    }

    pub fn get_model_ref(&self, id: usize) -> &Model {
        return &self.model_resources[id];
    }
}
