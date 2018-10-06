extern crate glium;

use std::str;

pub fn load(display: &glium::Display, vs: &[u8], fs: &[u8]) -> glium::Program {
    let program = glium::Program::from_source(
        display,
        &str::from_utf8(vs).unwrap(),
        &str::from_utf8(fs).unwrap(),
        None,
    ).unwrap();

    return program;
}
