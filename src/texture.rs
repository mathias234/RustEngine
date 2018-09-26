extern crate glium;
extern crate image;
use colored::*;
use stopwatch::Stopwatch;

pub fn load(display: &glium::Display, filename: String) -> glium::texture::SrgbTexture2d {
	println!("Loading texture: {}", filename);
	let sw = Stopwatch::start_new();

	let image = image::open(filename);

	if !image.is_ok() {
		println!("{}", "Failed to load texture".red());
	}

	let image = image.unwrap().to_rgba();

	let image_dimensions = image.dimensions();

	let image =
		glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

	println!(
		"{}{}{}",
		"Image file loaded, took ".green(),
		sw.elapsed_ms().to_string().green(),
		"ms \n".green()
	);

	glium::texture::SrgbTexture2d::new(display, image).unwrap()
}
