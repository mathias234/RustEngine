#[macro_use]
extern crate glium;
extern crate colored;
extern crate nalgebra as na;
extern crate ncollide3d;
extern crate nphysics3d;
extern crate stopwatch;
extern crate tobj;

mod camera;
mod game;
mod gameobject;
mod material;
mod math_helper;
mod model;
mod physics_engine;
mod quaternion;
mod renderer;
mod resource_manager;
mod shader;
mod texture;
mod ui_renderer;
mod vector;

fn main() {
    // skip a few lines in the console
    println!("\n\n\n");

    let mut win_width = 1024;
    let mut win_height = 768;

    use glium::glutin;
    use stopwatch::Stopwatch;

    // Create A window
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Rust Engine")
        .with_dimensions(glutin::dpi::LogicalSize::new(
            win_width as f64,
            win_height as f64,
        ));

    let glutin_context = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(window, glutin_context, &events_loop).unwrap();

    let mut render_context = renderer::RenderContext::new(win_width, win_height);
    let mut resource_context = resource_manager::ResourceContext::new();
    let mut physics_context = physics_engine::PhysicsContext::new();
    let mut ui_context = ui_renderer::UIContext::new(&display, win_width as f32, win_height as f32);

    game::start(
        &display,
        &mut render_context,
        &mut resource_context,
        &mut physics_context,
    );

    let mut sw = Stopwatch::start_new();
    let mut delta_time: f64 = 0.0;

    // Game Loop
    let mut closed = false;
    while !closed {
        physics_context.step();
        game::update(&mut render_context, delta_time as f32);

        for i in 0..render_context.gameobjects.len() {
            resource_context
                .get_gameobject_ref_mut(render_context.gameobjects[i])
                .update(&mut physics_context);
        }

        let mut target = display.draw();

        renderer::render(&mut render_context, &mut resource_context, &mut target);
        game::render_gui(&mut ui_context);

        ui_context.draw_frame(&resource_context, &mut target, &display);

        target.finish().unwrap();

        events_loop.poll_events(|ev| match ev {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => closed = true,
                glutin::WindowEvent::Resized(logical_size) => {
                    render_context.resized(logical_size.width as i32, logical_size.height as i32);
                    win_width = logical_size.width as i32;
                    win_height = logical_size.height as i32;
                    ui_context.screen_resize(win_width as f32, win_height as f32);
                }
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
