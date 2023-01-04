use crate::*;

/// The unit of distance used throughout.
pub type Unit = u16;

/// A rectangle on the screen in (X, Y, W, H) format, from top left.
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
    #[inline]
    pub fn size (&self) -> (Unit, Unit) {
        (self.w(), self.h())
    }
}

/// Render the contained Widget in a sub-Area starting some distance from
/// the upper left corner of the Area that was passed.
#[derive(Copy, Clone, Default)]
pub struct Offset<W: Widget>(pub Unit, pub Unit, pub W);

impl<W: Widget> Widget for Offset<W> {
    impl_render!(self, out, area => {
        let Area(x, y, w, h) = area;
        let area = Area(x + self.0, y + self.1, w.saturating_sub(self.0), h.saturating_sub(self.1));
        self.2.render(out, area)
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

/// Set exact size
#[derive(Debug)]
pub enum Fix<W: Widget> {
    X(Unit, W),
    Y(Unit, W),
    XY((Unit, Unit), W)
}

impl<W: Widget> Fix<W> {
    pub fn get (&self) -> &W {
        match self { Self::X(_, w) => w, Self::Y(_, w) => w, Self::XY(_, w) => w }
    }
    pub fn get_mut (&mut self) -> &mut W {
        match self { Self::X(_, w)  => w, Self::Y(_, w)  => w, Self::XY(_, w) => w }
    }
    pub fn constrain (&self, area: Area) -> Area {
        match self {
            Self::X(width, _)            => Area(area.0, area.1, *width, area.3),
            Self::Y(height, _)           => Area(area.0, area.1, area.2, *height),
            Self::XY((width, height), _) => Area(area.0, area.1, *width, *height)
        }
    }
}

/// Set minimum size
#[derive(Debug)]
pub enum Min<W: Widget> {
    X(Unit, W),
    Y(Unit, W),
    XY((Unit, Unit), W)
}

impl<W: Widget> Widget for Fix<W> {
    impl_render!(self, out, area => {
        let size = self.get().render(out, self.constrain(area))?;
        Ok(match self {
            Self::X(width, _)            => (*width, size.1),
            Self::Y(height, _)           => (size.0, *height),
            Self::XY((width, height), _) => (*width, *height),
        })
    });
    impl_handle!(self, event => self.get_mut().handle(event));
}

impl<W: Widget> Min<W> {
    pub fn get (&self) -> &W {
        match self { Self::X(_, w)  => w, Self::Y(_, w)  => w, Self::XY(_, w) => w }
    }
    pub fn get_mut (&mut self) -> &mut W {
        match self { Self::X(_, w)  => w, Self::Y(_, w)  => w, Self::XY(_, w) => w }
    }
    pub fn constrain (&self, area: Area) -> Area {
        match self {
            Self::X(min_width, _)  => Area(area.0, area.1, area.2.max(*min_width), area.3),
            Self::Y(min_height, _) => Area(area.0, area.1, area.2, area.3.max(*min_height)),
            Self::XY((min_width, min_height), _) => {
                Area(area.0, area.1, area.2.max(*min_width), area.3.max(*min_height))
            }
        }
    }
}

impl<W: Widget> Widget for Min<W> {
    impl_render!(self, out, area => self.get().render(out, self.constrain(area)));
    impl_handle!(self, event => self.get_mut().handle(event));
}

/// Set maximum size
#[derive(Debug)]
pub enum Max<W: Widget> { X(Unit, W), Y(Unit, W), XY((Unit, Unit), W) }

impl<W: Widget> Max<W> {
    pub fn get (&self) -> &W {
        match self { Self::X(_, w)  => w, Self::Y(_, w)  => w, Self::XY(_, w) => w }
    }
    pub fn get_mut (&mut self) -> &mut W {
        match self { Self::X(_, w)  => w, Self::Y(_, w)  => w, Self::XY(_, w) => w }
    }
    pub fn constrain (&self, area: Area) -> Area {
        match self {
            Self::X(max_width, _)  => Area(area.0, area.1, area.2.min(*max_width), area.3),
            Self::Y(max_height, _) => Area(area.0, area.1, area.2, area.3.min(*max_height)),
            Self::XY((max_width, max_height), _) => {
                Area(area.0, area.1, area.2.min(*max_width), area.3.min(*max_height))
            }
        }
    }
}

impl<W: Widget> Widget for Max<W> {
    impl_render!(self, out, area => self.get().render(out, self.constrain(area)));
    impl_handle!(self, event => self.get_mut().handle(event));
}

/// X (left to right), Y (top to bottom), or Z (back to front)
#[derive(Debug, Default)]
pub enum Axis { X, #[default] Y, Z }

/// Order multiple `Widget`s along X (columns), Y (rows), or Z (layers).
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
                for item in self.1.iter().rev() {
                    let (w, h) = item.render(out, area)?;
                    x = x.max(w);
                    y = y.max(h);
                }
            }
        };
        Ok((x, y))
    });
}

/// Direction in which to perform alignment
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

/// Wraps a widget, applying alignment to it.
#[derive(Copy, Clone, Default)]
pub struct Aligned<W: Widget>(pub Align, pub W);

impl<W: Widget> Widget for Aligned<W> {
    impl_render!(self, out, area => {
        self.1.render(out, area)
    });
}
