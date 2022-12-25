#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(type_alias_impl_trait)]

//! `thatsit` is a toy TUI framework based on `crossterm`, containing a basic layout engine.
//! Its main design goal is **brevity**, of both API and implementation.

opt_mod::module_flat!(units);
opt_mod::module_flat!(themes);
opt_mod::module_flat!(layout);
opt_mod::module_flat!(focus);
opt_mod::module_flat!(widgets);
opt_mod::module_flat!(macros);

pub use std::io::{Result, Error, ErrorKind, Write};
pub use crossterm::{self, event::{KeyEvent, KeyCode, KeyEventState, KeyEventKind, KeyModifiers}};
pub use crossterm::QueueableCommand;
pub(crate) use crossterm::{
    ExecutableCommand,
    style::{Print, Color, ResetColor, SetForegroundColor, SetBackgroundColor},
    cursor::{MoveTo, Show, Hide},
    event::{Event},
    terminal::{
        Clear, ClearType,
        enable_raw_mode, disable_raw_mode,
        EnterAlternateScreen, LeaveAlternateScreen
    }
};

use std::{fmt::Debug, sync::{mpsc::Sender, atomic::{AtomicBool, Ordering}}, ops::{Deref, DerefMut}};

pub fn setup (term: &mut dyn Write, better_panic: bool) -> Result<()> {
    if better_panic {
        std::panic::set_hook(Box::new(|panic_info| {
            teardown(&mut std::io::stdout()).unwrap();
            ::better_panic::Settings::auto().create_panic_handler()(panic_info);
        }));
    }
    term.execute(EnterAlternateScreen)?.execute(Hide)?;
    enable_raw_mode()
}

pub fn teardown (term: &mut dyn Write) -> Result<()> {
    term.execute(ResetColor)?.execute(Show)?.execute(LeaveAlternateScreen)?;
    disable_raw_mode()
}

pub fn clear (term: &mut dyn Write) -> Result<()> {
    term.queue(ResetColor)?.queue(Clear(ClearType::All))? .queue(Hide)?;
    Ok(())
}

pub fn spawn_input_thread (tx: Sender<Event>, exited: &'static AtomicBool) {
    std::thread::spawn(move || {
        loop {
            if exited.fetch_and(true, Ordering::Relaxed) { break }
            if crossterm::event::poll(std::time::Duration::from_millis(100)).is_ok() {
                if tx.send(crossterm::event::read().unwrap()).is_err() { break }
            }
        }
    });
}

pub fn write_error (term: &mut dyn Write, msg: &str) -> Result<()> {
    clear(term)?;
    term.queue(SetForegroundColor(Color::Red))?;
    write_text(term, 0, 0, msg)
}

pub fn write_text (term: &mut dyn Write, x: Unit, y: Unit, text: &str) -> Result<()> {
    term.execute(MoveTo(x, y))?.execute(Print(text))?;
    Ok(())
}

/// A terminal UI widget
pub trait TUI {
    /// Handle input events.
    fn handle (&mut self, _event: &Event) -> Result<bool> { Ok(false) }
    /// Handle focus changes.
    fn focus (&mut self, _focus: bool) -> bool { unimplemented!() }
    /// Is this widget focused?
    fn focused (&self) -> bool { unimplemented!() }
    /// Define the layout for this widget
    fn layout <'a> (&'a self) -> Thunk<'a> {
        Thunk { items: vec![], min_size: Size::MIN, render_fn: render_nil }
    }
    /// Draw this widget.
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        self.layout().render(term, area)
    }
}

impl TUI for Box<dyn TUI> {
    fn focus (&mut self, focus: bool) -> bool { (*self).deref_mut().focus(focus) }
    fn focused (&self) -> bool { (*self).deref().focused() }
    fn layout <'a> (&'a self) -> Thunk<'a> { (*self).deref().layout() }
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        (*self).deref().render(term, area)
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        (*self).deref_mut().handle(event)
    }
}

//impl TUI for &mut dyn TUI {
    //fn min_size (&self) -> Size {
        //(*self).min_size()
    //}
    //fn max_size (&self) -> Size {
        //(*self).max_size()
    //}
    //fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        //(*self).render(term, area)
    //}
    //fn handle (&mut self, event: &Event) -> Result<bool> {
        //(*self).handle(event)
    //}
    //fn focus (&mut self, focus: bool) -> bool {
        //(*self).focus(focus)
    //}
    //fn focused (&self) -> bool {
        //(*self).focused()
    //}
    //fn layout <'a> (&'a self) -> Thunk<'a> {
        //(*self).layout()
    //}
//}

impl<T: TUI> TUI for Option<T> {
    fn focus (&mut self, focus: bool) -> bool {
        match self { Some(x) => x.focus(focus), None => false }
    }
    fn focused (&self) -> bool {
        match self { Some(x) => x.focused(), None => false }
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        match self { Some(x) => x.handle(event), None => Ok(false) }
    }
    fn layout <'a> (&'a self) -> Thunk<'a> {
        match self { Some(x) => x.layout(), None => Thunk::NIL }
    }
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        match self { Some(x) => x.render(term, area), None => Ok(()) }
    }
}

impl<T: TUI> TUI for std::cell::RefCell<T> {
    fn focus (&mut self, focus: bool) -> bool { self.borrow_mut().focus(focus) }
    fn focused (&self) -> bool { self.borrow().focused() }
    fn handle (&mut self, event: &Event) -> Result<bool> { self.borrow_mut().handle(event) }
    fn layout <'a> (&'a self) -> Thunk<'a> {
        unsafe { self.try_borrow_unguarded() }.unwrap().layout()
    }
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        self.borrow().render(term, area)
    }
}

impl<T: TUI> TUI for std::rc::Rc<std::cell::RefCell<T>> {
    fn handle (&mut self, event: &Event) -> Result<bool> { self.borrow_mut().handle(event) }
    fn focus (&mut self, focus: bool) -> bool { self.borrow_mut().focus(focus) }
    fn focused (&self) -> bool { self.borrow().focused() }
    fn layout <'a> (&'a self) -> Thunk<'a> {
        unsafe { self.try_borrow_unguarded() }.unwrap().layout()
    }
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        self.borrow().render(term, area)
    }
}

impl<'a> Default for &'a dyn TUI { fn default () -> Self { BLANK } }

impl<'a> Default for Box<dyn TUI> {
    fn default () -> Self { Box::new(BLANK.clone()) }
}

impl Debug for &mut dyn TUI {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}|mut)", self.layout().min_size)
    }
}

impl Debug for &dyn TUI {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({})", self.layout().min_size)
    }
}

impl Debug for Box<dyn TUI> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({})", self.layout().min_size)
    }
}

impl<'a> Debug for Thunk<'a> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(thunk: {} items, min {})", self.items.len(), self.min_size)
    }
}
