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

/// Get's the squared euclidean distance between a and b, e.g. (||b - a||_2)^2
fn get_squared_2_norm(pos_a: &Position, pos_b: &Position) -> f64 {
    (pos_b.x - pos_a.x).powf(2.0)
     + (pos_b.y - pos_a.y).powf(2.0)
     + (pos_b.z - pos_a.z).powf(2.0)
}

/// Get's the euclidean distance between a and b, e.g. ||b - a||_2
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
    let inv_norm: f64 = 1.0 / get_2_norm(pos_a, pos_b);
    mul(&sub(&pos_b, &pos_a), inv_norm)
}

/// Calculates the electrical force between two vertices a and b
fn calculate_electrical_force(pos_a: &Position, pos_b: &Position,
    charge_a: f64, charge_b: f64) -> Position {
    let scalar: f64 = K_E * ((charge_a * charge_b) / get_squared_2_norm(pos_a, pos_b));
    mul(&get_unit_vector(pos_a, pos_b), scalar)
}

/// Calculates the mechanical pullback force between two vertices a and b
///  by using the old and new distance as well as the unit vector between them.
fn calculate_mechanical_force(old_distance: f64, new_distance: f64,
    unit_vector: &Position) -> Position {
    mul(unit_vector, -K_S * (new_distance - old_distance))
}

fn charge_modifier(c: f64) -> f64 {
    c
}

fn len_modifier(l: f64) -> f64 {
    l
}

/// HashMap to map values to rod lengths.
/// Assumes values are unique among nodes which maps to the edge identifier.
/// Assumes for any two vertices a and b there is at most one edge between them.
fn init_len_map<V: Clone, E: Clone>(graph: &Graph<V, E>) -> HashMap<EdgeId<V>, f64> where V: Eq, V: Hash {
    let mut len_map: HashMap<EdgeId<V>, f64> = HashMap::with_capacity(graph.len());
    for node in graph {
        for edge in &node.borrow().edges {
            let a = edge.a.borrow();
            let b = edge.b.borrow();

            let edge_id = EdgeId { a: a.val.clone(), b: b.val.clone() };
            let val = len_modifier(min(a.get_deg(), b.get_deg()) as f64);

            len_map.insert(edge_id, val);
        }
    }

    len_map
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
    let pos_map = init_pos_map(&graph);
    let charge_map = init_charge_map(&graph);
    let len_map = init_len_map(&graph);
}
