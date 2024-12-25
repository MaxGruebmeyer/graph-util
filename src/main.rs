mod datastructures;
use crate::datastructures::*;

mod admat;
use crate::admat::*;

mod generators;
use crate::generators::*;

fn main() {
    println!("Hello world!");

    // TODO (GM): Visualize these!

    let filename = "admat.txt";
    let admat = read_admat(&filename);

    print_admat(&admat);
    println!();
    print_admat(&graph_to_admat(admat_to_graph(&admat)));
    println!();

    let p5 = generate_path(5);
    print_admat(&graph_to_admat(p5));
    println!();

    let c3 = generate_circle(3);
    print_admat(&graph_to_admat(c3));
    println!();

    println!("Goodbye, cruel world!");
}
