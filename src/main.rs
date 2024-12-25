mod datastructures;
use crate::datastructures::*;

mod admat;
use crate::admat::*;

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

fn main() {
    println!("Hello world!");

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
