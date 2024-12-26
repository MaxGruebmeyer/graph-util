// TODO (GM): Can we merge multiple files into one module?
mod datastructures;

mod admat;
use crate::admat::*;

mod generators;
use crate::generators::*;

mod visualization;
use crate::visualization::visualization::*;

mod linalg;

fn main() {
    let filename = "../src/resources/admat.txt";
    let admat = read_admat(&filename);
    let p4 = generate_path(4);
    let c7 = generate_circle(7);

    print_admat(&admat);
    print_admat(&graph_to_admat(&p4));
    print_admat(&graph_to_admat(&c7));

    visualize(&p4);
}
