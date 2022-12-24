use thatsit::*;

#[cfg(feature = "cli")]
#[derive(clap::Subcommand, Clone)]
pub enum Triton {
    PCG {}
}

#[cfg(feature = "cli")]
pub(crate) fn cli (command: &Triton) {
    unimplemented!()
}

#[derive(Default)]
pub struct TritonTUI {
    menu: FocusColumn<Box<dyn TUI>>
}

impl TritonTUI {
    pub fn new () -> Self {
        let mut menu = FocusColumn::default();
        //menu.add("Edit program".into(), ())
             //.add("Edit combi".into(),  ())
             //.add("Edit multi".into(),  ())
             //.add("Edit arp".into(),    ())
             //.add("Edit PRRP".into(),   ());
        Self { menu }
    }
}

impl TUI for TritonTUI {
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        self.menu.render(term, area)
    }
}
