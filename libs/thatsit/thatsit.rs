#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(type_alias_impl_trait)]

//! `thatsit` is a toy TUI framework based on `crossterm`, containing a basic layout engine.
//! Its main design goal is **brevity**, of both API and implementation.

opt_mod::module_flat!(units);
opt_mod::module_flat!(themes);
opt_mod::module_flat!(layout);
opt_mod::module_flat!(scroll);
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
        size,
        Clear, ClearType,
        enable_raw_mode, disable_raw_mode,
        EnterAlternateScreen, LeaveAlternateScreen
    }
};

use std::{
    fmt::Debug,
    sync::{
        mpsc::{channel, Sender},
        atomic::{AtomicBool, Ordering}
    },
    ops::{Deref, DerefMut},
    cell::RefCell,
    rc::Rc
};

/// Run the main loop. The main thread goes into a render loop. A separate input thread is
/// launched, which sends input to the main thread.
///
/// # Arguments
///
/// * `exited` - Atomic exit flag. Setting this to `true` tells both threads to end.
/// * `term` - A writable output, such as `std::io::stdout()`.
/// * `app` - An instance of the root widget that contains your application.
pub fn run <T: TUI> (
    exited: &'static std::sync::atomic::AtomicBool,
    term:   &mut dyn Write,
    app:    T
) -> Result<()> {
    let app: std::cell::RefCell<T> = RefCell::new(app);
    // Set up event channel and input thread
    let (tx, rx) = channel::<Event>();
    spawn_input_thread(tx, exited);
    // Setup terminal and panic hook
    setup(term, true)?;
    // Render app and listen for updates
    loop {
        // Clear screen
        clear(term).unwrap();
        // Break loop if exited
        if exited.fetch_and(true, Ordering::Relaxed) == true {
            break
        }
        // Render
        let screen_size: Size = size().unwrap().into();
        match app.borrow().layout(screen_size) {
            Ok(layout) => if let Err(error) = layout.render(
                term, Area(Point::MIN, screen_size)
            ) {
                write_error(term, format!("{error}").as_str()).unwrap();
            },
            Err(error) => {
                write_error(term, format!("{error}").as_str()).unwrap();
            }
        }
        // Flush output buffer
        term.flush().unwrap();
        // Wait for input and update
        app.borrow_mut().handle(&rx.recv().unwrap()).unwrap();
    }
    // Clean up
    teardown(term)?;
    Ok(())
}

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
    /// Describe this widget out of renderable elements
    fn layout <'a> (&'a self, max: Size) -> Result<Thunk<'a>> {
        Ok(Thunk { items: vec![], min_size: Size::MIN, render_fn: render_nil })
    }
    /// Render this widget by directly emitting draw commands
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        self.layout(area.1)?.render(term, area)
    }
}

/// Box widgets to transfer ownership where you don't want to specify the type.
impl TUI for Box<dyn TUI> {
    fn focus (&mut self, focus: bool) -> bool {
        (*self).deref_mut().focus(focus)
    }
    fn focused (&self) -> bool {
        (*self).deref().focused()
    }
    fn layout <'a> (&'a self, max: Size) -> Result<Thunk<'a>> {
        (*self).deref().layout(max) 
    }
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        (*self).deref().render(term, area)
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        (*self).deref_mut().handle(event)
    }
}

/// Optional widgets can be hidden by setting them to `None`
/// (losing their state)
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
    fn layout <'a> (&'a self, max: Size) -> Result<Thunk<'a>> {
        match self { Some(x) => x.layout(max), None => Ok(Thunk::NIL) }
    }
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        match self { Some(x) => x.render(term, area), None => Ok(()) }
    }
}

/// `RefCell<T> where T: TUI` transparently wraps widgets
impl<T: TUI> TUI for RefCell<T> {
    fn focus (&mut self, focus: bool) -> bool {
        self.borrow_mut().focus(focus)
    }
    fn focused (&self) -> bool {
        self.borrow().focused()
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        self.borrow_mut().handle(event)
    }
    fn layout <'a> (&'a self, max: Size) -> Result<Thunk<'a>> {
        unsafe { self.try_borrow_unguarded() }.unwrap().layout(max)
    }
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        self.borrow().render(term, area)
    }
}

/// `Rc<RefCell<T>> where T: TUI`s transparently wraps widgets
impl<T: TUI> TUI for Rc<RefCell<T>> {
    fn handle (&mut self, event: &Event) -> Result<bool> {
        self.borrow_mut().handle(event)
    }
    fn focus (&mut self, focus: bool) -> bool {
        self.borrow_mut().focus(focus)
    }
    fn focused (&self) -> bool {
        self.borrow().focused()
    }
    fn layout <'a> (&'a self, max: Size) -> Result<Thunk<'a>> {
        unsafe { self.try_borrow_unguarded() }.unwrap().layout(max)
    }
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        self.borrow().render(term, area)
    }
}

/// Where an unspecified widget is used, the blank one is provided.
impl<'a> Default for &'a dyn TUI { fn default () -> Self { BLANK } }

/// Where an unspecified boxed widget is used, the blank one is provided, in a box.
impl<'a> Default for Box<dyn TUI> { fn default () -> Self { Box::new(BLANK.clone()) } }

impl Debug for &mut dyn TUI {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "&mut dyn[{:?}]", self.layout(Size::MAX))
    }
}

impl Debug for &dyn TUI {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "&dyn[{:?}]", self.layout(Size::MAX))
    }
}

impl Debug for Box<dyn TUI> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Box<dyn[{:?}]>", self.layout(Size::MAX))
    }
}

impl<'a> Debug for Thunk<'a> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Thunk[{}; {}; {:?}]>", self.min_size, self.items.len(), self.items)
    }
}
