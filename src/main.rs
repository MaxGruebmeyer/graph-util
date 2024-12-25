extern crate regex;

use std::{
    cell::RefCell,
    fs::File,
    io::BufRead,
    io::BufReader,
    rc::Rc
};

use regex::Regex;

type Value = i32;
type Weight = i8;
type Graph<V, E> = Vec<Node<V, E>>;

// TODO (GM): Because I'm lazy!
type DGraphNode = Node<Value, Weight>;
type DGraph = Graph<Value, Weight>;
type AdMat = Vec<Vec<Weight>>;

const DEFAULT_EDGE_WEIGHT: Weight = 1;

// TODO (GM): Move all that shit into its own file!
#[derive(Clone)]
struct Node<V, W> where V: Clone, W: Clone {
    val: V,
    // TODO (GM): Do some reading regarding Rc!
    // TODO (GM): Think about thread-safety!
    edges: Vec<Rc<Edge<V, W>>>,
}

impl<V: std::clone::Clone, W: std::clone::Clone> Node<V, W> {
    fn new(val: V) -> Node<V, W> {
        Node {
            val,
            edges: Vec::new(),
        }
    }

    fn get_deg(&self) -> usize {
        self.edges.len()
    }
}

// TODO (GM): Implement UnidirectionalEdge
/// Represents a bidirectional edge connecting a to b.
#[derive(Clone)]
struct Edge<V, W> where V: Clone, W: Clone {
    a: Rc<RefCell<Node<V, W>>>,
    b: Rc<RefCell<Node<V, W>>>,
    weight: W,
}

impl<V: std::clone::Clone, W: std::clone::Clone> Edge<V, W> {
    // TODO (GM): Is there an option to provide a default value for weight?
    fn new(a: &mut Node<V, W>, b: &mut Node<V, W>, weight: W) -> Edge<V, W> {
        let edge = Edge {
            // TODO (GM): Is cloning really the way?
            a: Rc::new(RefCell::new(a.clone())),
            b: Rc::new(RefCell::new(b.clone())),
            weight,
        };

        a.edges.push(edge.clone().into());
        b.edges.push(edge.clone().into());
        edge
    }
}

// TODO (GM): For now: incoming = outgoing!

// TODO (GM): Fix!
// /// Generates a bidirectional path with size n
// fn generate_path(n: usize) -> DGraph {
//     let mut nodes: DGraph = Vec::new();
//     if n == 0 {
//         return nodes;
//     }
// 
//     nodes.push(DGraphNode::new(0));
//     for i in 1..n {
//         let mut cur = DGraphNode::new(i.try_into().unwrap());
//         let last: &mut DGraphNode = nodes.last_mut().unwrap();
// 
//         // TODO (GM): There has to be a better way!
//         last.next.push((DEFAULT_EDGE_WEIGHT, cur.clone().into()));
//         cur.next.push((DEFAULT_EDGE_WEIGHT, (*last).clone().into()));
//     }
// 
//     nodes
// }
// 
// /// Generates a bidirectional circle with size n
// fn generate_circle(n: usize) -> DGraph {
//     let mut nodes = generate_path(n);
//     if n < 2 {
//         return nodes;
//     }
// 
//     let (first, rest) = nodes.split_first_mut().unwrap();
//     let last = rest.last_mut().unwrap();
// 
//     first.next.push((DEFAULT_EDGE_WEIGHT, (*last).clone().into()));
//     last.next.push((DEFAULT_EDGE_WEIGHT, (*first).clone().into()));
//     nodes
// }

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

    println!("Goodbye, cruel world!");
}
