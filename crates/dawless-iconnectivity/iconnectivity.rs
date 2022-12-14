use dawless_common::*;

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
impl TUI for MioXLTUI {
    fn size (&self) -> Size {
        self.menu.size()
    }
    fn render (&self, term: &mut dyn Write, space: &Space) -> Result<()> {
        Frame { title: "MioXL".into(), ..Frame::default() }
            .render(term, space)?;
        self.menu.render(term, &space.inset(1))
    }
}
