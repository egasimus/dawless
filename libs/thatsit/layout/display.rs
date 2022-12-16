use crate::*;

impl<'a> std::fmt::Debug for Layout<'a> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Layout::{}", match self {
            Self::None        => "None",
            Self::Blank(_)    => "Blank",
            Self::Item(_,_)   => "Item",
            Self::Layers(_,_) => "Layers",
            Self::Column(_,_) => "Column",
            Self::Row(_,_)    => "Row",
            Self::Grid(_,_)   => "Grid"
        })
    }
}

impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let Size { min: Point(min_w, min_h), max: Point(max_w, max_h) } = self;
        write!(f, "[between {min_w:?}x{min_h:?} and {max_w:?}x{max_h:?}]")
    }
}
