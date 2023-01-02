use thatsit::*;
use thatsit_widgets::*;

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

impl<'a> Widget for MioXLUI<'a> {
    impl_render!(self, term, area => self.menu.render(term, area));
}
