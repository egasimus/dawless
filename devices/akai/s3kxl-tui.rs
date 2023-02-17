use thatsit::{*, layouts::*};

pub struct S3000XLUI<'a, T, U> {
    menu: FocusStack<'a, T, U>
}

impl<'a, T, U> S3000XLUI<'a, T, U> {
    pub fn new () -> Self {
        let mut menu = FocusStack::default();
        //menu.add("Edit sample".into(), ());
        Self { menu }
    }
}

impl<'a, T, U> Output<T, U> for S3000XLUI<'a, T, U> {
    fn render (&self, engine: &mut T) -> Result<Option<U>> {
        self.menu.render(engine)
    }
}

