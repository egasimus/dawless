use super::*;

#[derive(Default, Debug, Copy, Clone)]
pub struct Space(pub Point, pub Point);

impl Space {
    pub fn new (x: Unit, y: Unit, w: Unit, h: Unit) -> Self {
        Self(Point(x, y), Point(w, h))
    }
    /** Return the center of the space. */
    pub fn center (&self) -> (Unit, Unit) {
        let Self(Point(x, y), Point(w, h)) = self;
        (x + w/2, y + h/2)
    }
    /** Add/subtract from each value. */
    pub fn add (&self, dx: i16, dy: i16, dw: i16, dh: i16) -> Self {
        let Self(Point(x, y), Point(w, h)) = *self;
        let x = (x as i16 + dx) as Unit;
        let y = (y as i16 + dy) as Unit;
        let w = (w as i16 + dw) as Unit;
        let h = (h as i16 + dh) as Unit;
        Self(Point(x, y), Point(w, h))
    }
    /** Return part of the space.
      * Positive x and y coordinates are offsets from top left.
      * Negative x and y coordinates are offsets from bottom right.
      * Zero w and h inherit the width and height from the parent.
      * Positive w and h are literal width and height.
      * Negative w and h are subtracted from parent width and height. */
    pub fn sub (&self, dx: i16, dy: i16, dw: i16, dh: i16) -> Self {
        let Self(Point(x, y), Point(w, h)) = *self;
        let x = if dx >= 0 { x + dx as Unit } else { x + w - dx as Unit };
        let y = if dy >= 0 { y + dy as Unit } else { y + w - dy as Unit };
        let w = if dw > 0 { dw as Unit } else if dw < 0 { w - (- dw) as Unit } else { w };
        let h = if dh > 0 { dh as Unit } else if dh < 0 { h - ( -dh) as Unit } else { h };
        Self(Point(x, y), Point(w, h))
    }
    pub fn size (&self, w: Unit, h: Unit) -> Self {
        Self(self.0, Point(w, h))
    }
    pub fn inset (&self, d: Unit) -> Self {
        let Self(Point(x, y), Point(w, h)) = *self;
        Self(Point(x + d, y + d), Point(w.saturating_sub(2*d), h.saturating_sub(2*d)))
    }
    pub fn inset_w (&self, d: Unit) -> Self {
        let Self(Point(x, y), Point(w, h)) = *self;
        Self(Point(x + d, y), Point(w - 2*d, h))
    }
    pub fn inset_h (&self, d: Unit) -> Self {
        let Self(Point(x, y), Point(w, h)) = *self;
        Self(Point(x, y + d), Point(w, h - 2*d))
    }
    pub fn offset (&self, Point(dx, dy): Point) -> Self {
        let Self(Point(x, y), _) = *self;
        Self(Point(x + dx, y + dy), self.1)
    }
    pub fn clip (&self, Point(dw, dh): Point) -> Self {
        let Self(_, Point(w, h)) = *self;
        Self(self.0, Point(w - dw, h - dh))
    }
    pub fn right (&self, distance: Unit) -> Self {
        let Self(Point(x, y), Point(w, _)) = *self;
        Self(Point(x + w + distance, y), Point(0, 0))
    }
    pub fn below (&self, distance: Unit) -> Self {
        let Self(Point(x, y), Point(_, h)) = *self;
        Self(Point(x, y + h + distance), Point(0, 0))
    }
    pub fn join (&self, space: &Self) -> Self {
        let Self(Point(x,  y),  Point(w,  h)) = *self;
        let Self(Point(x2, y2), Point(w2, h2)) = *space;
        let x = x.min(x2);
        let y = y.min(y2);
        let x3 = (x + w).max(x2 + w2);
        let y3 = (y + h).max(y2 + h2);
        let w = x3 - x;
        let h = y3 - y;
        Self(Point(x, y), Point(w, h))
    }
}
