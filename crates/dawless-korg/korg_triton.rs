use dawless_common::*;

#[cfg(feature = "cli")]
#[derive(clap::Subcommand, Clone)]
pub enum Triton {
    PCG {}
}

#[cfg(feature = "cli")]
pub(crate) fn cli (command: &Triton) {
    unimplemented!()
}

pub struct TritonTUI {
    menu: List<()>
}
impl TritonTUI {
    pub fn new () -> Self {
        let mut menu = List::default();
        menu.add("Edit program".into(), ())
             .add("Edit combi".into(),  ())
             .add("Edit multi".into(),  ())
             .add("Edit arp".into(),    ())
             .add("Edit PRRP".into(),   ());
        Self { menu }
    }
}
impl TUI for TritonTUI {
    fn layout (&self) -> Layout {
        self.menu.layout()
    }
    fn render (&self, term: &mut dyn Write, space: &Space) -> Result<()> {
        Frame { title: "Triton".into(), ..Frame::default() }
            .render(term, space)?;
        self.menu.render(term, &space.inset(1))
    }
}
