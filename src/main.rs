#[macro_use]
extern crate glium;
extern crate binary_rw;
extern crate colored;
extern crate nalgebra as na;
extern crate ncollide3d;
extern crate nphysics3d;
extern crate rusttype;
extern crate stopwatch;
extern crate tobj;

mod assets;
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
    args_parser();
}

fn args_parser() {
    let mut should_start_game = true;
    let mut compile_assets = false;

    let args = std::env::args();
    println!("Args: ");
    for argument in args {
        println!("  {}", argument);
        if argument == "-compile_assets" {
            should_start_game = false;
            compile_assets = true;
        }
    }

    if should_start_game {
        start_game();
    }
    if compile_assets {
        assets::compile_assets();
    }
}

fn start_game() {
    println!("Starting game");

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
    let mut game_state = game::GameState::start(
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
        game_state.update(&mut render_context, delta_time as f32);

        for i in 0..render_context.gameobjects.len() {
            resource_context
                .get_gameobject_ref_mut(render_context.gameobjects[i])
                .update(&mut physics_context);
        }

        let mut target = display.draw();

        renderer::render(&mut render_context, &mut resource_context, &mut target);
        game_state.render_gui(&mut ui_context);

        ui_context.draw_frame(&mut resource_context, &mut target, &display);

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
                ev => {
                    game_state.process_input(&mut render_context, &ev);

                    let position = match ev {
                        glutin::WindowEvent::CursorMoved { position, .. } => Some(position),
                        _ => None,
                    };

                    if position.is_some() {
                        let position = position.unwrap();
                        ui_context.mouse_x = position.x as f32;
                        ui_context.mouse_y = position.y as f32;
                    }

                    match ev {
                        glutin::WindowEvent::MouseInput { button, state, .. } => match button {
                            glutin::MouseButton::Left => {
                                if state == glutin::ElementState::Pressed {
                                    ui_context.left_mouse_down = true;
                                } else {
                                    ui_context.left_mouse_down = false;
                                }
                            }
                            glutin::MouseButton::Right => {
                                if state == glutin::ElementState::Pressed {
                                    ui_context.right_mouse_down = true;
                                } else {
                                    ui_context.right_mouse_down = false;
                                }
                            }
                            _ => (),
                        },
                        _ => (),
                    };
                }
            },
            glutin::Event::DeviceEvent { event, .. } => match event {
                ev => {
                    game_state.process_input_device(&mut render_context, &ev);
                }
            },
            _ => (),
        });

        // get the elapsed time as nano secounds in case this frame happened very fast
        let elapsed_time = sw.elapsed().subsec_nanos();
        delta_time = elapsed_time as f64 / 1_000_000_000.0;
        sw.restart();
    }
}
