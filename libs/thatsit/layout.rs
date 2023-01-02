use crate::*;

pub type Unit = u16;

#[derive(Copy, Clone, Default, Debug)]
pub struct Area(pub Unit, pub Unit, pub Unit, pub Unit);

impl Area {
    /// Return an error if this area is larger than the minimum needed size
    pub fn min (&self, (min_w, min_h): (Unit, Unit)) -> Result<&Self> {
        if self.w() < min_w || self.h() < min_h {
            Err(Error::new(
                ErrorKind::Other, format!("no space ({:?} < {}x{})", self, min_w, min_h)
            ))
        } else {
            Ok(self)
        }
    }
    /// Move the cursor to the upper left corner of the area
    pub fn home <'a> (&'a self, out: &'a mut dyn Write) -> Result<&'a mut dyn Write> {
        out.queue(MoveTo(self.x(), self.y()))
    }
    #[inline]
    pub fn x (&self) -> Unit {
        self.0
    }
    #[inline]
    pub fn y (&self) -> Unit {
        self.1
    }
    #[inline]
    pub fn w (&self) -> Unit {
        self.2
    }
    #[inline]
    pub fn h (&self) -> Unit {
        self.3
    }
}

/// Render the contained Widget in a sub-Area starting some distance from
/// the upper left corner of the Area that was passed.
#[derive(Copy, Clone, Default)]
pub struct Offset<W: Widget>(pub Unit, pub Unit, pub W);

impl<W: Widget> Widget for Offset<W> {
    impl_render!(self, out, area => {
        let Area(x, y, w, h) = area;
        self.2.render(out, Area(x + self.0, y + self.1, w - self.0, h - self.1))
    });
}

/// Callable struct that collects Layout-wrapped Widgets into itself.
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

/// Wrapper that allows owned and borrowed Widgets to be treated equally.
pub enum Layout<'a> {
    Box(Box<dyn Widget + 'a>),
    Ref(&'a dyn Widget),
    None
}

impl<'a> Debug for Layout<'a> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Layout({})", match self {
            Self::Box(_) => "Box",
            Self::Ref(_) => "Ref",
            Self::None   => ".x.",
        })
    }
}

impl<'a> Widget for Layout<'a> {
    impl_render!(self, out, area => {
        match self {
            Self::Box(item) => (*item).render(out, area),
            Self::Ref(item) => (*item).render(out, area),
            Self::None => Ok((0, 0))
        }
    });
}

#[derive(Debug, Default)]
pub enum Axis { X, #[default] Y, Z }

#[derive(Debug, Default)]
pub struct Stacked<'a>(pub Axis, pub Vec<Layout<'a>>);

impl<'a> Stacked<'a> {
    pub fn x (items: impl Fn(&mut Collect<'a>)) -> Self {
        Self(Axis::X, Collect::collect(items).0)
    }
    pub fn y (items: impl Fn(&mut Collect<'a>)) -> Self {
        Self(Axis::Y, Collect::collect(items).0)
    }
    pub fn z (items: impl Fn(&mut Collect<'a>)) -> Self {
        Self(Axis::Z, Collect::collect(items).0)
    }
}

impl<'a> Widget for Stacked<'a> {
    impl_render!(self, out, area => {
        let mut x = 0;
        let mut y = 0;
        match self.0 {
            Axis::X =>{
                area.min((self.1.len() as Unit, 1))?; // FIXME height
                for item in self.1.iter() {
                    let (w, h) = Offset(x, 0, item).render(out, area)?;
                    x = x + w;
                    y = y.max(h);
                }
            },
            Axis::Y => {
                area.min((1, self.1.len() as Unit))?; // FIXME width
                for item in self.1.iter() {
                    let (w, h) = Offset(0, y, item).render(out, area)?;
                    x = x.max(w);
                    y = y + h;
                }
            },
            Axis::Z => {
                area.min((1, 1 as Unit))?; // FIXME size
                for item in self.1.iter() {
                    let (w, h) = item.render(out, area)?;
                    x = x.max(w);
                    y = y.max(h);
                }
            }
        };
        Ok((x, y))
    });
}

#[derive(Copy, Clone, Default, Debug)]
pub enum Align {
    TopLeft,
    Top,
    TopRight,
    Left,
    #[default] Center,
    Right,
    BottomLeft,
    Bottom,
    BottomRight
}

#[derive(Copy, Clone, Default)]
pub struct Aligned<W: Widget>(pub Align, pub W);

impl<W: Widget> Widget for Aligned<W> {
    impl_render!(self, out, area => {
        self.1.render(out, area)
    });
}
