use crate::*;

pub struct Collect<'a>(pub Vec<Layout<'a>>);

impl<'a> Collect<'a> {
    pub fn collect (collect: impl Fn(&mut Collect<'a>)) -> Self {
        let mut items = Self(vec![]);
        collect(&mut items);
        items
    }
}

impl<'a, T: Render + 'a> FnOnce<(T, )> for Collect<'a> {
    type Output = ();
    extern "rust-call" fn call_once (mut self, args: (T,)) -> Self::Output {
        self.call_mut(args)
    }
}

impl<'a, T: Render + 'a> FnMut<(T, )> for Collect<'a> {
    extern "rust-call" fn call_mut (&mut self, args: (T,)) -> Self::Output {
        args.0.collect(self)
    }
}

/// Shorthand for implementing the `render` method of a `Render` trait.
#[macro_export] macro_rules! impl_render {
    ($self:ident, $out:ident, $area:ident => $body:expr) => {
        fn render (&$self, $out: &mut dyn Write, $area: Area) -> Result<()> { $body }
    }
}

pub trait Render {
    impl_render!(self, _out, _area => Ok(()));
    fn collect <'a> (self, collect: &mut Collect<'a>) where Self: 'a + Sized {
        collect.0.push(Layout::Box(Box::new(self)));
    }
}

impl std::fmt::Debug for dyn Render {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "dyn[Render]")
    }
}

impl<T: Render> Render for &T {
    impl_render!(self, out, area => (*self).render(out, area));
    fn collect <'a> (self, collect: &mut Collect<'a>) where Self: 'a + Sized {
        collect.0.push(Layout::Ref(self));
    }
}

impl<'a> Render for Box<dyn Render + 'a> {
    impl_render!(self, out, area => (**self).render(out, area));
    fn collect <'b> (self, collect: &mut Collect<'b>) where Self: 'b + Sized {
        collect.0.push(Layout::Box(self));
    }
}

impl<T: Render> Render for Option<T> {
    impl_render!(self, out, area => match self {
        Some(item) => item.render(out, area),
        None => Ok(())
    });
}

impl Render for () {}

impl Render for &str {
    impl_render!(self, out, area => {
        area.min(self.len() as Unit, 1)?;
        out.queue(MoveTo(area.0, area.1))?.queue(Print(&self))?;
        Ok(())
    });
}

impl Render for String {
    impl_render!(self, out, area => {
        area.min(self.len() as Unit, 1)?;
        out.queue(MoveTo(area.0, area.1))?.queue(Print(&self))?;
        Ok(())
    });
}

/// Compare render output against an expected value.
#[macro_export] macro_rules! assert_rendered {
    ($layout:ident == $expected:expr) => {
        let mut output = Vec::<u8>::new();
        assert_eq!($layout.render(&mut output, Area(Point(5, 5), Size(10, 10))).unwrap(), ());
        assert_eq!(from_utf8(&output).unwrap(), $expected);
    }
}
