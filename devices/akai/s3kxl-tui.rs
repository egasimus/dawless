use thatsit::*;
use thatsit_focus::*;

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

impl<'a> Render for S3000XLTUI<'a> {
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        self.menu.render(term, area)
    }
}
