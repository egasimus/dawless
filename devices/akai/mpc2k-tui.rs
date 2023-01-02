use thatsit::*;
use thatsit_focus::*;

pub struct MPC2000TUI<'a> {
    menu: FocusStack<'a>
}

impl<'a> MPC2000TUI<'a> {
    pub fn new () -> Self {
        let mut menu = FocusStack::default();
        //menu.add("Edit sample".into(), ());
        Self { menu }
    }
}

impl<'a> Render for MPC2000TUI<'a> {
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        self.menu.render(term, area)
    }
}
