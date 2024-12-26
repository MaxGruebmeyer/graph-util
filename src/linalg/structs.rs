// TODO (GM): Just use the vector struct? What about linalg libraries?
//  or write your own linalg library?
//  But find a way to generalize it!
#[derive(Clone)]
pub struct Pos2D {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone)]
pub struct Pos3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
