extern crate regex;

mod datastructures;
use crate::datastructures::*;

use std::{
    collections::HashMap,
    fs::File,
    io::BufRead,
    io::BufReader,
};

use regex::Regex;

// TODO (GM): Test these methods!
/// Generates a bidirectional path with size n
fn generate_path(n: usize) -> DGraph {
    let mut nodes: DGraph = Vec::new();
    if n == 0 {
        return nodes;
    }

    nodes.push(DNode::new(0));
    for i in 1..n {
        let cur = DNode::new(i.try_into().unwrap());
        let last = nodes.last_mut().unwrap();

        DEdge::new(&cur, last, DEFAULT_EDGE_WEIGHT);
    }

    nodes
}

/// Generates a bidirectional circle with size n
fn generate_circle(n: usize) -> DGraph {
    let nodes = generate_path(n);
    if n < 2 {
        return nodes;
    }

    let (first, rest) = nodes.split_first().unwrap();
    let last = rest.last().unwrap();

    DEdge::new(first, last, DEFAULT_EDGE_WEIGHT);

    nodes
}

// TODO (GM): Other graph generation functions (complete, hypercube, ...)!

fn print_admat(admat: &AdMat) {
    for row in admat {
        for x in row {
            print!("{x}");
        }

        println!("");
    }
}

fn read_admat(filename: &str) -> AdMat {
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
fn admat_to_graph(admat: &AdMat) -> DGraph {
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
fn graph_to_admat(graph: DGraph) -> AdMat {
    let mut admat: AdMat = vec![vec![0; graph.len()]; graph.len()];

    // HashMap to map values to positions.
    // Assumes values are unique among nodes.
    let mut pos_lookup: HashMap<Value, usize> = HashMap::new();

    for i in 0..graph.len() {
        match pos_lookup.insert(graph[i].borrow().val, i) {
            None => {},
            Some(x) => { panic!("Node values were not unique, {x} occurred multiple times!"); },
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

fn main() {
    println!("Hello world!");

    // TODO (GM): Column-oriented or row-oriented?
    // TODO (GM): Also allow 1d-adj_matrix?
    // TODO (GM): Structs for nodes + methods to build adjacency-matrix from it!
    let adj_matrix: Vec<Vec<Weight>> = Vec::new();

    // TODO (GM): Visualize these!
    // TODO (GM): Fix them + unit tests!
    // let _p5 = generate_path(5);
    // let _c3 = generate_circle(3);

    let filename = "admat.txt";
    let admat = read_admat(&filename);

    print_admat(&admat);
    println!();
    print_admat(&graph_to_admat(admat_to_graph(&admat)));

    println!("Goodbye, cruel world!");
}
