#[macro_use]
extern crate glium;
extern crate stopwatch;
extern crate tobj;

mod camera;
mod game;
mod gameobject;
mod math_helper;
mod model;
mod quaternion;
mod renderer;
mod shader;
mod texture;
mod vector;

fn main() {
    use glium::glutin;
    use stopwatch::Stopwatch;

    // Create A window
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Rust Engine")
        .with_dimensions(glutin::dpi::LogicalSize::new(1024.0, 768.0));
    let glutin_context = glutin::ContextBuilder::new().with_vsync(false);
    let display = glium::Display::new(window, glutin_context, &events_loop).unwrap();

    let mut render_context = renderer::init_renderer();

    game::start(&display, &mut render_context);

    let mut sw = Stopwatch::start_new();
    let mut delta_time: f64 = 0.0;

    // Game Loop
    let mut closed = false;
    while !closed {
        game::update(&mut render_context, delta_time as f32);

        let mut target = display.draw();

        renderer::update_renderer(&mut render_context, &mut target);

        target.finish().unwrap();

        events_loop.poll_events(|ev| match ev {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => closed = true,
                ev => game::process_input(&mut render_context, &ev),
            },
            glutin::Event::DeviceEvent { event, .. } => match event {
                ev => game::process_input_device(&mut render_context, &ev),
            },
            _ => (),
        });

        // get the elapsed time as nano secounds in case this frame happened very fast
        let elapsed_time = sw.elapsed().subsec_nanos();
        delta_time = elapsed_time as f64 / 1_000_000_000.0;
        sw.restart();
    }
}
