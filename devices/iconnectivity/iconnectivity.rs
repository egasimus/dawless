use thatsit::*;
use thatsit_focus::*;

pub struct MioXLTUI {
    menu: FocusColumn<Box<dyn TUI>>
}

impl MioXLTUI {
    pub fn new () -> Self {
        let mut menu = FocusColumn::default();
        //menu.add("Edit preset".into(), ())
            //.add("Route MIDI".into(),  ());
        Self { menu }
    }
}

impl TUI for MioXLTUI {
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        self.menu.render(term, area)
    }
}
