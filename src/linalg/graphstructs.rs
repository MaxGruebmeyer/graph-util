use crate::linalg::structs::*;

use std::{
    collections::HashMap,
    hash::Hash,
};

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct EdgeId<V> where V: Clone, V: Eq, V: Hash {
    pub a: V,
    pub b: V,
}

pub struct RenderInfo<V> where V: Clone, V: Eq, V: Hash {
    pub pos_info: HashMap<V, Pos3D>,
    pub charge_info: HashMap<V, f64>,
    pub rod_info: HashMap<EdgeId<V>, f64>,
}
