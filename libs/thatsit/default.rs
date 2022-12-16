use crate::*;

impl<'a> Default for &'a dyn TUI {
    fn default () -> Self {
        &Blank {}
    }
}

impl Default for Box<dyn TUI> {
    fn default () -> Self {
        Box::new(Blank {})
    }
}

impl Default for Sizing {
    fn default () -> Self {
        Self::Grow(1)
    }
}

impl<'a> Default for Layout<'a> {
    fn default () -> Self {
        Self::Item(Sizing::Fixed(Area::MIN), &Blank {})
    }
}
