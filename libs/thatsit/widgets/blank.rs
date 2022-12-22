use crate::*;

#[derive(Debug, Default)]
/// An empty widget
pub struct Blank;

/// An instance of the empty widget
pub const BLANK: &'static Blank = &Blank;

impl TUI for Blank {
    fn min_size (&self) -> Size {
        Size::MIN
    }
    fn max_size (&self) -> Size {
        Size::MAX
    }
    fn render (&self, _: &mut dyn Write, _: Area) -> Result<()> {
        Ok(())
    }
}
