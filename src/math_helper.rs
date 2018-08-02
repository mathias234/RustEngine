pub fn new_matrix() -> [[f32; 4]; 4] {
    [
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
    ]    
}

pub fn mat_mul(l: [[f32; 4]; 4], r: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut res : [[f32; 4]; 4] = new_matrix();

    for i in 0..4 {
        for j in 0..4 {
            res[i][j] =
                    l[i][0] * r[0][j] +
                    l[i][1] * r[1][j] +
                    l[i][2] * r[2][j] +
                    l[i][3] * r[3][j];
        }
    }

    return res;
}