#![feature(unboxed_closures, fn_traits)]

pub use std::io::{Result, Error, ErrorKind, Write};
pub use crossterm::{self, event::{KeyEvent, KeyCode, KeyEventState, KeyEventKind, KeyModifiers}};
pub use crossterm::QueueableCommand;
pub(crate) use crossterm::{
    ExecutableCommand,
    style::{Print, Color, ResetColor, SetForegroundColor, /*SetBackgroundColor*/},
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
    sync::{mpsc::{channel, Sender}, atomic::{AtomicBool, Ordering}},
    cell::RefCell,
};

/// Shorthand for implementing the `render` method of a widget.
#[macro_export] macro_rules! impl_render {
    ($self:ident, $out:ident, $area:ident => $body:expr) => {
        fn render (&$self, $out: &mut dyn Write, $area: Area) -> Result<(Unit, Unit)> { $body }
    }
}

/// Shorthand for implementing the `handle` method of a widget.
#[macro_export] macro_rules! impl_handle {
    ($self:ident, $event:ident => $body:expr) => {
        fn handle (&mut $self, $event: &Event) -> Result<bool> {
            $body
        }
    }
}

/// An interface component. Can render itself and handle input.
pub trait Widget {
    impl_render!(self, _out, _area => Ok((0, 0)));
    impl_handle!(self, _event => Ok(false));
    fn collect <'a> (self, collect: &mut Collect<'a>) where Self: 'a + Sized {
        collect.0.push(Layout::Box(Box::new(self)));
    }
}

/// Widgets work the same when referenced.
impl<W: Widget> Widget for &W {
    impl_render!(self, out, area => (*self).render(out, area));
    impl_handle!(self, _event => unreachable!());
    fn collect <'a> (self, collect: &mut Collect<'a>) where Self: 'a + Sized {
        collect.0.push(Layout::Ref(self));
    }
}

opt_mod::module_flat!(render);
opt_mod::module_flat!(layout);
opt_mod::module_flat!(handle);
opt_mod::module_flat!(focus);
opt_mod::module_flat!(utils);
