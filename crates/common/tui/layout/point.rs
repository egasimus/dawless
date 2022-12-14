use super::{*, super::*};

#[derive(Copy, Clone, Default, Debug)]
pub struct Point(pub Unit, pub Unit);

impl Point {
    pub fn null () -> Self {
        Self(0, 0)
    }
}

impl From<(u16, u16)> for Point {
    fn from ((a, b): (u16, u16)) -> Self {
        Self(a, b)
    }
}
