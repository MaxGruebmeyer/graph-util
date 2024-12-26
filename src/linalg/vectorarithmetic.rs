use crate::linalg::structs::*;

/// Get's the squared euclidean dist between a and b, e.g. (||b - a||_2)^2
pub fn get_squared_2_norm(pos_a: &Pos3D, pos_b: &Pos3D) -> f64 {
    (pos_b.x - pos_a.x).powf(2.0)
     + (pos_b.y - pos_a.y).powf(2.0)
     + (pos_b.z - pos_a.z).powf(2.0)
}

/// Get's the euclidean dist between a and b, e.g. ||b - a||_2
pub fn get_2_norm(pos_a: &Pos3D, pos_b: &Pos3D) -> f64 {
    get_squared_2_norm(pos_a, pos_b).sqrt()
}

/// Adds both vectors
pub fn add_vec(a: &Pos3D, b: &Pos3D) -> Pos3D {
    Pos3D { x: (a.x + b.x), y: (a.y + b.y), z: (a.z + b.z) }
}

/// Subtracts the second vector from the first.
pub fn sub_vec(minuend: &Pos3D, subtrahend: &Pos3D) -> Pos3D {
    Pos3D {
        x: (minuend.x - subtrahend.x),
        y: (minuend.y - subtrahend.y),
        z: (minuend.z - subtrahend.z),
    }
}

/// Multiplies a vector with a scalar
pub fn mul(vec: &Pos3D, mul: f64) -> Pos3D {
    Pos3D {
        x: vec.x * mul,
        y: vec.y * mul,
        z: vec.z * mul,
    }
}
