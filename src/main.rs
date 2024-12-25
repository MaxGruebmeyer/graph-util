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
type Graph<V, E> = Vec<Rc<RefCell<Node<V, E>>>>;

// TODO (GM): Because I'm lazy!
type DNode = Node<Value, Weight>;
type DEdge = Edge<Value, Weight>;
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
    fn new(val: V) -> Rc<RefCell<Node<V, W>>> {
        Rc::new(RefCell::new(Node {
            val,
            edges: Vec::new(),
        }))
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
    fn new(a: &Rc<RefCell<Node<V, W>>>, b: &Rc<RefCell<Node<V, W>>>, weight: W) -> Rc<Edge<V, W>> {
        let edge = Rc::new(Edge {
            // TODO (GM): Is cloning really the way?
            a: a.clone(),
            b: b.clone(),
            weight,
        });

        // TODO (GM): Again, is cloning the way?
        a.borrow_mut().edges.push(edge.clone());
        b.borrow_mut().edges.push(edge.clone());

        edge
    }
}

// TODO (GM): Test these methods!
/// Generates a bidirectional path with size n
fn generate_path(n: usize) -> DGraph {
    let mut nodes: DGraph = Vec::new();
    if n == 0 {
        return nodes;
    }

    nodes.push(DNode::new(0));
    for i in 1..n {
        let mut cur = DNode::new(i.try_into().unwrap());
        let last = nodes.last_mut().unwrap();

        DEdge::new(&cur, last, DEFAULT_EDGE_WEIGHT);
    }

    nodes
}

/// Generates a bidirectional circle with size n
fn generate_circle(n: usize) -> DGraph {
    let mut nodes = generate_path(n);
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
