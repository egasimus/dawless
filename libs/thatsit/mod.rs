#![feature(unboxed_closures, fn_traits)]

/// `thatsit` is a minimal GUI framework geared towards helping you build your custom task-specific
/// widgets out of a set of composable UI behaviors. It currently targets terminal user interfaces
/// (TUIs), building upon `crossterm`, which provides the basics: displaying styled and positioned
/// text, as well as receiving user input. On top of this, it adds a simple layout system for
/// positioning widgets relative to each other, taking into account the screen size.

pub use std::io::{Result, Error, ErrorKind, Write};
pub use crossterm::{
    self,
    QueueableCommand,
    style::Stylize,
    event::{Event, KeyEvent, KeyCode, KeyEventState, KeyEventKind, KeyModifiers}
};

pub(crate) use crossterm::{
    ExecutableCommand,
    style::{
        Print, Color, ResetColor, SetForegroundColor, /*SetBackgroundColor,*/
        StyledContent
    },
    cursor::{MoveTo, Show, Hide},
    terminal::{
        size,
        Clear, ClearType,
        enable_raw_mode, disable_raw_mode,
        EnterAlternateScreen, LeaveAlternateScreen
    }
};

pub(crate) use std::{
    fmt::{Debug, Display},
    sync::{mpsc::{channel, Sender}, atomic::{AtomicBool, Ordering}},
    cell::RefCell,
};

opt_mod::module_flat!(widget);
opt_mod::module_flat!(layout);
opt_mod::module_flat!(focus);
opt_mod::module_flat!(scroll);
opt_mod::module_flat!(utils);

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
