use crate::*;

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
pub struct TritonUI<'a> {
    menu: FocusStack<'a>
}

impl<'a> TritonUI<'a> {
    pub fn new () -> Self {
        let mut menu = FocusStack::default();
        //menu.add("Edit program".into(), ())
             //.add("Edit combi".into(),  ())
             //.add("Edit multi".into(),  ())
             //.add("Edit arp".into(),    ())
             //.add("Edit PRRP".into(),   ());
        Self { menu }
    }
}

impl<'a> Widget for TritonUI<'a> {
    impl_render!(self, out, area => self.menu.render(out, area));
}