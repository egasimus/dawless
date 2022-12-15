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
    /** Return the layout of the children of this component. */
    fn layout (&self) -> Layout {
        Layout::Solid(Point(0, 0))
    }
    /** Return the minimum/maximum size for this component. */
    fn size (&self) -> Size {
        self.layout().size()
    }
    /** Draw to the terminal. */
    fn render (&self, term: &mut dyn Write, space: &Space) -> Result<()> {
        self.layout().render(term, space)
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
