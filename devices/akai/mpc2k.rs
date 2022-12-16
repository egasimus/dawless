use thatsit::*;

pub struct MPC2000TUI {
    menu: List<()>
}

impl MPC2000TUI {
    pub fn new () -> Self {
        let mut menu = List::default();
        menu.add("Edit sample".into(), ());
        Self { menu }
    }
}

impl TUI for MPC2000TUI {
    fn layout (&self) -> Layout {
        self.menu.layout()
    }
    fn render (&self, term: &mut dyn Write, space: &Space) -> Result<()> {
        Frame { title: "MPC2000".into(), ..Frame::default() }
            .render(term, space)?;
        self.menu.render(term, &space.inset(1))
    }
}

