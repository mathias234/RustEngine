use resource_manager::*;

#[derive(Copy, Clone)]
pub struct Material {
	pub shader_prog: Resource,
	pub diffuse_tex: Resource,
	pub normal_tex: Resource,
	pub color: [f32; 3],
	pub tiling: [f32; 2],
}

impl Material {
	pub fn new(
		shader_prog: Resource,
		diffuse_tex: Resource,
		normal_tex: Resource,
		color: [f32; 3],
		tiling: [f32; 2],
	) -> Material {
		Material {
			shader_prog: shader_prog,
			diffuse_tex: diffuse_tex,
			normal_tex: normal_tex,
			color: color,
			tiling: tiling,
		}
	}
}
