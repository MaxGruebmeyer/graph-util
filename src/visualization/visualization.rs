use crate::datastructures::*;
use crate::linalg::conversion::*;
use crate::linalg::graphphysics::*;
use crate::linalg::graphstructs::*;

use std::hash::Hash;

// TODO (GM): Documentation!
pub fn visualize<V: Clone, E: Clone>(graph: &Graph<V, E>) where V: Eq, V: Hash {
    let mut render_info = RenderInfo {
        pos_info: init_pos_map(&graph),
        charge_info: init_charge_map(&graph),
        rod_info: init_rod_map(&graph),
    };

    calc_til_stable(&mut render_info);

    println!("Final coordiantes are:");
    for (_, v) in render_info.pos_info {
        println!("x = {}, y = {}, z = {}", v.x, v.y, v.z);
    }
}
