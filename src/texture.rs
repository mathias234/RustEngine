extern crate glium;
extern crate image;
use colored::*;

pub fn load(display: &glium::Display, buffer: &[u8]) -> glium::texture::SrgbTexture2d {
	let image = image::load_from_memory(buffer);

	if !image.is_ok() {
		println!("{}", "Failed to load texture".red());
	}

	let image = image.unwrap().to_rgba();

	let image_dimensions = image.dimensions();

	let image =
		glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

	glium::texture::SrgbTexture2d::new(display, image).unwrap()
}
