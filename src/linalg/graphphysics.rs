use crate::linalg::structs::*;
use crate::linalg::vectorarithmetic::*;
use crate::linalg::physics::*;
use crate::linalg::graphstructs::*;

use std::{
    collections::HashMap,
    hash::Hash,
};

/// Tolerance that defines once a graph system is considered stable
const DRIFT_TOL: f64 = 10e-5;

/// Max amount of iterations until the calculation is aborted
const MAX_DRIFT_ITERATIONS: usize = 250;

fn get_diff<V: Hash + Eq>(map_a: &HashMap<V, Pos3D>, map_b: &HashMap<V, Pos3D>) -> f64 {
    let mut norm: f64 = 0.0;
    for (k,v) in map_a {
        norm += get_2_norm(v, map_b.get(k).unwrap());
    }

    norm
}

fn apply_e_forces<V: Clone + Hash + Eq>(render_info: &RenderInfo<V>)
-> HashMap<V, Pos3D> {
    let mut updated_pos = render_info.pos_info.clone();

    for (k1, pos1) in render_info.pos_info.iter() {
        for (k2, pos2) in render_info.pos_info.iter() {
            if k1 == k2 {
                continue;
            }

            let charge1: &f64 = render_info.charge_info.get(k1).unwrap();
            let charge2: &f64 = render_info.charge_info.get(k2).unwrap();

            let applied_e_force = calc_electrical_force(pos1, pos2, *charge1, *charge2);
            let updated = sub_vec(updated_pos.get(k1).unwrap(), &applied_e_force);

            updated_pos.get_mut(k1).map(|val| { *val = updated; });
        }
    }

    updated_pos
}

fn apply_mech_forces<V: Clone + Hash + Eq>(render_info: &RenderInfo<V>,
    updated_pos: &mut HashMap<V, Pos3D>) {
    for (edge, rod_len) in render_info.rod_info.iter() {
        let a: &V = &edge.a;
        let b: &V = &edge.b;

        // Always use the rod length as the old distance.
        // We currently DO NOT support stretching rods.
        // Furthermore it's not guaranteed, that the old distance is actually
        //  equal to the rod length -> TODO (GM): Fix that?
        let old_dist = *rod_len;

        let new_pos_a = updated_pos.get(a).unwrap();
        let new_pos_b = updated_pos.get(b).unwrap();
        let new_dist = get_2_norm(new_pos_b, new_pos_a);
        let unit_vec = get_unit_vector(new_pos_a, new_pos_b);

        // Only half the force needs to be applied to both parties
        let mech_force = mul(
            &calc_mechanical_force(old_dist, new_dist, &unit_vec),
            0.5);

        updated_pos.get_mut(a).map(|val| { *val = add_vec(val, &mech_force ) });
        updated_pos.get_mut(b).map(|val| { *val = sub_vec(val, &mech_force ) });
    }
}

fn update_positions<V: Clone + Hash + Eq>(render_info: &mut RenderInfo<V>) -> f64 {
    let mut updated_pos = apply_e_forces(render_info);

    // TODO (GM): Use return type instead of mutable borrow?
    apply_mech_forces(render_info, &mut updated_pos);

    let diff = get_diff(&updated_pos, &render_info.pos_info);
    render_info.pos_info = updated_pos;

    diff
}

// TODO (GM): Make this converge way faster and work for general graphs
//  -> e.g. use lagrange multiplicators, gradient descent or some other shit
pub fn calc_til_stable<V: Clone + Hash + Eq>(render_info: &mut RenderInfo<V>) {
    let mut diff = 0.0;
    for _ in 0..MAX_DRIFT_ITERATIONS {
        diff = update_positions(render_info);
        println!("Diff: {diff}");

        if diff < DRIFT_TOL {
            return;
        }
    }

    panic!("Could not calculate the system in {MAX_DRIFT_ITERATIONS} iterations! Last diff was {diff}.");
}
