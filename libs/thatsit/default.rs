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

impl<'a> Default for Sizing<'a> {
    fn default () -> Self {
        Self::AUTO
    }
}

impl<'a> Default for Layout<'a> {
    fn default () -> Self {
        Self::Item(Sizing::Fixed(Size::MIN), &Blank {})
    }
}
