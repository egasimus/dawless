opt_mod::module_flat!(widgets);
opt_mod::module_flat!(layout);
opt_mod::module_flat!(themes);

pub use std::io::{Result, Error, ErrorKind, Write};

pub use crossterm;

pub(crate) use crossterm::{
    ExecutableCommand,
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

pub use crossterm::QueueableCommand;

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

pub fn write_error (term: &mut dyn Write, msg: &str) -> Result<()> {
    clear(term)?;
    term.queue(SetForegroundColor(Color::Red))?
        .queue(MoveTo(0, 0))?
        .queue(Print(msg))?;
    Ok(())
}

pub trait TUI: Sync {
    /** Return the layout of the children of this component. */
    fn layout (&self) -> Layout {
        Layout::None
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

impl std::fmt::Debug for dyn TUI {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TUI {}", self.size())
    }
}
