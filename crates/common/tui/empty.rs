use super::*;

pub struct EmptyTUI {}

impl TUI for EmptyTUI {
    fn render (&self, term: &mut dyn Write) -> Result<()> {
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
