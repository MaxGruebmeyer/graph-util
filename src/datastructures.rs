use std::{
    cell::RefCell,
    rc::Rc,
};

pub type Value = i32;
pub type Weight = i8;
pub type Graph<V, E> = Vec<Rc<RefCell<Node<V, E>>>>;

// TODO (GM): Because I'm lazy!
pub type DNode = Node<Value, Weight>;
pub type DEdge = Edge<Value, Weight>;
pub type DGraph = Graph<Value, Weight>;
pub type AdMat = Vec<Vec<Weight>>;

pub const DEFAULT_EDGE_WEIGHT: Weight = 1;

// TODO (GM): Move all that shit into its own file!
// TODO (GM): Documentation!
#[derive(Clone)]
pub struct Node<V, W> where V: Clone, W: Clone {
    pub val: V,
    // TODO (GM): Do some reading regarding Rc!
    // TODO (GM): Think about thread-safety!
    pub edges: Vec<Rc<Edge<V, W>>>,

    /// Node color. Can be used for different algorithms, e.g. DFS
    pub color: i8,
}

impl<V: std::clone::Clone, W: std::clone::Clone> Node<V, W> {
    pub fn new(val: V) -> Rc<RefCell<Node<V, W>>> {
        Rc::new(RefCell::new(Node {
            val,
            edges: Vec::new(),
            color: 0
        }))
    }

    pub fn get_deg(&self) -> usize {
        self.edges.len()
    }
}

// TODO (GM): Implement UnidirectionalEdge
/// Represents a bidirectional edge connecting a to b.
#[derive(Clone)]
pub struct Edge<V, W> where V: Clone, W: Clone {
    pub a: Rc<RefCell<Node<V, W>>>,
    pub b: Rc<RefCell<Node<V, W>>>,
    pub weight: W,
}

impl<V: std::clone::Clone, W: std::clone::Clone> Edge<V, W> {
    // TODO (GM): Is there an option to provide a default value for weight?
    pub fn new(a: &Rc<RefCell<Node<V, W>>>, b: &Rc<RefCell<Node<V, W>>>, weight: W) -> Rc<Edge<V, W>> {
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
