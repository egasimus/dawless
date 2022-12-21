use crate::*;

impl<'a> Default for &'a dyn TUI<'a> {
    fn default () -> Self {
        &Blank {}
    }
}

impl<'a> Default for Box<dyn TUI<'a>> {
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
