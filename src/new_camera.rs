struct CameraState {
    aspect_ratio: f32,
    position: [f32; 3],
    rotation: [f32; 3],

    moving_up: bool,
    moving_left: bool,
    moving_down: bool,
    moving_right: bool,
    moving_forward: bool,
    moving_backward: bool,
}
impl CameraState {
    pub fn new(i32 screen_width, i32 screen_height) -> CameraState {
        CameraState {
            aspect_ratio: screen_width as f32 / screen_height as f32,
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            moving_up: false,
            moving_left: false,
            moving_down: false,
            moving_right: false,
            moving_forward: false,
            moving_backward: false,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        
    }

    pub fn process_input(&mut self, event: &glutin::WindowEvent) {
        let input = match *event {
            glutin::WindowEvent::KeyboardInput { input, .. } => input,
            _ => return,
        };
        let pressed = input.state == glutin::ElementState::Pressed;
        let key = match input.virtual_keycode {
            Some(key) => key,
            None => return,
        };
        match key {
            glutin::VirtualKeyCode::Up => self.moving_up = pressed,
            glutin::VirtualKeyCode::Down => self.moving_down = pressed,
            glutin::VirtualKeyCode::A => self.moving_left = pressed,
            glutin::VirtualKeyCode::D => self.moving_right = pressed,
            glutin::VirtualKeyCode::W => self.moving_forward = pressed,
            glutin::VirtualKeyCode::S => self.moving_backward = pressed,
            _ => (),
        };
    }
}
