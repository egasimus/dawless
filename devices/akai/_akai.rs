#![allow(incomplete_features)]
#![feature(adt_const_params)]
opt_mod::optional_module_flat!("cli": cli @ "_akai-cli.rs");
opt_mod::optional_module_flat!("tui": tui @ "_akai-tui.rs");
opt_mod::module_flat!(s3kxl);
opt_mod::module_flat!(mpc2k);
