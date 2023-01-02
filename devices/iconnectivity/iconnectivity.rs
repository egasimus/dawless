use thatsit::*;
use thatsit_focus::*;

pub struct MioXLUI<'a> {
    menu: FocusStack<'a>
}

impl<'a> MioXLUI<'a> {
    pub fn new () -> Self {
        let mut menu = FocusStack::default();
        //menu.add("Edit preset".into(), ())
            //.add("Route MIDI".into(),  ());
        Self { menu }
    }
}

impl<'a> Render for MioXLUI<'a> {
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        self.menu.render(term, area)
    }
}
