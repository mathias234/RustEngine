pub fn new_matrix() -> [[f32; 4]; 4] {
    [
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
    ]
}

pub fn mat_mul(l: [[f32; 4]; 4], r: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut res: [[f32; 4]; 4] = new_matrix();

    for i in 0..4 {
        for j in 0..4 {
            res[i][j] =
                l[i][0] * r[0][j] + l[i][1] * r[1][j] + l[i][2] * r[2][j] + l[i][3] * r[3][j];
        }
    }

    return res;
}

pub fn vec3_mul(l: [f32; 3], r: [f32; 3]) -> [f32; 3] {
    let mut res: [f32; 3] = [0.0, 0.0, 0.0];

    for i in 0..3 {
        res[i] = l[i] * r[i];
    }

    return res;
}

pub fn vec3_div(l: [f32; 3], r: [f32; 3]) -> [f32; 3] {
    let mut res: [f32; 3] = [0.0, 0.0, 0.0];

    for i in 0..3 {
        res[i] = l[i] / r[i];
    }

    return res;
}

pub fn rotaiton_matrix(x: f32, y: f32, z: f32) -> [[f32; 4]; 4] {
    let cos_x = x.cos();
    let sin_x = x.sin();

    let cos_y = y.cos();
    let sin_y = y.sin();

    let cos_z = z.cos();
    let sin_z = z.sin();

    let rz = [
        [cos_z, sin_z, 0.0, 0.0],
        [-sin_z, cos_z, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let rx = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, cos_x, sin_x, 0.0],
        [0.0, -sin_x, cos_x, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let ry = [
        [cos_y, 0.0, sin_y, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-sin_y, 0.0, cos_y, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];

    mat_mul(rz, mat_mul(ry, rx))
}

pub fn rotation_matrix_vec3(forward: [f32; 3], up: [f32; 3], right: [f32; 3]) -> [[f32; 4]; 4] {
    let f = forward;
    let r = right;
    let u = up;

    [
        [r[0], u[0], f[0], 0.0],
        [r[1], u[1], f[1], 0.0],
        [r[2], u[2], f[2], 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}
