use math_helper;
use vector::Vector3;

#[derive(Copy, PartialEq, Clone, Debug)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[allow(dead_code)]
impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Quaternion {
        Quaternion {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    pub fn new_axis_angle(axis: Vector3, angle: f32) -> Quaternion {
        let sin_half_angle = (angle / 2.0).sin();
        let cos_half_angle = (angle / 2.0).cos();

        Quaternion {
            x: axis.x * sin_half_angle,
            y: axis.y * sin_half_angle,
            z: axis.z * sin_half_angle,
            w: cos_half_angle,
        }
    }

    pub fn to_euler(&self) -> Vector3 {
        let q1 = self.normalized();
        let e_x = (-2.0 * (q1.y * q1.z - q1.w * q1.x))
            .atan2(q1.w * q1.w - q1.x * q1.x - q1.y * q1.y + q1.z * q1.z);
        let e_y = (2.0 * (q1.x * q1.z + q1.w * q1.y)).asin();
        let e_z = (-2.0 * (q1.x * q1.y - q1.w * q1.z))
            .atan2(q1.w * q1.w + q1.x * q1.x - q1.y * q1.y - q1.z * q1.z);

        return Vector3::new(e_x.to_degrees(), e_y.to_degrees(), e_z.to_degrees());
    }

    pub fn from_euler(euler: Vector3) -> Quaternion {
        let e_x = euler.x.to_radians();
        let e_y = euler.y.to_radians();
        let e_z = euler.z.to_radians();

        let c1 = (e_x / 2.0).cos();
        let s1 = (e_x / 2.0).sin();
        let c2 = (e_y / 2.0).cos();
        let s2 = (e_y / 2.0).sin();
        let c3 = (e_z / 2.0).cos();
        let s3 = (e_z / 2.0).sin();

        let q_w = c1 * c2 * c3 - s1 * s2 * s3;
        let q_x = s1 * c2 * c3 + c1 * s2 * s3;
        let q_y = c1 * s2 * c3 - s1 * c2 * s3;
        let q_z = c1 * c2 * s3 + s1 * s2 * c3;

        return Quaternion::new(q_x, q_y, q_z, q_w);
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalized(&self) -> Quaternion {
        let length = self.length();

        Quaternion {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
            w: self.w / length,
        }
    }

    pub fn conjugate(&self) -> Quaternion {
        Quaternion {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }

    pub fn mul_f32(&self, r: f32) -> Quaternion {
        Quaternion {
            x: self.x * r,
            y: self.y * r,
            z: self.z * r,
            w: self.w * r,
        }
    }

    pub fn mul_quat(&self, r: Quaternion) -> Quaternion {
        let w_ = self.w * r.w - self.x * r.x - self.y * r.y - self.z * r.z;
        let x_ = self.x * r.w + self.w * r.x + self.y * r.z - self.z * r.y;
        let y_ = self.y * r.w + self.w * r.y + self.z * r.x - self.x * r.z;
        let z_ = self.z * r.w + self.w * r.z + self.x * r.y - self.y * r.x;

        Quaternion {
            x: x_,
            y: y_,
            z: z_,
            w: w_,
        }
    }

    pub fn mul_vec3(&self, r: Vector3) -> Quaternion {
        let w_ = -self.x * r.x - self.y * r.y - self.z * r.z;
        let x_ = self.w * r.x + self.y * r.z - self.z * r.y;
        let y_ = self.w * r.y + self.z * r.x - self.x * r.z;
        let z_ = self.w * r.z + self.x * r.y - self.y * r.x;

        Quaternion {
            x: x_,
            y: y_,
            z: z_,
            w: w_,
        }
    }

    pub fn sub(&self, r: Quaternion) -> Quaternion {
        Quaternion {
            x: self.x - r.x,
            y: self.y - r.y,
            z: self.z - r.z,
            w: self.w - r.w,
        }
    }

    pub fn add(&self, r: Quaternion) -> Quaternion {
        Quaternion {
            x: self.x + r.x,
            y: self.y + r.y,
            z: self.z + r.z,
            w: self.w + r.w,
        }
    }

    pub fn to_rotation_matrix(&self) -> [[f32; 4]; 4] {
        let forward = Vector3::new(
            2.0 * (self.x * self.z - self.w * self.y),
            2.0 * (self.y * self.z + self.w * self.x),
            1.0 - 2.0 * (self.x * self.x + self.y * self.y),
        );
        let up = Vector3::new(
            2.0 * (self.x * self.y + self.w * self.z),
            1.0 - 2.0 * (self.x * self.x + self.z * self.z),
            2.0 * (self.y * self.z - self.w * self.x),
        );
        let right = Vector3::new(
            1.0 - 2.0 * (self.y * self.y + self.z * self.z),
            2.0 * (self.x * self.y - self.w * self.z),
            2.0 * (self.x * self.z + self.w * self.y),
        );

        math_helper::rotation_matrix_vec3(forward, up, right)
    }

    pub fn dot(&self, r: Quaternion) -> f32 {
        self.x * r.x + self.y * r.y + self.z * r.z + self.w * r.w
    }

    pub fn rotate(&self, angle: Vector3) -> Vector3 {
        let con = self.conjugate();
        let w = self.mul_vec3(angle).mul_quat(con);
        Vector3::new(w.x, w.y, w.z)
    }

    pub fn forward(&self) -> Vector3 {
        self.rotate(Vector3::new(0.0, 0.0, 1.0))
    }
    pub fn right(&self) -> Vector3 {
        self.rotate(Vector3::new(1.0, 0.0, 0.0))
    }
}
