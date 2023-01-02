use thatsit::*;
use thatsit_focus::*;

pub struct S3000XLTUI {
    menu: FocusColumn<Box<dyn Render>>
}

impl S3000XLTUI {
    pub fn new () -> Self {
        let mut menu = FocusColumn::default();
        //menu.add("Edit sample".into(), ());
        Self { menu }
    }
}

impl Render for S3000XLTUI {
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        self.menu.render(term, area)
    }
}
