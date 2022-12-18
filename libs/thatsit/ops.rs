use crate::*;

impl std::ops::Add for Point {
    type Output = Self;
    fn add (self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl From<(Unit, Unit)> for Point {
    fn from ((a, b): (u16, u16)) -> Self { Self(a, b) }
}

impl From<(Unit, Unit)> for Size {
    fn from ((w, h): (Unit, Unit)) -> Self { Self(w, h) }
}
