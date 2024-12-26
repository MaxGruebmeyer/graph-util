use crate::datastructures::*;
use crate::linalg::structs::*;
use crate::linalg::graphphysics::*;

use std::{
    cmp::min,
    collections::HashMap,
    hash::Hash,
};

fn charge_modifier(c: f64) -> f64 {
    c
}

fn rod_modifier(l: f64) -> f64 {
    l
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

/// Initializes HashMap to map values to positions.
/// Assumes values are unique among nodes.
fn init_pos_map<V: Clone, E: Clone>(graph: &Graph<V, E>) -> HashMap<V, Pos3D> where V: Eq, V: Hash {
    let mut pos_lookup: HashMap<V, Pos3D> = HashMap::with_capacity(graph.len());
    for i in 0..graph.len() {
        let key: V = graph[i].borrow().val.clone();

        // Use i as iterator because there will be problems if two nodes start
        //  at the same position because unit vector cannot be created
        //  (division by zero).
        let pos = Pos3D { x: i as f64, y: i as f64, z: i as f64 };

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

    calc_til_stable(&mut render_info);
}
