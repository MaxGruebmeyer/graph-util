use crate::datastructures::*;

use std::{
    cmp::min,
    collections::HashMap,
    hash::Hash,
};

/// Electrical constant used in the calculation of F_e
const K_E: f64 = 1.0;

/// Stability constant for calculation of F_s
const K_S: f64 = 1.0;

#[derive(Clone)]
struct Position {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct EdgeId<V> where V: Clone, V: Eq, V: Hash {
    a: V,
    b: V,
}

struct RenderInfo<V> where V: Clone, V: Eq, V: Hash {
    pos_info: HashMap<V, Position>,
    charge_info: HashMap<V, f64>,
    rod_info: HashMap<EdgeId<V>, f64>,
}

/// Get's the squared euclidean dist between a and b, e.g. (||b - a||_2)^2
fn get_squared_2_norm(pos_a: &Position, pos_b: &Position) -> f64 {
    (pos_b.x - pos_a.x).powf(2.0)
     + (pos_b.y - pos_a.y).powf(2.0)
     + (pos_b.z - pos_a.z).powf(2.0)
}

/// Get's the euclidean dist between a and b, e.g. ||b - a||_2
fn get_2_norm(pos_a: &Position, pos_b: &Position) -> f64 {
    get_squared_2_norm(pos_a, pos_b).sqrt()
}

/// Subtracts the second vector from the first.
fn sub(minuend: &Position, subtrahend: &Position) -> Position {
    Position {
        x: (minuend.x - subtrahend.x),
        y: (minuend.y - subtrahend.y),
        z: (minuend.z - subtrahend.z),
    }
}

/// Multiplies a vector with a scalar
fn mul(vec: &Position, mul: f64) -> Position {
    Position {
        x: vec.x * mul,
        y: vec.y * mul,
        z: vec.z * mul,
    }
}

/// Get's the unit vector of a and b, e.g. (b - a)/||b - a||_2
fn get_unit_vector(pos_a: &Position, pos_b: &Position) -> Position {
    let norm = get_2_norm(pos_a, pos_b);
    if norm == 0.0 {
        panic!("Cannot create unit vector. Norm is 0!");
    }

    mul(&sub(&pos_b, &pos_a), 1.0 / norm)
}

/// Calculates the electrical force between two vertices a and b
fn calc_electrical_force(pos_a: &Position, pos_b: &Position,
    charge_a: f64, charge_b: f64) -> Position {
    let scalar: f64 = K_E * ((charge_a * charge_b) / get_squared_2_norm(pos_a, pos_b));
    mul(&get_unit_vector(pos_a, pos_b), scalar)
}

/// Calculates the mechanical pullback force between two vertices a and b
///  by using the old and new dist as well as the unit vector between them.
fn calc_mechanical_force(old_dist: f64, new_dist: f64,
    unit_vector: &Position) -> Position {
    mul(unit_vector, -K_S * (new_dist - old_dist))
}

fn charge_modifier(c: f64) -> f64 {
    c
}

fn rod_modifier(l: f64) -> f64 {
    l
}

fn get_diff<V: Hash + Eq>(map_a: &HashMap<V, Position>, map_b: &HashMap<V, Position>) -> f64 {
    let mut norm: f64 = 0.0;
    for (k,v) in map_a {
        norm += get_2_norm(v, map_b.get(k).unwrap());
    }

    norm
}

fn update_positions<V: Clone + Hash + Eq>(render_info: &mut RenderInfo<V>) -> f64 {
    let mut updated_pos = render_info.pos_info.clone();

    // TODO (GM): Does this work?
    for (k1, pos1) in render_info.pos_info.iter() {
        for (k2, pos2) in render_info.pos_info.iter() {
            if k1 == k2 {
                continue;
            }

            let charge1: &f64 = render_info.charge_info.get(k1).unwrap();
            let charge2: &f64 = render_info.charge_info.get(k2).unwrap();

            let applied_e_force = calc_electrical_force(pos1, pos2, *charge1, *charge2);
            let updated = sub(updated_pos.get(k1).unwrap(), &applied_e_force);

            updated_pos.get_mut(k1).map(|val| { *val = updated; });
        }
    }

    for (edge, rod_len) in render_info.rod_info.iter() {
        let a: &V = &edge.a;
        let b: &V = &edge.b;

        // TODO (GM): Remove?
        // let old_pos_a = render_info.pos_info.get(a).unwrap();
        // let old_pos_b = render_info.pos_info.get(b).unwrap();
        // let old_dist = get_2_norm(old_pos_b, old_pos_a);
        let old_dist = *rod_len;

        let new_pos_a = updated_pos.get(a).unwrap();
        let new_pos_b = updated_pos.get(b).unwrap();

        let new_dist = get_2_norm(new_pos_b, new_pos_a);

        // TODO (GM): Is this correct?
        let unit_vec = get_unit_vector(new_pos_a, new_pos_b);
        let applied_mech_force_a = calc_mechanical_force(old_dist,
            new_dist,
            &unit_vec);

        let applied_mech_force_b = mul(&applied_mech_force_a, -1.0);

        // TODO (GM): Does this work?
        updated_pos.get_mut(a).map(|val| { *val = applied_mech_force_a });
        updated_pos.get_mut(b).map(|val| { *val = applied_mech_force_b });
    }

    let diff = get_diff(&updated_pos, &render_info.pos_info);
    render_info.pos_info = updated_pos;

    diff
}

/// HashMap to map values to rod lengths.
/// Assumes values are unique among nodes which maps to the edge identifier.
/// Assumes for any two vertices a and b there is at most one edge between them.
fn init_rod_map<V: Clone, E: Clone>(graph: &Graph<V, E>) -> HashMap<EdgeId<V>, f64> where V: Eq, V: Hash {
    let mut rod_map: HashMap<EdgeId<V>, f64> = HashMap::with_capacity(graph.len());
    for node in graph {
        for edge in &node.borrow().edges {
            let a = edge.a.borrow();
            let b = edge.b.borrow();

            let edge_id = EdgeId { a: a.val.clone(), b: b.val.clone() };
            let val = rod_modifier(min(a.get_deg(), b.get_deg()) as f64);

            rod_map.insert(edge_id, val);
        }
    }

    rod_map
}

/// Initializes HashMap to map values to charges.
/// Assumes values are unique among nodes.
fn init_charge_map<V: Clone, E: Clone>(graph: &Graph<V, E>) -> HashMap<V, f64> where V: Eq, V: Hash {
    let mut pos_lookup: HashMap<V, f64> = HashMap::with_capacity(graph.len());
    for node in graph {
        let key: V = node.borrow().val.clone();
        let val = charge_modifier(node.borrow().get_deg() as f64);

        match pos_lookup.insert(key, val) {
            None => {},
            Some(_) => { panic!("Node values were not unique, a key occurred multiple times!"); },
        }
    }

    pos_lookup
}

/// Initializes HashMap to map values to positions.
/// Assumes values are unique among nodes.
fn init_pos_map<V: Clone, E: Clone>(graph: &Graph<V, E>) -> HashMap<V, Position> where V: Eq, V: Hash {
    let mut pos_lookup: HashMap<V, Position> = HashMap::with_capacity(graph.len());
    for node in graph {
        let key: V = node.borrow().val.clone();
        let pos = Position { x: 0.0, y: 0.0, z: 0.0 };

        match pos_lookup.insert(key, pos) {
            None => {},
            Some(_) => { panic!("Node values were not unique, a key occurred multiple times!"); },
        }
    }

    pos_lookup
}

// TODO (GM): Documentation!
pub fn visualize<V: Clone, E: Clone>(graph: &Graph<V, E>) where V: Eq, V: Hash {
    let mut render_info = RenderInfo {
        pos_info: init_pos_map(&graph),
        charge_info: init_charge_map(&graph),
        rod_info: init_rod_map(&graph),
    };

    // TODO (GM): Re-apply this until it converges?
    for _ in 0..5 {
        let diff = update_positions(&mut render_info);
        println!("New difference is {diff}");
    }
}
