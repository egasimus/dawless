use crate::*;
use std::{ops::{Add, Sub}, fmt::{Debug, Display}};

/// The unit of the coordinate system
pub type Unit = u16;

/// A pair of coordinates
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Point (/** Column */ pub Unit, /** Row */ pub Unit);

impl Point {
    pub const NIL: Self = Self(0, 0);
    pub const MIN: Self = Self(Unit::MIN, Unit::MIN);
    pub const MAX: Self = Self(Unit::MAX, Unit::MAX);
    #[inline] pub fn x (self) -> Unit { self.0 }
    #[inline] pub fn y (self) -> Unit { self.1 }
    pub fn clip (self, other: Self) -> Self { Self(self.0.min(other.0), self.1.min(other.1)) }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "+{}+{}", self.0, self.1)
    }
}

impl Add for Point {
    type Output = Self;
    fn add (self, other: Self) -> Self { Self(self.0 + other.0, self.1 + other.1) }
}

impl Sub for Point {
    type Output = Self;
    fn sub (self, other: Self) -> Self { Self(self.0 - other.0, self.1 - other.1) }
}

impl From<(Unit, Unit)> for Point {
    fn from ((a, b): (u16, u16)) -> Self { Self(a, b) }
}

/// A pair of dimensions
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Size (/** Width */ pub Unit, /** Height */ pub Unit);

/// The width and height of a rectangle.
/// TODO implement layout as partitioning of `Size` objects.
impl Size {
    pub const MIN: Self = Self(0, 0);
    pub const MAX: Self = Self(Unit::MAX, Unit::MAX);
    #[inline] pub fn width  (self) -> Unit { self.0 }
    #[inline] pub fn height (self) -> Unit { self.1 }
    /// Increase own size to fit other
    #[inline] pub fn stretch (self, other: Self) -> Self {
        Self(self.0.max(other.0), self.1.max(other.1))
    }
    /// Grow width, stretch height
    #[inline] pub fn expand_row (self, other: Self) -> Self {
        Self(self.0.saturating_add(other.0), self.1.max(other.1))
    }
    /// Stretch width, grow height
    #[inline] pub fn expand_column (self, other: Self) -> Self {
        Self(self.0.max(other.0), self.1.saturating_add(other.1))
    }
    /// Limit the size to the other size
    #[inline] pub fn crop_to (self, other: Self) -> Self {
        Self(self.0.min(other.0), self.1.min(other.1))
    }
    /// Return error if the other area is too small
    pub fn at_least (self, other: Self) -> Result<()> {
        if self.0 > other.0 {
            let message = format!("need {} columns", self.0);
            return Err(Error::new(ErrorKind::Other, message))
        }
        if self.1 > other.1 {
            let message = format!("need {} rows", self.0);
            return Err(Error::new(ErrorKind::Other, message))
        }
        Ok(())
    }

    pub fn limit <'a> (self, max: Self, render: impl Fn(&mut dyn Write, Area)->Result<()>)
        -> Result<Layout<'a>>
    {
        if self.0 <= max.0 && self.1 <= max.1 {
            Ok(Layout(&render))
        } else {
            Err(Error::new(ErrorKind::Other, "not enough space"))
        }
    }

}

impl Add for Size {
    type Output = Self;
    fn add (self, other: Self) -> Self { Self(self.0 + other.0, self.1 + other.1) }
}

impl Sub for Size {
    type Output = Self;
    fn sub (self, other: Self) -> Self { Self(self.0 - other.0, self.1 - other.1) }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let Size(w, h) = self;
        write!(f, "{w}x{h}")
    }
}

impl From<(Unit, Unit)> for Size {
    fn from ((w, h): (Unit, Unit)) -> Self { Self(w, h) }
}

/// A rectangle, made of a `Point` and a `Size`
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Area (/** Position */ pub Point, /** Size */ pub Size);

/// A rectangle of the drawing area.
/// TODO implement layout as partitioning of `self.1` objects offset by `self.0`
impl Area {
    #[inline] pub fn x (self) -> Unit { self.0.x() }
    #[inline] pub fn y (self) -> Unit { self.0.y() }
    #[inline] pub fn width (self) -> Unit { self.1.width() }
    #[inline] pub fn height (self) -> Unit { self.1.height() }
}

impl Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.1, self.0)
    }
}
