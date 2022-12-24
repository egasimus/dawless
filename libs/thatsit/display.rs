use crate::*;
use std::fmt::{Debug, Display};

impl Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.1, self.0)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let Point(x, y) = self;
        write!(f, "+{x}+{y}")
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let Size(w, h) = self;
        write!(f, "{w}x{h}")
    }
}

impl Debug for &mut dyn TUI {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}-{}|mut)", self.min_size(), self.max_size())
    }
}

impl Debug for &dyn TUI {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}-{})", self.min_size(), self.max_size())
    }
}

impl Debug for Box<dyn TUI> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}-{})", self.min_size(), self.max_size())
    }
}

impl<'a> Debug for Thunk<'a> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(thunk: {} items, min {})", self.items.len(), self.min_size)
    }
}
