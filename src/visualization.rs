use crate::datastructures::*;

use std::{
    cmp::min,
    collections::HashMap,
    hash::Hash,
};

/// Electrical constant used in the calculation of F_e
const k_e: f64 = 1.0;

/// Stability constant for calculation of F_s
const k_s: f64 = 1.0;

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

fn get_2_norm(pos_a: &Position, pos_b: &Position) -> f64 {
    ((pos_a.x - pos_b.x).powf(2.0)
     + (pos_a.y - pos_b.y).powf(2.0)
     + (pos_a.z - pos_b.z).powf(2.0)).sqrt()
}

fn calculate_electrical_force(pos_a: &Position, pos_b: &Position) {

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
