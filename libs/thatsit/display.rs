use crate::*;
use std::fmt::{Debug, Display};

impl<'a> Debug for Layout<'a> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Layout::{}", match self {
            Self::Item(sizing, _) =>
                format!("Item({sizing:?})"),
            Self::Layers(_,_) =>
                String::from("Layers"),
            Self::Column(_,_) =>
                String::from("Column"),
            Self::Row(_,_) =>
                String::from("Row"),
            Self::Grid(_,_) =>
                String::from("Grid")
        })
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

impl<'a> Debug for &dyn TUI<'a> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}-{})", self.min_size(), self.max_size())
    }
}

impl<'a> Debug for Box<dyn TUI<'a>> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}-{})", self.min_size(), self.max_size())
    }
}
