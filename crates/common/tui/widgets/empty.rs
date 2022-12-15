use super::{*, super::{*, layout::*}};

pub struct EmptyTUI {}

impl TUI for EmptyTUI {
    fn render (&self, _term: &mut dyn Write, _space: &Space) -> Result<()> {
        Ok(())
    }
}

impl<'a> Default for &'a dyn TUI {
    fn default () -> Self {
        &EmptyTUI {}
    }
}

impl Default for Box<dyn TUI> {
    fn default () -> Self {
        Box::new(EmptyTUI {})
    }
}
