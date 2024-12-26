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
    let path = generate_path(14);
    let circle = generate_circle(21);

    print_admat(&admat);
    print_admat(&graph_to_admat(&path));
    print_admat(&graph_to_admat(&circle));

    // TODO (GM): Fix this for circles and general graphs!
    // visualize(&circle);

    visualize(&path);
}
