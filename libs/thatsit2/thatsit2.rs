#![feature(unboxed_closures, fn_traits)]

pub use std::io::{Result, Error, ErrorKind, Write};
pub use crossterm::{self, event::{KeyEvent, KeyCode, KeyEventState, KeyEventKind, KeyModifiers}};
pub use crossterm::QueueableCommand;
pub(crate) use crossterm::{
    ExecutableCommand,
    style::{Print, Color, ResetColor, SetForegroundColor, SetBackgroundColor},
    cursor::{MoveTo, Show, Hide},
    event::{Event},
    terminal::{
        size,
        Clear, ClearType,
        enable_raw_mode, disable_raw_mode,
        EnterAlternateScreen, LeaveAlternateScreen
    }
};

use std::{
    fmt::Debug,
    sync::{
        mpsc::{channel, Sender},
        atomic::{AtomicBool, Ordering}
    },
    ops::{Deref, DerefMut},
    cell::RefCell,
    rc::Rc,
    borrow::Borrow
};

#[derive(Copy, Clone)]
pub struct Area(u16, u16, u16, u16);

impl Area {
    fn min (&self, w: u16, h: u16) -> Result<()> {
        if self.2 < w || self.3 < h {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, format!("no space")))
        }
    }
}

pub struct Collect<'l>(pub Vec<Layout<'l>>);

impl<'l> Collect<'l> {
    pub fn collect (collect: impl Fn(&mut Collect<'l>)) -> Self {
        let mut items = Self(vec![]);
        collect(&mut items);
        items
    }
}

impl<'l, T: Render + 'l> FnOnce<(T, )> for Collect<'l> {
    type Output = ();
    extern "rust-call" fn call_once (mut self, args: (T,)) -> Self::Output {
        self.call_mut(args)
    }
}

impl<'l, T: Render + 'l> FnMut<(T, )> for Collect<'l> {
    extern "rust-call" fn call_mut (&mut self, args: (T,)) -> Self::Output {
        args.0.collect(self)
    }
}

pub trait Render {
    fn render (&self, _out: &mut dyn Write, _area: Area) -> Result<()> {
        Ok(())
    }
    fn collect <'l> (self, collect: &mut Collect<'l>) where Self: 'l + Sized {
        collect.0.push(Layout::Box(Box::new(self)));
    }
}

impl<T: Render> Render for &T {
    fn render (&self, out: &mut dyn Write, area: Area) -> Result<()> {
        (*self).render(out, area)
    }
    fn collect <'l> (self, collect: &mut Collect<'l>) where Self: 'l + Sized {
        collect.0.push(Layout::Ref(self));
    }
}

impl<'a> Render for Box<dyn Render + 'a> {
    fn render (&self, out: &mut dyn Write, area: Area) -> Result<()> {
        (**self).render(out, area)
    }
    fn collect <'l> (self, collect: &mut Collect<'l>) where Self: 'l + Sized {
        collect.0.push(Layout::Box(self));
    }
}

impl Render for &str {
    fn render (&self, out: &mut dyn Write, area: Area) -> Result<()> {
        area.min(self.len() as u16, 1)?;
        out.queue(MoveTo(area.0, area.1))?.queue(Print(&self))?;
        Ok(())
    }
}

impl Render for String {
    fn render (&self, out: &mut dyn Write, area: Area) -> Result<()> {
        area.min(self.len() as u16, 1)?;
        out.queue(MoveTo(area.0, area.1))?.queue(Print(&self))?;
        Ok(())
    }
}

#[derive(Copy, Clone)]
pub struct Offset<T: Render>(u16, u16, T);

impl<T: Render> Render for Offset<T> {
    fn render (&self, out: &mut dyn Write, area: Area) -> Result<()> {
        out.queue(MoveTo(self.0, self.1))?;
        self.2.render(out, area)
    }
}

pub enum Layout<'l> {
    Box(Box<dyn Render + 'l>),
    Ref(&'l dyn Render)
}

impl<'l> Render for Layout<'l> {
    fn render (&self, out: &mut dyn Write, area: Area) -> Result<()> {
        match self {
            Self::Box(item) => (*item).render(out, area),
            Self::Ref(item) => (*item).render(out, area),
        }
    }
}

pub enum Axis { X, Y, Z }

pub struct Stacked<'l>(Axis, Vec<Layout<'l>>);

impl<'l> Stacked<'l> {
    pub fn x (items: impl Fn(&mut Collect<'l>)) -> Self {
        Self(Axis::X, Collect::collect(items).0)
    }
    pub fn y (items: impl Fn(&mut Collect<'l>)) -> Self {
        Self(Axis::Y, Collect::collect(items).0)
    }
    pub fn z (items: impl Fn(&mut Collect<'l>)) -> Self {
        Self(Axis::Z, Collect::collect(items).0)
    }
}

impl<'l> Render for Stacked<'l> {
    fn render (&self, out: &mut dyn Write, area: Area) -> Result<()> {
        match self.0 {
            Axis::X =>{
                area.min(self.1.len() as u16, 1)?;
                for (index, item) in self.1.iter().enumerate() {
                    Offset(index as u16, 0, item).render(out, area)?;
                }
            },
            Axis::Y => {
                area.min(1, self.1.len() as u16)?;
                for (index, item) in self.1.iter().enumerate() {
                    Offset(0, index as u16, item).render(out, area)?;
                }
            },
            Axis::Z => {
                area.min(1, 1 as u16)?;
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
            layer(Box::new(Stacked::x(|column|{
                column(Box::new(String::from("C1")) as Box<dyn Render>);
                column(Box::new(String::from("C2")) as Box<dyn Render>);
                column(Box::new(String::from("C3")) as Box<dyn Render>);
            })) as Box<dyn Render>);
            layer(Box::new(Stacked::y(|row|{
                row(Box::new(String::from("R1")) as Box<dyn Render>);
                row(Box::new(String::from("R2")) as Box<dyn Render>);
                row(Box::new(String::from("R3")) as Box<dyn Render>);
            })) as Box<dyn Render>);
        });
        layout.render(&mut output, Area(10, 10, 20, 20));
    }
}
