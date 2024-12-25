mod datastructures;
use crate::datastructures::*;

mod admat;
use crate::admat::*;

mod generators;
use crate::generators::*;

fn main() {
    let filename = "admat.txt";
    let admat = read_admat(&filename);

    print_admat(&admat);
    print_admat(&graph_to_admat(generate_path(4)));
    print_admat(&graph_to_admat(generate_circle(7)));

    // TODO (GM): Work on visualization!
}
