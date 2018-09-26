#[derive(Copy, Clone)]
pub struct Material {
	pub diffuse_tex: usize,
	pub normal_tex: usize,
	pub shader_prog: usize,
}

impl Material {
	pub fn new(diffuse_tex: usize, normal_tex: usize, shader_prog: usize) -> Material {
		Material {
			diffuse_tex: diffuse_tex,
			normal_tex: normal_tex,
			shader_prog: shader_prog,
		}
	}
}