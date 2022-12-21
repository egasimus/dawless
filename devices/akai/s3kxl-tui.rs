use thatsit::*;

pub struct S3000XLTUI {
    menu: List<()>
}

impl S3000XLTUI {
    pub fn new () -> Self {
        let mut menu = List::default();
        menu.add("Edit sample".into(), ());
        Self { menu }
    }
}

impl<'a> TUI<'a> for S3000XLTUI {
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        self.menu.render(term, area)
    }
}
