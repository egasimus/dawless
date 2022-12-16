use super::super::{*, layout::*};

pub struct Blank {}

impl TUI for Blank {
    fn min_size (&self) -> Area {
        Area::MIN
    }
    fn max_size (&self) -> Area {
        Area::MAX
    }
    fn render (&self, _term: &mut dyn Write, _area: Area) -> Result<()> {
        Ok(())
    }
}
