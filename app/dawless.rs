opt_mod::optional_module_flat!("cli": cli @ "dawless-cli.rs");
opt_mod::optional_module_flat!("tui": tui @ "dawless-tui.rs");

pub fn main () {
    #[cfg(feature="cli")]
    crate::cli::main()
}
