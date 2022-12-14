pub mod widgets;
pub use widgets::*;

pub mod layout;
pub use layout::*;

pub mod theme;
pub use theme::*;

pub use std::io::{Result, Error, ErrorKind, Write};

pub(crate) use crossterm::{
    QueueableCommand, ExecutableCommand,
    style::{
        Print,
        Color, ResetColor, SetForegroundColor, SetBackgroundColor,
        Attribute, SetAttribute
    },
    cursor::{MoveTo, Show, Hide},
    event::{Event, KeyEvent, KeyCode},
    terminal::{
        Clear, ClearType,
        enable_raw_mode, disable_raw_mode,
        EnterAlternateScreen, LeaveAlternateScreen
    }
};

pub fn setup (term: &mut dyn Write) -> Result<()> {
    term.execute(EnterAlternateScreen)?
        .execute(Hide)?;
    enable_raw_mode()
}

pub fn teardown (term: &mut dyn Write) -> Result<()> {
    term.execute(ResetColor)?
        .execute(Show)?
        .execute(LeaveAlternateScreen)?;
    disable_raw_mode()
}

pub fn clear (term: &mut dyn Write) -> Result<()> {
    term.queue(ResetColor)?
        .queue(Clear(ClearType::All))?
        .queue(Hide)?;
    Ok(())
}

pub trait TUI: Sync {
    /** Return minimum and maximum sizes for this component. */
    fn size (&self) -> Size {
        Size::default()
    }
    /** Draw to the terminal. */
    fn render (&self, _term: &mut dyn Write, _space: &Space) -> Result<()> {
        Ok(())
    }
    /** Handle input events. */
    fn handle (&mut self, _event: &Event) -> Result<bool> {
        Ok(false)
    }
    /** Handle focus changes. */
    fn focus (&mut self, _focus: bool) -> bool {
        false
    }
}
