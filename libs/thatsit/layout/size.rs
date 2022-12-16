use super::{*, super::*};

#[derive(Copy, Clone, Default, Debug, PartialEq)]
/// A pair of coordinates
pub struct Point(pub Unit, pub Unit);

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

impl Point {
    pub const NUL: Self = Self(0, 0);
    pub const MIN: Self = Self(Unit::MIN, Unit::MIN);
    pub const MAX: Self = Self(Unit::MAX, Unit::MAX);
    pub fn clip (self, other: Self) -> Self {
        Self(self.0.min(other.0), self.1.min(other.1))
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
/// A range of sizes
pub struct Size {
    /// The minimum allowable size
    pub min: Point,
    /// The maximum allowable size
    pub max: Point
}

impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let Size { min: Point(min_w, min_h), max: Point(max_w, max_h) } = self;
        write!(f, "[between {min_w:?}x{min_h:?} and {max_w:?}x{max_h:?}]")
    }
}

impl Size {

    /// Any size between minimum and maximum
    pub const ANY: Self = Self { min: Point::MIN, max: Point::MAX };

    /// The minimum size
    pub const MIN: Self = Self { min: Point::MIN, max: Point::MIN };

    /// The maximum size
    pub const MAX: Self = Self { min: Point::MAX, max: Point::MAX };

    /// Create a fixed size
    pub fn fixed (size: Point) -> Self {
        Self { min: size, max: size }
    }

    /// Increment size to fit other
    pub fn stretch (self, other: Size) -> Self {
        let Size { min: Point(old_min_x, old_min_y), max: Point(old_max_x, old_max_y) } = self;
        let Size { min: Point(new_min_x, new_min_y), max: Point(new_max_x, new_max_y) } = other;
        Self {
            min: Point(old_min_x.max(new_min_x), old_min_y.max(new_min_y)),
            max: Point(old_max_x.max(new_max_x), old_max_y.max(new_max_y))
        }
    }
    /// Stretch width and append height
    pub fn add_to_column (self, other: Size) -> Self {
        let Size { min: Point(old_min_x, old_min_y), max: Point(old_max_x, old_max_y) } = self;
        let Size { min: Point(new_min_x, new_min_y), max: Point(new_max_x, new_max_y) } = other;
        Self {
            min: Point(old_min_x.max(new_min_x), old_min_y.saturating_add(new_min_y)),
            max: Point(old_max_x.max(new_max_x), old_max_y.saturating_add(new_max_y))
        }
    }
    /// Stretch height and append width
    pub fn add_to_row (self, other: Size) -> Self {
        let Size { min: Point(old_min_x, old_min_y), max: Point(old_max_x, old_max_y) } = self;
        let Size { min: Point(new_min_x, new_min_y), max: Point(new_max_x, new_max_y) } = other;
        Self {
            min: Point(old_min_x.saturating_add(new_min_x), old_min_y.max(new_min_y)),
            max: Point(old_max_x.saturating_add(new_max_x), old_max_y.max(new_max_y))
        }
    }

    pub fn clip (self, Point(mut w, mut h): Point) -> Result<Point> {
        let Size { min: Point(min_w, min_h), max: Point(max_w, max_h) } = self;
        if w < min_w {
            let message = format!("too small: {}x{} < {}x{}", w, h, min_w, min_h);
            return Err(Error::new(ErrorKind::Other, message))
        }
        if h < min_h {
            let message = format!("too small: {}x{} < {}x{}", w, h, min_w, min_h);
            return Err(Error::new(ErrorKind::Other, message))
        }
        if w > max_w {
            w = max_w
        }
        if h > max_h {
            h = max_h
        }
        Ok(Point(w, h))
    }

    pub fn resolve (self, sizing: &Sizing) -> Self {
        match sizing {
            Sizing::Auto          => self,
            Sizing::Min           => Size::fixed(self.min),
            Sizing::Max           => Size::fixed(self.max),
            Sizing::Fixed(point)  => Size::fixed(*point),
            Sizing::Stretch(size) => *size
        }
    }

}

pub fn add_min (a: Option<Unit>, b: Option<Unit>) -> Option<Unit> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a.saturating_add(b)),
        (Some(a), None)    => Some(a),
        (None,    Some(b)) => Some(b),
        (None,    None)    => None
    }
}

pub fn add_max (a: Option<Unit>, b: Option<Unit>) -> Option<Unit> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a.saturating_add(b)),
        (Some(_), None)    => None,
        (None,    Some(_)) => None,
        (None,    None)    => None
    }
}
