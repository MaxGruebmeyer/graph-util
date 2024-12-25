use crate::datastructures::*;

// TODO (GM): Unit test these methods!
/// Generates a bidirectional path with size n
pub fn generate_path(n: usize) -> DGraph {
    let mut nodes: DGraph = Vec::new();
    if n == 0 {
        return nodes;
    }

    nodes.push(DNode::new(0));
    for i in 1..n {
        let cur = DNode::new(i.try_into().unwrap());
        let last = nodes.last_mut().unwrap();

        DEdge::new(&cur, last, DEFAULT_EDGE_WEIGHT);
        nodes.push(cur);
    }

    nodes
}

/// Generates a bidirectional circle with size n
pub fn generate_circle(n: usize) -> DGraph {
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
