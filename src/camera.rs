extern crate glium;

use glium::glutin;
use math_helper;
use quaternion::Quaternion;
use vector::Vector3;

pub struct CameraState {
    aspect_ratio: f32,
    pub position: Vector3,
    pub rotation: Quaternion,

    moving_up: bool,
    moving_left: bool,
    moving_down: bool,
    moving_right: bool,
    moving_forward: bool,
    moving_backward: bool,

    mouse_locked: bool,
}

impl CameraState {
    pub fn new(screen_width: i32, screen_height: i32) -> CameraState {
        CameraState {
            aspect_ratio: screen_width as f32 / screen_height as f32,
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Quaternion::new(0.0, 0.0, 0.0, 1.0),
            moving_up: false,
            moving_left: false,
            moving_down: false,
            moving_right: false,
            moving_forward: false,
            moving_backward: false,
            mouse_locked: false,
        }
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.aspect_ratio = width as f32 / height as f32;
    }

    pub fn update(&mut self, delta_time: f32) {
        let move_speed = 5.5;
        if self.moving_forward {
            let forward = self.rotation.forward();
            self.position.x += forward.x * move_speed * delta_time;
            self.position.y += forward.y * move_speed * delta_time;
            self.position.z += forward.z * move_speed * delta_time;
        }
        if self.moving_backward {
            let forward = self.rotation.forward();
            self.position.x -= forward.x * move_speed * delta_time;
            self.position.y -= forward.y * move_speed * delta_time;
            self.position.z -= forward.z * move_speed * delta_time;
        }
        if self.moving_left {
            let right = self.rotation.right();
            self.position.x -= right.x * move_speed * delta_time;
            self.position.y -= right.y * move_speed * delta_time;
            self.position.z -= right.z * move_speed * delta_time;
        }
        if self.moving_right {
            let right = self.rotation.right();
            self.position.x += right.x * move_speed * delta_time;
            self.position.y += right.y * move_speed * delta_time;
            self.position.z += right.z * move_speed * delta_time;
        }
    }

    fn rotate(&mut self, axis: Vector3, angle: f32) {
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
            [-self.position.x, -self.position.y, -self.position.z, 1.0],
        ]
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        math_helper::mat_mul(self.get_translation_matrix(), self.get_rotation_matrix())
    }

    pub fn get_perspective(&self) -> [[f32; 4]; 4] {
        // 60 degrees fov
        let fov: f32 = 1.04719755;
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
            glutin::VirtualKeyCode::LControl => self.mouse_locked = pressed,
            _ => (),
        };
    }

    fn process_mouse(&mut self, event: &glutin::DeviceEvent) {
        let delta = match *event {
            glutin::DeviceEvent::MouseMotion { delta, .. } => delta,
            _ => return,
        };

        let mouse_delta_x = delta.0;
        let mouse_delta_y = delta.1;

        if self.mouse_locked {
            let right = self.rotation.right();

            if mouse_delta_x != 0.0 {
                self.rotate(Vector3::new(0.0, 1.0, 0.0), mouse_delta_x as f32 / 50.0);
            }
            if mouse_delta_y != 0.0 {
                self.rotate(right, mouse_delta_y as f32 / 50.0);
            }
        }
    }

    pub fn process_input(&mut self, event: &glutin::WindowEvent) {
        self.process_key(event);
    }

    pub fn process_input_device(&mut self, event: &glutin::DeviceEvent) {
        self.process_mouse(event);
    }
}
