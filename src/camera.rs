extern crate glium;
use glium::glutin;
use math_helper;
use quaternion::Quaternion;

pub struct CameraState {
    aspect_ratio: f32,
    position: [f32; 3],
    rotation: Quaternion,

    moving_up: bool,
    moving_left: bool,
    moving_down: bool,
    moving_right: bool,
    moving_forward: bool,
    moving_backward: bool,

    mouse_locked: bool,

    last_cursor_position_x: i32,
    last_cursor_position_y: i32,
    last_cursor_set: bool,
}

impl CameraState {
    pub fn new(screen_width: i32, screen_height: i32) -> CameraState {
        CameraState {
            aspect_ratio: screen_width as f32 / screen_height as f32,
            position: [0.0, 0.0, 0.0],
            rotation: Quaternion::new(0.0, 0.0, 0.0, 1.0),
            moving_up: false,
            moving_left: false,
            moving_down: false,
            moving_right: false,
            moving_forward: false,
            moving_backward: false,
            mouse_locked: false,
            last_cursor_position_x: 0,
            last_cursor_position_y: 0,
            last_cursor_set: false,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        let sensitivityX = 2.66 * delta_time;
        let sensitivityY = 2.0 * delta_time;

        // println!("[{}, {}]", mouse_delta_x, mouse_delta_y);

        if self.moving_forward {
            let forward = self.rotation.forward();
            self.position[0] += forward[0] * 1.0 * delta_time;
            self.position[1] += forward[1] * 1.0 * delta_time;
            self.position[2] += forward[2] * 1.0 * delta_time;
        }
        if self.moving_backward {
            let forward = self.rotation.forward();
            self.position[0] -= forward[0] * 1.0 * delta_time;
            self.position[1] -= forward[1] * 1.0 * delta_time;
            self.position[2] -= forward[2] * 1.0 * delta_time;
        }
    }

    fn rotate(&mut self, axis: [f32; 3], angle: f32) {
        let rot = Quaternion::new_axis_angle(axis, angle);

        let old_rot = self.rotation;

        self.rotation = rot.mul_quat(old_rot);
    }

    fn get_rotation_matrix(&self) -> [[f32; 4]; 4] {
        self.rotation.conjugate().to_rotation_matrix()
    }

    fn get_translation_matrix(&self) -> [[f32; 4]; 4] {
        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [-self.position[0], -self.position[1], -self.position[2], 1.0],
        ]
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        math_helper::mat_mul(self.get_translation_matrix(), self.get_rotation_matrix())
    }

    pub fn get_perspective(&self) -> [[f32; 4]; 4] {
        let fov: f32 = 3.141592 / 2.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [f / self.aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
            [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
        ]
    }

    fn process_key(&mut self, event: &glutin::WindowEvent) {
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
            glutin::VirtualKeyCode::Tab => self.mouse_locked = !self.mouse_locked,
            _ => (),
        };
    }

    fn process_mouse(&mut self, event: &glutin::WindowEvent) {
        let cursor_moved = match *event {
            glutin::WindowEvent::CursorMoved { position, .. } => position,
            _ => return,
        };

        let current_cursor_position_x = cursor_moved.x as i32;
        let current_cursor_position_y = cursor_moved.y as i32;

        if self.mouse_locked {
            let mouse_delta_x = current_cursor_position_x - self.last_cursor_position_x;
            let mouse_delta_y = current_cursor_position_y - self.last_cursor_position_y;

            let right = self.rotation.right();

            if mouse_delta_x != 0 {
                self.rotate([0.0, 1.0, 0.0], mouse_delta_x as f32 / 50.0);
            }
            if mouse_delta_y != 0 {
                self.rotate(right, mouse_delta_y as f32 / 50.0);
            }
        }

        self.last_cursor_position_x = current_cursor_position_x;
        self.last_cursor_position_y = current_cursor_position_y;
        self.last_cursor_set = true;
    }

    pub fn process_input(&mut self, event: &glutin::WindowEvent) {
        self.process_key(event);
        self.process_mouse(event);
    }
}
