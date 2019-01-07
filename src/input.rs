use glium::glutin;
pub struct Input {
    last_keys: Vec<bool>,
    current_keys: Vec<bool>,

    last_mouse: Vec<bool>,
    current_mouse: Vec<bool>,

    mouse_position: [f32; 2],
}

#[allow(dead_code)]
impl Input {
    pub fn new() -> Input {
        Input {
            last_keys: vec![false; 161],
            current_keys: vec![false; 161],

            last_mouse: vec![false; 32], // hardcoded max mouse buttons, TODO: maybe quary the hardware for the actual size
            current_mouse: vec![false; 32],

            mouse_position: [0.0, 0.0],
        }
    }

    pub fn get_key_down(&mut self, code: glutin::VirtualKeyCode) -> bool {
        (self.current_keys[code as usize] && !self.last_keys[code as usize])
    }

    pub fn get_key(&mut self, code: glutin::VirtualKeyCode) -> bool {
        self.current_keys[code as usize]
    }

    pub fn get_key_up(&mut self, code: glutin::VirtualKeyCode) -> bool {
        (!self.current_keys[code as usize] && self.last_keys[code as usize])
    }

    pub fn get_mouse_down(&mut self, mouse_button: glium::glutin::MouseButton) -> bool {
        (self.current_mouse[mouse_code_to_id(mouse_button)]
            && !self.last_mouse[mouse_code_to_id(mouse_button)])
    }

    pub fn get_mouse(&mut self, mouse_button: glium::glutin::MouseButton) -> bool {
        self.current_mouse[mouse_code_to_id(mouse_button)]
    }

    pub fn get_mouse_up(&mut self, mouse_button: glium::glutin::MouseButton) -> bool {
        (!self.current_mouse[mouse_code_to_id(mouse_button)]
            && self.last_mouse[mouse_code_to_id(mouse_button)])
    }

    pub fn get_mouse_pos(&mut self) -> [f32; 2] {
        return self.mouse_position;
    }

    pub fn start_event_proc(&mut self) {
        // copy the current keys into the last keys array
        for i in 0..self.current_keys.len() {
            self.last_keys[i] = self.current_keys[i];
        }

        for i in 0..self.current_mouse.len() {
            self.last_mouse[i] = self.current_mouse[i];
        }
    }

    pub fn process_events(&mut self, event: &glutin::WindowEvent) {
        let keyboard_input = match *event {
            glutin::WindowEvent::KeyboardInput { input, .. } => Some(input),
            _ => None,
        };

        let mouse_input = match *event {
            glutin::WindowEvent::MouseInput { button, state, .. } => Some((button, state)),
            _ => None,
        };

        let mouse_position = match *event {
            glutin::WindowEvent::CursorMoved { position, .. } => Some(position),
            _ => None,
        };

        if mouse_position.is_some() {
            let mouse_position = mouse_position.unwrap();
            self.mouse_position = [mouse_position.x as f32, mouse_position.y as f32];
        }

        if keyboard_input.is_some() {
            let keyboard_input = keyboard_input.unwrap();
            let pressed = keyboard_input.state == glutin::ElementState::Pressed;

            let key = match keyboard_input.virtual_keycode {
                Some(key) => key,
                None => return,
            };

            if pressed {
                self.current_keys[key as usize] = true;
            } else {
                self.current_keys[key as usize] = false;
            }
        }

        if mouse_input.is_some() {
            let (mouse_button, mouse_state) = mouse_input.unwrap();

            if mouse_state == glutin::ElementState::Pressed {
                self.current_mouse[mouse_code_to_id(mouse_button)] = true;
            } else {
                self.current_mouse[mouse_code_to_id(mouse_button)] = false;
            }
        }
    }
}

fn mouse_code_to_id(mouse_button: glium::glutin::MouseButton) -> usize {

    let result = match mouse_button {
        glutin::MouseButton::Left => 0,
        glutin::MouseButton::Right => 1,
        glutin::MouseButton::Middle =>  2,
        glutin::MouseButton::Other(mb) =>  mb,
    } as i32;

    if result == -1 {
        println!("Unknown mouse key code");
        return 0;
    }

    result as usize
}
