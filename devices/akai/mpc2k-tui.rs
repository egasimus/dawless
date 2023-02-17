use thatsit::{*, layouts::*};

pub struct MPC2000UI<'a, T, U> {
    menu: FocusStack<'a, T, U>
}

impl<'a, T, U> MPC2000UI<'a, T, U> {
    pub fn new () -> Self {
        let mut menu = FocusStack::default();
        //menu.add("Edit sample".into(), ());
        Self { menu }
    }
}

impl<'a, T, U> Output<T, U> for MPC2000UI<'a, T, U> {
    fn render (&self, engine: &mut T) -> Result<Option<U>> {
        self.menu.render(engine)
    }
}
