use thatsit::*;

pub struct MioXLTUI {
    menu: List<()>
}
impl MioXLTUI {
    pub fn new () -> Self {
        let mut menu = List::default();
        menu.add("Edit preset".into(), ())
            .add("Route MIDI".into(),  ());
        Self { menu }
    }
}
impl<'a> TUI<'a> for MioXLTUI {
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        self.menu.render(term, area)
    }
}
