#![feature(unboxed_closures, fn_traits)]

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
