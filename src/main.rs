use std::{
    convert::TryInto,
    rc::Rc,
};

type Values = i32;
type Weights = i8;
type Graph<V, E> = Vec<GraphNode<V, E>>;

// TODO (GM): Because I'm lazy!
type DGraphNode = GraphNode<Values, Weights>;
type DGraph = Graph<Values, Weights>;

const DEFAULT_EDGE_WEIGHT: Weights = 1;

// TODO (GM): Multiple structs for weighted, unweighted, directed, undirected graphs!
/// Struct representing a graph node with value of type V and
///  edge weight of type E.
#[derive(Clone)]
struct GraphNode<V, E> {
    /// The value of the node
    val: V,

    // TODO (GM): Do we also need incoming?
    /// Outgoing connections
    next: Vec<(E, Rc<GraphNode<V, E>>)>,

    /// Bool signaling whether this node was visited before.
    /// Useful for some algorithms.
    /// It can be changed at any times, however no assumptions should be
    /// made on this field before or after running an algorithm.
    // TODO (GM): Make assumptions?
    was_visited: bool,
}

impl<V,E> GraphNode<V, E> {
    fn new(val: V) -> GraphNode<V, E> {
        GraphNode {
            val,
            was_visited: false,
            next: Vec::new(),
        }
    }
}

// TODO (GM): For now: incoming = outgoing!

/// Generates a bidirectional path with size n
fn generate_path(n: usize) -> DGraph {
    let mut nodes: DGraph = Vec::new();
    if n == 0 {
        return nodes;
    }

    nodes.push(DGraphNode::new(0));
    for i in 1..n {
        let mut cur = DGraphNode::new(i.try_into().unwrap());
        let last: &mut DGraphNode = nodes.last_mut().unwrap();

        // TODO (GM): There has to be a better way!
        last.next.push((DEFAULT_EDGE_WEIGHT, cur.clone().into()));
        cur.next.push((DEFAULT_EDGE_WEIGHT, (*last).clone().into()));
    }

    nodes
}

/// Generates a bidirectional circle with size n
fn generate_circle(n: usize) -> DGraph {
    let mut nodes = generate_path(n);
    if n < 2 {
        return nodes;
    }

    let (first, rest) = nodes.split_first_mut().unwrap();
    let last = rest.last_mut().unwrap();

    first.next.push((DEFAULT_EDGE_WEIGHT, (*last).clone().into()));
    last.next.push((DEFAULT_EDGE_WEIGHT, (*first).clone().into()));
    nodes
}

// TODO (GM): Other graph generation functions (complete, hypercube, ...)!

fn main() {
    println!("Hello world!");

    // TODO (GM): Column-oriented or row-oriented?
    // TODO (GM): Also allow 1d-adj_matrix?
    // TODO (GM): Structs for nodes + methods to build adjacency-matrix from it!
    let adj_matrix: Vec<Vec<Weights>> = Vec::new();

    // TODO (GM): Visualize these!
    let _p5 = generate_path(5);
    let _c3 = generate_circle(3);

    println!("Goodbye, cruel world!");
}
