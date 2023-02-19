#![feature(custom_inner_attributes, proc_macro_hygiene, adt_const_params)]

opt_mod::optional_module_flat!("cli": cli);
opt_mod::optional_module_flat!("tui": tui);

pub mod akai;
pub mod iconnectivity;
pub mod korg;

pub fn main () {
    #[cfg(feature="cli")]
    crate::cli::main()
}
