pub mod empty;
pub use empty::*;

pub mod frame;
pub use frame::*;

pub mod list;
pub use list::*;

pub mod file;
pub use file::*;

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

#[derive(Copy, Clone)]
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

pub type Rect = (u16, u16, u16, u16);

pub trait TUI: Sync {
    fn render (&self, term: &mut dyn Write)
        -> Result<()>;
    fn handle (&mut self, _event: &Event)
        -> Result<bool> { Ok(false) }
    fn focus (&mut self, _focus: bool)
        -> bool { false }
    fn layout (&mut self, col1: u16, row1: u16, cols: u16, rows: u16)
        -> Result<()> { Ok(()) }
}

//impl FnOnce<(&mut dyn Write,)> for dyn TUI {
    //type Output = Result<()>;
    //extern "rust-call" fn call_once (self, term: (&mut dyn Write,)) -> Self::Output {
        //self.render(term.0)
    //}
//}

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
