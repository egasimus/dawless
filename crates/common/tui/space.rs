use super::*;

#[derive(Default, Debug, Copy, Clone)]
pub struct Space {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

impl Space {
    pub fn new (x: u16, y: u16, w: u16, h: u16) -> Self {
        Self { x, y, w, h }
    }
    /** Return the center of the space. */
    pub fn center (&self) -> (u16, u16) {
        (self.x + self.w/2, self.y + self.h/2)
    }

    /** Add/subtract from each value. */
    pub fn add (&self, dx: i16, dy: i16, dw: i16, dh: i16) -> Self {
        Self {
            x: (self.x as i16 + dx) as u16,
            y: (self.y as i16 + dy) as u16,
            w: (self.w as i16 + dw) as u16,
            h: (self.h as i16 + dh) as u16
        }
    }

    /** Return part of the space.
      * Positive x and y coordinates are offsets from top left.
      * Negative x and y coordinates are offsets from bottom right.
      * Zero w and h inherit the width and height from the parent.
      * Positive w and h are literal width and height.
      * Negative w and h are subtracted from parent width and height. */
    pub fn sub (&self, dx: i16, dy: i16, dw: i16, dh: i16) -> Self {
        let Self { x, y, w, h } = *self;
        Self {
            x: if dx >= 0 { x + dx as u16 } else { x + w - dx as u16 },
            y: if dy >= 0 { y + dy as u16 } else { y + w - dy as u16 },
            w: if dw > 0 { dw as u16 } else if dw < 0 { w - (- dw) as u16 } else { w },
            h: if dh > 0 { dh as u16 } else if dh < 0 { h - ( -dh) as u16 } else { h },
        }
    }

    pub fn size (&self, w: u16, h: u16) -> Self {
        let x = self.x;
        let y = self.y;
        Self { x, y, w, h }
    }

    pub fn inset (&self, d: u16) -> Self {
        Self {
            x: self.x + d,
            y: self.y + d,
            w: self.w - d - d,
            h: self.h - d - d,
        }
    }

    pub fn inset_w (&self, d: u16) -> Self {
        Self {
            x: self.x + d,
            y: self.y,
            w: self.w - d - d,
            h: self.h,
        }
    }

    pub fn inset_h (&self, d: u16) -> Self {
        Self {
            x: self.x,
            y: self.y + d,
            w: self.w,
            h: self.h - d - d,
        }
    }

    pub fn offset (&self, dx: u16, dy: u16) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
            w: self.w,
            h: self.h
        }
    }

    pub fn clip (&self, dw: u16, dh: u16) -> Self {
        Self {
            x: self.x,
            y: self.y,
            w: self.w - dw,
            h: self.h - dh
        }
    }

    pub fn right (&self, distance: u16) -> Self {
        let x = self.x + self.w + distance;
        let y = self.y;
        Self { x, y, w: 0, h: 0 }
    }

    pub fn below (&self, distance: u16) -> Self {
        let x = self.x;
        let y = self.y + self.h + distance;
        Self { x, y, w: 0, h: 0 }
    }

    pub fn join (&self, space: &Space) -> Self {
        let x = self.x.min(space.x);
        let y = self.y.min(space.y);
        let x2 = (self.x + self.w).max(space.x + space.w);
        let y2 = (self.y + self.h).max(space.y + space.h);
        let w = x2 - x;
        let h = y2 - y;
        Self { x, y, w, h }
    }

}
