extern crate glium;

use std::fs::File;
use std::io::prelude::*;

pub fn read_shader_file(display: &glium::Display, filename: &str) -> glium::Program {
    let vs_shader = filename.to_owned() + ".vs";
    let fs_shader = filename.to_owned() + ".fs";

    let mut vertex_shader_src = String::new();
    let mut vs_file_reader = File::open(vs_shader).unwrap();
    vs_file_reader.read_to_string(&mut vertex_shader_src).unwrap();


    let mut fragment_shader_src = String::new();
    let mut fs_file_reader = File::open(fs_shader).unwrap();
    fs_file_reader.read_to_string(&mut fragment_shader_src).unwrap();

    return glium::Program::from_source(display, &vertex_shader_src, &fragment_shader_src, None).unwrap();
}