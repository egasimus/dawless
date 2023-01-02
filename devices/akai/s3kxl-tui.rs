use thatsit::*;
use thatsit_widgets::*;

pub struct S3000XLTUI<'a> {
    menu: FocusStack<'a>
}

impl<'a> S3000XLTUI<'a> {
    pub fn new () -> Self {
        let mut menu = FocusStack::default();
        //menu.add("Edit sample".into(), ());
        Self { menu }
    }
}

impl<'a> Widget for S3000XLTUI<'a> {
    impl_render!(self, out, area => self.menu.render(out, area));
}
