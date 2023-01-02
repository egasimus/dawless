use crate::*;

pub type Unit = u16;

#[derive(Copy, Clone)]
pub struct Area(pub Unit, pub Unit, pub Unit, pub Unit);

impl Area {
    pub fn min (&self, w: Unit, h: Unit) -> Result<()> {
        if self.2 < w || self.3 < h {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, format!("no space")))
        }
    }
    pub fn x (&self) -> Unit {
        self.0
    }
    pub fn y (&self) -> Unit {
        self.1
    }
    pub fn w (&self) -> Unit {
        self.2
    }
    pub fn h (&self) -> Unit {
        self.3
    }
}

#[derive(Copy, Clone, Default)]
pub struct Offset<T: Render>(Unit, Unit, T);

impl<T: Render> Render for Offset<T> {
    fn render (&self, out: &mut dyn Write, area: Area) -> Result<()> {
        out.queue(MoveTo(self.0, self.1))?;
        self.2.render(out, area)
    }
}

pub enum Layout<'a> {
    Box(Box<dyn Render + 'a>),
    Ref(&'a dyn Render),
    None
}

impl<'a> std::fmt::Debug for Layout<'a> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Layout({})", match self {
            Self::Box(_) => "Box",
            Self::Ref(_) => "Ref",
            Self::None   => ".x.",
        })
    }
}

impl<'a> Render for Layout<'a> {
    fn render (&self, out: &mut dyn Write, area: Area) -> Result<()> {
        match self {
            Self::Box(item) => (*item).render(out, area),
            Self::Ref(item) => (*item).render(out, area),
            Self::None => Ok(())
        }
    }
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

impl<'a> Render for Stacked<'a> {
    fn render (&self, out: &mut dyn Write, area: Area) -> Result<()> {
        match self.0 {
            Axis::X =>{
                area.min(self.1.len() as Unit, 1)?;
                for (index, item) in self.1.iter().enumerate() {
                    Offset(index as Unit, 0, item).render(out, area)?;
                }
            },
            Axis::Y => {
                area.min(1, self.1.len() as Unit)?;
                for (index, item) in self.1.iter().enumerate() {
                    Offset(0, index as Unit, item).render(out, area)?;
                }
            },
            Axis::Z => {
                area.min(1, 1 as Unit)?;
                for item in self.1.iter() {
                    item.render(out, area)?;
                }
            }
        };
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_row_column () {
        let mut output = Vec::<u8>::new();
        let layout = Stacked::z(|layer|{
            layer(Stacked::x(|row|{
                row(String::from("R1"));
                row(String::from("R2"));
                row(String::from("R3"));
            }));
            layer(Stacked::y(|column|{
                column(String::from("C1"));
                column(String::from("C2"));
                column(String::from("C3"));
            }));
        });
        layout.render(&mut output, Area(10, 10, 20, 20));
    }
}
