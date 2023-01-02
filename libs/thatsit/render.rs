use crate::*;

pub struct Collect<'a>(pub Vec<Layout<'a>>);

impl<'a> Collect<'a> {
    pub fn collect (collect: impl Fn(&mut Collect<'a>)) -> Self {
        let mut items = Self(vec![]);
        collect(&mut items);
        items
    }
}

impl<'a, W: Widget + 'a> FnOnce<(W, )> for Collect<'a> {
    type Output = ();
    extern "rust-call" fn call_once (mut self, args: (W,)) -> Self::Output {
        self.call_mut(args)
    }
}

impl<'a, W: Widget + 'a> FnMut<(W, )> for Collect<'a> {
    extern "rust-call" fn call_mut (&mut self, args: (W,)) -> Self::Output {
        args.0.collect(self)
    }
}

impl std::fmt::Debug for dyn Widget {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "dyn[Widget]")
    }
}

impl<'a> Widget for Box<dyn Widget + 'a> {
    impl_render!(self, out, area => (**self).render(out, area));
    fn collect <'b> (self, collect: &mut Collect<'b>) where Self: 'b + Sized {
        collect.0.push(Layout::Box(self));
    }
}

impl<W: Widget> Widget for Option<W> {
    impl_render!(self, out, area => match self {
        Some(item) => item.render(out, area),
        None => Ok((0, 0))
    });
}

impl Widget for () {}

impl Widget for &str {
    impl_render!(self, out, area => {
        let w = self.len() as Unit;
        area.min(w, 1)?;
        out.queue(MoveTo(area.0, area.1))?.queue(Print(&self))?;
        Ok((w, 1))
    });
}

impl Widget for String {
    impl_render!(self, out, area => {
        let w = self.len() as Unit;
        area.min(w, 1)?;
        out.queue(MoveTo(area.0, area.1))?.queue(Print(&self))?;
        Ok((w, 1))
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
