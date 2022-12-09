pub mod frame;
pub use frame::*;

pub mod list;
pub use list::*;

pub mod file;
pub use file::*;

pub(crate) use std::io::{Result, Write};
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

pub trait TUI: Sync {
    fn render (&self, term: &mut dyn Write, _col1: u16, _row1: u16, _cols: u16, _rows: u16)
        -> Result<()>;
    fn handle (&mut self, _event: &Event)
        -> Result<bool> { Ok(false) }
    fn focus (&mut self, _focus: bool)
        -> bool { false }
}

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
