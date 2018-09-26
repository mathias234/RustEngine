extern crate glium;

use std::fs::File;
use std::io::prelude::*;
use stopwatch::Stopwatch;

pub fn load(display: &glium::Display, filename: &str) -> glium::Program {
    println!("Loading shader: {}", filename);
    let sw = Stopwatch::start_new();

    let vs_shader = filename.to_owned() + ".vs";
    let fs_shader = filename.to_owned() + ".fs";

    let mut vertex_shader_src = String::new();
    let mut vs_file_reader = File::open(vs_shader).unwrap();
    vs_file_reader
        .read_to_string(&mut vertex_shader_src)
        .unwrap();

    let mut fragment_shader_src = String::new();
    let mut fs_file_reader = File::open(fs_shader).unwrap();
    fs_file_reader
        .read_to_string(&mut fragment_shader_src)
        .unwrap();

    let program =
        glium::Program::from_source(display, &vertex_shader_src, &fragment_shader_src, None)
            .unwrap();

    println!("Shader file loaded, took {}ms", sw.elapsed_ms());

    return program;
}
