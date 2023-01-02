use thatsit::*;
use thatsit_widgets::*;

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

impl<'a> Widget for MPC2000TUI<'a> {
    impl_render!(self, out, area => self.menu.render(out, area));
}
