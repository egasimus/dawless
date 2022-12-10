pub mod empty;
pub use empty::*;

pub mod space;
pub use space::*;

pub mod theme;
pub use theme::*;

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
    term.execute(EnterAlternateScreen)?;
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
        -> Result<Space> { Ok(Space::new(0, 0, 0, 0)) }
}
