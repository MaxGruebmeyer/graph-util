extern crate regex;
use regex::Regex;

use crate::datastructures::*;

use std::{
    collections::HashMap,
    fs::File,
    io::BufRead,
    io::BufReader,
};

// TODO (GM): write_admat function!

pub fn read_admat(filename: &str) -> AdMat {
    println!("Reading adjacency matrix from file '{}'", filename);

    let mut mat: AdMat = Vec::new();
    let num_regex = Regex::new(r"(\d+)").unwrap();

    for line in BufReader::new(File::open(filename)
        .expect("Could not open file!"))
        .lines()
        .map(|l| l.expect("Could not read line!")) {
            let mut cur: Vec<Weight> = Vec::new();
            for (_, [num]) in num_regex.captures_iter(&line)
                 .map(|c| c.extract()) {
                     cur.push(num.parse::<Weight>().unwrap());
                 }

            mat.push(cur);
        }

    mat
}

// TODO (GM): Make this generic?
pub fn admat_to_graph(admat: &AdMat) -> DGraph {
    let mut graph: DGraph = Vec::new();

    for i in 0..admat.len() {
        graph.push(Node::new(i as Value));
    }

    for i in 0..admat.len() {
        for j in 0..admat.len() {
            if admat[i][j] == 0 {
                continue;
            }

            // TODO (GM): Does this work if a == b?
            let a = &graph[i];
            let b = &graph[j];

            Edge::new(a, b, admat[i][j] as Weight);
        }
    }

    graph
}

// TODO (GM): Documentation!
pub fn graph_to_admat(graph: &DGraph) -> AdMat {
    let mut admat: AdMat = vec![vec![0; graph.len()]; graph.len()];

    // HashMap to map values to positions.
    // Assumes values are unique among nodes.
    let mut pos_lookup: HashMap<Value, usize> = HashMap::with_capacity(graph.len());

    for i in 0..graph.len() {
        let key = graph[i].borrow().val;
        match pos_lookup.insert(key, i) {
            None => {},
            Some(_) => { panic!("Node values were not unique, {key} occurred multiple times!"); },
        }
    }

    for node in graph {
        for edge in &node.borrow().edges {
            // TODO (GM): Does this actually work?

            // Easier to calculate both positions here than to check which
            //  one is which.
            let i = *pos_lookup.get(&edge.a.borrow().val).unwrap();
            let j = *pos_lookup.get(&edge.b.borrow().val).unwrap();

            // Currently we only support undirected edges
            admat[i][j] = edge.weight;
            admat[j][i] = edge.weight;
        }
    }

    admat
}

pub fn print_admat(admat: &AdMat) {
    for row in admat {
        for x in row {
            print!("{x}");
        }

        println!("");
    }

    println!();
}
