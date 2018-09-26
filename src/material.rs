#[derive(Copy, Clone)]
pub struct Material {
	pub shader_prog: usize,
	pub diffuse_tex: usize,
	pub normal_tex: usize,
	pub color: [f32; 3],
	pub tiling: [f32; 2],
}

impl Material {
	pub fn new(
		shader_prog: usize,
		diffuse_tex: usize,
		normal_tex: usize,
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
