#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(type_alias_impl_trait)]

//! `thatsit` is a toy TUI framework based on `crossterm`, containing a basic layout engine.
//! Its main design goal is **brevity**, of both API and implementation.

opt_mod::module_flat!(widgets);
opt_mod::module_flat!(themes);
opt_mod::module_flat!(display);
opt_mod::module_flat!(default);
opt_mod::module_flat!(ops);
opt_mod::module_flat!(macros);
opt_mod::module_flat!(layout);

pub use std::io::{Result, Error, ErrorKind, Write};
pub use crossterm;
pub use crossterm::QueueableCommand;
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


use std::{sync::{mpsc::Sender, atomic::{AtomicBool, Ordering}}, ops::{Deref, DerefMut}};

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
            if exited.fetch_and(true, Ordering::Relaxed) {
                break
            }
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
    /// Return the minimum size for this component.
    fn min_size (&self) -> Size { Size::MIN }
    /// Return the minimum size for this component.
    fn max_size (&self) -> Size { Size::MAX }
    /// Handle input events.
    fn handle (&mut self, _event: &Event) -> Result<bool> { Ok(false) }
    /// Handle focus changes.
    fn focus (&mut self, _focus: bool) -> bool { false }
    /// Is this widget focused?
    fn focused (&self) -> bool { false }
    /// Define the layout for this widget
    fn layout <'a> (&'a self) -> Thunk<'a> {
        Thunk { items: vec![], min_size: self.min_size(), render_fn: render_nil }
    }
    /// Draw this widget.
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        self.layout().render(term, area)
    }
}

impl TUI for Box<dyn TUI> {
    fn min_size (&self) -> Size {
        (*self).deref().min_size()
    }
    fn max_size (&self) -> Size {
        (*self).deref().max_size()
    }
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        (*self).deref().render(term, area)
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        (*self).deref_mut().handle(event)
    }
    fn focus (&mut self, focus: bool) -> bool {
        (*self).deref_mut().focus(focus)
    }
    fn focused (&self) -> bool {
        (*self).deref().focused()
    }
    fn layout <'a> (&'a self) -> Thunk<'a> {
        (*self).deref().layout()
    }
}

impl<T: TUI> TUI for Option<T> {
    fn min_size (&self) -> Size {
        match self { Some(x) => x.min_size(), None => Size::MIN }
    }
    fn max_size (&self) -> Size {
        match self { Some(x) => x.max_size(), None => Size::MIN }
    }
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        match self { Some(x) => x.render(term, area), None => Ok(()) }
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        match self { Some(x) => x.handle(event), None => Ok(false) }
    }
    fn focus (&mut self, focus: bool) -> bool {
        match self { Some(x) => x.focus(focus), None => false }
    }
    fn focused (&self) -> bool {
        match self { Some(x) => x.focused(), None => false }
    }
    fn layout <'a> (&'a self) -> Thunk<'a> {
        match self { Some(x) => x.layout(), None => Thunk::NIL }
    }
}

impl<T: TUI> TUI for std::cell::RefCell<T> {
    fn min_size (&self) -> Size {
        self.borrow().min_size()
    }
    fn max_size (&self) -> Size {
        self.borrow().max_size()
    }
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        self.borrow().render(term, area)
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        self.borrow_mut().handle(event)
    }
    fn focus (&mut self, focus: bool) -> bool {
        self.borrow_mut().focus(focus)
    }
    fn focused (&self) -> bool {
        self.borrow().focused()
    }
    fn layout <'a> (&'a self) -> Thunk<'a> {
        unsafe { self.try_borrow_unguarded() }.unwrap().layout()
    }
}

impl<T: TUI> TUI for std::rc::Rc<std::cell::RefCell<T>> {
    fn min_size (&self) -> Size {
        self.borrow().min_size()
    }
    fn max_size (&self) -> Size {
        self.borrow().max_size()
    }
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        self.borrow().render(term, area)
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        self.borrow_mut().handle(event)
    }
    fn focus (&mut self, focus: bool) -> bool {
        self.borrow_mut().focus(focus)
    }
    fn focused (&self) -> bool {
        self.borrow().focused()
    }
    fn layout <'a> (&'a self) -> Thunk<'a> {
        unsafe { self.try_borrow_unguarded() }.unwrap().layout()
    }
}

/// The unit of the coordinate system
pub type Unit = u16;

/// A pair of coordinates
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Point (/** Column */ pub Unit, /** Row */ pub Unit);

/// A pair of dimensions
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Size (/** Width */ pub Unit, /** Height */ pub Unit);

/// A rectangle, made of a `Point` and a `Size`
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Area (/** Position */ pub Point, /** Size */ pub Size);

impl Point {
    pub const NIL: Self = Self(0, 0);
    pub const MIN: Self = Self(Unit::MIN, Unit::MIN);
    pub const MAX: Self = Self(Unit::MAX, Unit::MAX);
    #[inline] pub fn x (self) -> Unit { self.0 }
    #[inline] pub fn y (self) -> Unit { self.1 }
    pub fn clip (self, other: Self) -> Self {
        Self(self.0.min(other.0), self.1.min(other.1))
    }
}

impl Size {
    pub const MIN: Self = Self(0, 0);
    pub const MAX: Self = Self(Unit::MAX, Unit::MAX);
    #[inline] pub fn width  (self) -> Unit { self.0 }
    #[inline] pub fn height (self) -> Unit { self.1 }
    /// Increase own size to fit other
    #[inline] pub fn stretch (self, other: Self) -> Self {
        Self(self.0.max(other.0), self.1.max(other.1))
    }
    /// Grow width, stretch height
    #[inline] pub fn expand_row (self, other: Self) -> Self {
        Self(self.0.saturating_add(other.0), self.1.max(other.1))
    }
    /// Stretch width, grow height
    #[inline] pub fn expand_column (self, other: Self) -> Self {
        Self(self.0.max(other.0), self.1.saturating_add(other.1))
    }
    /// Limit the size to the other size
    #[inline] pub fn crop_to (self, other: Self) -> Self {
        Self(self.0.min(other.0), self.1.min(other.1))
    }
    /// Return error if the other area is too small
    pub fn fits_in (self, other: Self) -> Result<()> {
        if self.0 > other.0 {
            let message = format!("need {} columns", self.0);
            return Err(Error::new(ErrorKind::Other, message))
        }
        if self.1 > other.1 {
            let message = format!("need {} rows", self.0);
            return Err(Error::new(ErrorKind::Other, message))
        }
        Ok(())
    }
}

impl Area {
    #[inline] pub fn x (self) -> Unit { self.0.x() }
    #[inline] pub fn y (self) -> Unit { self.0.y() }
    #[inline] pub fn width (self) -> Unit { self.1.width() }
    #[inline] pub fn height (self) -> Unit { self.1.height() }
}
