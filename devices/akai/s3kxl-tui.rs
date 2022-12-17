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

impl TUI for S3000XLTUI {
    fn layout (&self) -> Layout {
        self.menu.layout()
    }
}
