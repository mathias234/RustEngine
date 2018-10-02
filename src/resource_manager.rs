extern crate glium;
use gameobject::GameObject;
use model::Model;
use std::collections::HashMap;

pub type Resource = usize;

pub struct ResourceContext {
    texture_resources: Vec<glium::texture::SrgbTexture2d>,
    shader_resources: Vec<glium::Program>,
    model_resources: Vec<Model>,
    gameobject_resources: Vec<GameObject>,

    glyph_storage: HashMap<char, Resource>,
}

#[allow(dead_code)]
impl ResourceContext {
    pub fn new() -> ResourceContext {
        ResourceContext {
            texture_resources: Vec::new(),
            shader_resources: Vec::new(),
            model_resources: Vec::new(),
            gameobject_resources: Vec::new(),
            glyph_storage: HashMap::new(),
        }
    }

    pub fn store_glyph(&mut self, character: char, texture: Resource) {
        self.glyph_storage.insert(character, texture);
    }

    pub fn get_glyph(&mut self, character: char) -> Option<Resource> {
        if self.glyph_storage.contains_key(&character) {
            let glyph = self.glyph_storage.get(&character).unwrap();
            return Some(*glyph);
        }

        return None;
    }

    pub fn alloc_tex(&mut self, tex: glium::texture::SrgbTexture2d) -> Resource {
        self.texture_resources.push(tex);

        return self.texture_resources.len() - 1;
    }

    pub fn alloc_shader(&mut self, shader: glium::Program) -> Resource {
        self.shader_resources.push(shader);

        return self.shader_resources.len() - 1;
    }

    pub fn alloc_model(&mut self, model: Model) -> Resource {
        self.model_resources.push(model);

        return self.model_resources.len() - 1;
    }

    pub fn alloc_gameobject(&mut self, gameobject: GameObject) -> Resource {
        self.gameobject_resources.push(gameobject);

        return self.gameobject_resources.len() - 1;
    }

    pub fn get_tex_ref_mut(&mut self, id: Resource) -> &mut glium::texture::SrgbTexture2d {
        return &mut self.texture_resources[id];
    }

    pub fn get_shader_ref_mut(&mut self, id: Resource) -> &mut glium::Program {
        return &mut self.shader_resources[id];
    }

    pub fn get_model_ref_mut(&mut self, id: Resource) -> &mut Model {
        return &mut self.model_resources[id];
    }

    pub fn get_gameobject_ref_mut(&mut self, id: Resource) -> &mut GameObject {
        return &mut self.gameobject_resources[id];
    }

    pub fn get_tex_ref(&self, id: Resource) -> &glium::texture::SrgbTexture2d {
        return &self.texture_resources[id];
    }

    pub fn get_shader_ref(&self, id: Resource) -> &glium::Program {
        return &self.shader_resources[id];
    }

    pub fn get_model_ref(&self, id: Resource) -> &Model {
        return &self.model_resources[id];
    }

    pub fn get_gameobject_ref(&self, id: Resource) -> &GameObject {
        return &self.gameobject_resources[id];
    }
}
