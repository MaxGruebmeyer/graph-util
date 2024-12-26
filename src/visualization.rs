use crate::datastructures::*;

use std::{
    collections::HashMap,
    hash::Hash,
};

struct Position {
    x: f64,
    y: f64,
    z: f64,
}

// TODO (GM): Work on visualization!

// TODO (GM): Documentation!
pub fn visualize<V: std::clone::Clone, E: std::clone::Clone>(graph: &Graph<V, E>) where V: Eq, V: Hash {
    // HashMap to map values to positions.
    // Assumes values are unique among nodes.
    let mut pos_lookup: HashMap<V, Position> = HashMap::with_capacity(graph.len());

    for node in graph {
        let key: V = node.borrow().val.clone();
        let pos = Position { x: 0.0, y: 0.0, z: 0.0 };

        match pos_lookup.insert(key, pos) {
            None => {},
            Some(_) => { panic!("Node values were not unique, a key occurred multiple times!"); },
        }

    }
}
