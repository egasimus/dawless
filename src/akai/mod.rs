#![allow(incomplete_features)]
#![feature(adt_const_params)]

opt_mod::optional_module_flat!("cli": cli);
opt_mod::optional_module_flat!("tui": tui);

pub mod s3kxl;
pub mod mpc2k;
