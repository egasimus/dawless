use super::{*, super::*};

#[derive(Copy, Clone, Default, Debug)]
pub struct Size {
    pub min_w: Option<Unit>,
    pub min_h: Option<Unit>,
    pub max_w: Option<Unit>,
    pub max_h: Option<Unit>
}

impl Size {
    pub fn from_fixed (Point(w, h): Point) -> Self {
        Self { min_w: Some(w), max_w: Some(w), min_h: Some(h), max_h: Some(h) }
    }
    pub fn add_h (&self, other: Self) -> Self {
        let min_h = add_min(self.min_h, other.min_h);
        let max_h = add_max(self.max_h, other.max_h);
        Self { min_h, max_h, ..*self }
    }
    pub fn add_w (&self, other: Self) -> Self {
        let min_w = add_min(self.min_w, other.min_w);
        let max_w = add_max(self.max_w, other.max_w);
        Self { min_w, max_w, ..*self }
    }
    pub fn add_wh (self, other: Self) -> Self {
        let min_w = add_min(self.min_w, other.min_w);
        let max_w = add_max(self.max_w, other.max_w);
        let min_h = add_min(self.min_h, other.min_h);
        let max_h = add_max(self.max_h, other.max_h);
        Self { min_w, max_w, min_h, max_h }
    }
    pub fn inc_w (self, w: Unit) -> Self {
        let min_w = add_min(self.min_w, Some(w));
        let max_w = add_max(self.max_w, Some(w));
        Self { min_w, max_w, ..self }
    }
    pub fn inc_h (self, h: Unit) -> Self {
        let min_h = add_min(self.min_h, Some(h));
        let max_h = add_max(self.max_h, Some(h));
        Self { min_h, max_h, ..self }
    }
    pub fn clip (self, Point(mut w, mut h): Point) -> Result<Point> {
        if let Some(min_w) = self.min_w && w < min_w {
            return Err(Error::new(ErrorKind::Other, "too narrow"))
        }
        if let Some(min_h) = self.min_h && h < min_h {
            return Err(Error::new(ErrorKind::Other, "too short"))
        }
        if let Some(max_w) = self.max_w && w > max_w {
            w = max_w
        }
        if let Some(max_h) = self.max_h && h > max_h {
            h = max_h
        }
        Ok(Point(w, h))
    }
    pub fn min (self) -> Point {
        Point(
            match self.min_w { Some(w) => w, None => 0 },
            match self.min_h { Some(h) => h, None => 0 },
        )
    }
    pub fn max (self) -> Point {
        Point(
            match self.max_w { Some(w) => w, None => Unit::MAX },
            match self.max_h { Some(h) => h, None => Unit::MAX },
        )
    }
}

pub fn add_min (a: Option<Unit>, b: Option<Unit>) -> Option<Unit> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a + b),
        (Some(a), None)    => Some(a),
        (None,    Some(b)) => Some(b),
        (None,    None)    => None
    }
}

pub fn add_max (a: Option<Unit>, b: Option<Unit>) -> Option<Unit> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a + b),
        (Some(_), None)    => None,
        (None,    Some(_)) => None,
        (None,    None)    => None
    }
}
