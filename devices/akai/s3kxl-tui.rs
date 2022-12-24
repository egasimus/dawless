use thatsit::*;

pub struct S3000XLTUI {
    menu: FocusColumn<Box<dyn TUI>>
}

impl S3000XLTUI {
    pub fn new () -> Self {
        let mut menu = FocusColumn::default();
        //menu.add("Edit sample".into(), ());
        Self { menu }
    }
}

impl TUI for S3000XLTUI {
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        self.menu.render(term, area)
    }
}
