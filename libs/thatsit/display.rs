use crate::*;
use std::fmt::{Debug, Display};

impl<'a> Debug for Layout<'a> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Layout::{}", match self {
            Self::Item(_,_)   => "Item",
            Self::Layers(_,_) => "Layers",
            Self::Column(_,_) => "Column",
            Self::Row(_,_)    => "Row",
            Self::Grid(_,_)   => "Grid"
        })
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let Point(x, y) = self;
        write!(f, "+{x}+{y}")
    }
}

impl Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let Area(w, h) = self;
        write!(f, "{w}x{h}")
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let Size { min: Point(min_w, min_h), max: Point(max_w, max_h) } = self;
        write!(f, "[between {min_w:?}x{min_h:?} and {max_w:?}x{max_h:?}]")
    }
}

impl Debug for dyn TUI {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}-{})", self.min_size(), self.max_size())
    }
}
