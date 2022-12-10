pub mod empty;
pub use empty::*;

pub mod label;
pub use label::*;

pub mod frame;
pub use frame::*;

pub mod list;
pub use list::*;

pub mod file;
pub use file::*;

pub mod scroll;
pub use scroll::*;

pub mod toggle;
pub use toggle::*;

pub use std::io::{Result, Write};

pub(crate) use crossterm::{
    QueueableCommand,
    style::{
        Print,
        Color, ResetColor, SetForegroundColor, SetBackgroundColor,
        Attribute, SetAttribute
    },
    cursor::MoveTo,
    event::{Event, KeyEvent, KeyCode}
};

#[derive(Copy, Clone, Debug)]
pub struct Theme {
    pub bg: Color,
    pub fg: Color,
    pub hi: Color
}

impl Default for Theme {
    fn default () -> Self {
        Theme {
            bg: Color::AnsiValue(232),
            fg: Color::White,
            hi: Color::Yellow
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Space {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

impl Space {
    pub fn new (x: u16, y: u16, w: u16, h: u16) -> Self {
        Self { x, y, w, h }
    }
    /** Return part of the space.
      * Positive x and y coordinates are offsets from top left.
      * Negative x and y coordinates are offsets from bottom right. 
      * Positive w and h are literal width and height.
      * Negative w and h are subtracted from parent width and height. */
    pub fn sub (&self, dx: i16, dy: i16, dw: i16, dh: i16) -> Self {
        let Self { x, y, w, h } = *self;
        Self {
            x: if dx >= 0 { x + dx as u16 } else { x + w - dx as u16 },
            y: if dy >= 0 { y + dy as u16 } else { y + w - dy as u16 },
            w: if dw >= 0 { dw as u16 } else { w - (- dw) as u16 },
            h: if dh >= 0 { dh as u16 } else { h - ( -dh) as u16 },
        }
    }
}

pub trait TUI: Sync {
    /** Draw to the terminal. */
    fn render (&self, term: &mut dyn Write)
        -> Result<()>;
    /** Handle input events. */
    fn handle (&mut self, _event: &Event)
        -> Result<bool> { Ok(false) }
    /** Handle focus changes. */
    fn focus (&mut self, _focus: bool)
        -> bool { false }
    /** Update the layout. */
    fn layout (&mut self, _space: &Space)
        -> Result<Space> { Ok(*_space) }
}

impl FnOnce<(&mut dyn Write,)> for Box<dyn TUI> {
    type Output = std::io::Result<()>;
    extern "rust-call" fn call_once (self, args: (&mut dyn Write,)) -> Self::Output {
        self.render(args.0)
    }
}

impl FnOnce<(&mut dyn Write,)> for &'_ dyn TUI {
    type Output = Result<()>;
    extern "rust-call" fn call_once (self, args: (&mut dyn Write,)) -> Self::Output {
        self.render(args.0)
    }
}
impl FnMut<(&mut dyn Write,)> for &'_ dyn TUI {
    extern "rust-call" fn call_mut (&mut self, args: (&mut dyn Write,)) -> Self::Output {
        self.render(args.0)
    }
}
impl Fn<(&mut dyn Write,)> for &'_ dyn TUI {
    extern "rust-call" fn call (&self, args: (&mut dyn Write,)) -> Self::Output {
        self.render(args.0)
    }
}

//impl FnOnce<(&Event,)> for &mut dyn TUI {
    //type Output = Result<bool>;
    //extern "rust-call" fn call_once (self, event: (&Event,)) -> Result<bool> {
        //self.handle(event.0)
    //}
//}

#[macro_export] macro_rules! tui {
    ($($body:item)*) => {
        #[cfg(feature = "tui")] pub use tui::*;
        #[cfg(feature = "tui")] mod tui {
            use super::*;
            use dawless_common::read;
            $($body)*
        }
    }
}
