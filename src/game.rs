use renderer::{RenderContext, Vertex, Model};

pub fn start(context: &mut RenderContext) {
    context.clear_r = 0.0;
    context.clear_b = 0.7;
    context.clear_g = 0.3;

    let mut verts: Vec<Vertex> = Vec::new();
    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    verts.push(vertex1);
    verts.push(vertex2);
    verts.push(vertex3);

    context.models.push(Model { vertices: verts, indices: Vec::new() })
}

pub fn update(context: &mut RenderContext) {

}