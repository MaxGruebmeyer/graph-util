use crate::linalg::structs::*;
use crate::linalg::vectorarithmetic::*;

/// Electrical constant used in the calculation of F_e
const K_E: f64 = 1.0;

/// Stability constant for calculation of F_s
const K_S: f64 = 1.0;

/// Get's the unit vector of a and b, e.g. (b - a)/||b - a||_2
pub fn get_unit_vector(pos_a: &Pos3D, pos_b: &Pos3D) -> Pos3D {
    let norm = get_2_norm(pos_a, pos_b);
    if norm == 0.0 {
        panic!("Cannot create unit vector. Norm is 0!");
    }

    mul(&sub_vec(&pos_b, &pos_a), 1.0 / norm)
}

/// Calculates the electrical force between two vertices a and b
pub fn calc_electrical_force(pos_a: &Pos3D, pos_b: &Pos3D,
    charge_a: f64, charge_b: f64) -> Pos3D {
    let scalar: f64 = K_E * ((charge_a * charge_b) / get_squared_2_norm(pos_a, pos_b));
    mul(&get_unit_vector(pos_a, pos_b), scalar)
}

/// Calculates the mechanical pullback force between two vertices a and b
///  by using the old and new dist as well as the unit vector between them.
pub fn calc_mechanical_force(old_dist: f64, new_dist: f64,
    unit_vector: &Pos3D) -> Pos3D {
    mul(unit_vector, K_S * (new_dist - old_dist))
}
