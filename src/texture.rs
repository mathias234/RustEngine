extern crate glium;
extern crate image;

pub fn load(display: &glium::Display, filename: String) -> glium::texture::SrgbTexture2d {
    println!("Loading texture: {}", filename);

    let image = image::open(filename).unwrap().to_rgba();

    let image_dimensions = image.dimensions();

    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    glium::texture::SrgbTexture2d::new(display, image).unwrap()
}
