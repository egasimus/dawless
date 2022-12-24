use thatsit::*;

opt_mod::optional_module_flat!("cli": cli @ "mpc2k-cli.rs");
opt_mod::optional_module_flat!("tui": tui @ "mpc2k-tui.rs");

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
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        self.menu.render(term, area)
    }
}

