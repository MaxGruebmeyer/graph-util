extern crate regex;

use std::{
    cell::RefCell,
    collections::HashMap,
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
// TODO (GM): Documentation!
#[derive(Clone)]
struct Node<V, W> where V: Clone, W: Clone {
    val: V,
    // TODO (GM): Do some reading regarding Rc!
    // TODO (GM): Think about thread-safety!
    edges: Vec<Rc<Edge<V, W>>>,

    /// Node color. Can be used for different algorithms, e.g. DFS
    color: i8,
}

impl<V: std::clone::Clone, W: std::clone::Clone> Node<V, W> {
    fn new(val: V) -> Rc<RefCell<Node<V, W>>> {
        Rc::new(RefCell::new(Node {
            val,
            edges: Vec::new(),
            color: 0
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
