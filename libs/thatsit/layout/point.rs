use super::*;

#[derive(Copy, Clone, Default, Debug)]
pub struct Point(pub Unit, pub Unit);

impl Point {
    pub fn null () -> Self {
        Self(0, 0)
    }
    pub fn clip (self, other: Self) -> Self {
        Self(self.0.min(other.0), self.1.min(other.1))
    }
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add (self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl From<(u16, u16)> for Point {
    fn from ((a, b): (u16, u16)) -> Self {
        Self(a, b)
    }
}
