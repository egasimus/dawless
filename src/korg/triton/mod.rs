use crate::*;
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
pub struct TritonUI {
    //menu: FocusStack<'a>
}

impl TritonUI {
    pub fn new () -> Self {
        //let mut menu = FocusStack::default();
        //menu.add("Edit program".into(), ())
             //.add("Edit combi".into(),  ())
             //.add("Edit multi".into(),  ())
             //.add("Edit arp".into(),    ())
             //.add("Edit PRRP".into(),   ());
        Self {
            //menu
        }
    }
}

impl<T, U> Output<T, U> for TritonUI {
    fn render (&self, context: &mut T) -> Result<Option<U>> {
        //impl_render!(self, out, area => self.menu.render(out, area));
        Ok(None)
    }
}
