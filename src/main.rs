mod datastructures;
use crate::datastructures::*;

mod admat;
use crate::admat::*;

mod generators;
use crate::generators::*;

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
