extern crate glium;
extern crate image;
use stopwatch::Stopwatch;

pub fn load(display: &glium::Display, filename: String) -> glium::texture::SrgbTexture2d {
	println!("Loading texture: {}", filename);
	let sw = Stopwatch::start_new();

	let image = image::open(filename).unwrap().to_rgba();

	let image_dimensions = image.dimensions();

	let image =
		glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

	println!("Image file loaded, took {}ms\n", sw.elapsed_ms());

	glium::texture::SrgbTexture2d::new(display, image).unwrap()
}
