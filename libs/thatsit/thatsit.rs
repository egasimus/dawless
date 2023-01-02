#![feature(unboxed_closures, fn_traits, type_alias_impl_trait, unsized_fn_params)]

//! `thatsit` is a toy TUI framework based on `crossterm`, containing a basic layout engine.
//! Its main design goal is **brevity**, of both API and implementation.

opt_mod::module_flat!(units);
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
        let layout = app.borrow().layout(screen_size);
        match layout {
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
pub trait TUI<'a> {
    fn layout <'b: 'a> (&'b self, _max: Size) -> Result<Layout<'b>> { Ok(Layout::None) }
    fn handle (&mut self, _event: &Event) -> Result<bool> { Ok(false) }
}

tui! {
    <'a> Box<dyn TUI<'a>> {
        <'b> layout (self, max) { (**self).layout(max) }
        handle (self, event) { (*self).deref_mut().handle(event) }
    }
}

tui! {
    <'a, T: TUI<'a>> Option<T> {
        <'b> layout (self, max) {
            match self { Some(x) => x.layout(max), None => Ok(Layout::None) }
        }
        handle (self, event) {
            match self { Some(x) => x.handle(event), None => Ok(false) }
        }
    }
}

tui! {
    <'a, T: TUI<'a>> RefCell<T> {
        <'b> layout (self, max) {
            unsafe { self.try_borrow_unguarded() }.unwrap().layout(max)
        }
        handle (self, event) {
            self.borrow_mut().handle(event)
        }
    }
}

/// `Rc<RefCell<T>> where T: TUI`s transparently wraps widgets
tui! {
    <'a, T: TUI<'a>> Rc<RefCell<T>> {
        <'b> layout (self, max) {
            unsafe { self.try_borrow_unguarded() }.unwrap().layout(max)
        }
        handle (self, event) {
            self.borrow_mut().handle(event)
        }
    }
}

/// Where an unspecified widget is used, the blank one is provided.
impl<'a> Default for &'a dyn TUI<'a> {
    fn default () -> Self { BLANK }
}

/// Where an unspecified boxed widget is used, the blank one is provided, in a box.
impl<'a> Default for Box<dyn TUI<'a>> {
    fn default () -> Self { Box::new(BLANK.clone()) }
}

impl<'a> Debug for &mut dyn TUI<'a> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "&mut dyn[{:?}]", "")//self.layout(Size::MAX))
    }
}

impl<'a> Debug for &dyn TUI<'a> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "&dyn[{:?}]", "")//self.layout(Size::MAX))
    }
}

impl<'a> Debug for Box<dyn TUI<'a>> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Box<dyn[{:?}]>", "")//self.layout(Size::MAX))
    }
}

pub type LayoutFn<'l> = dyn Fn(&Vec<Layout<'l>>, &mut dyn Write, Area)->Result<()>;

pub enum Layout<'l> {
    None,
    CustomFn(fn(&mut dyn Write, Area)->Result<()>),
    CustomBox(Box<dyn Fn(&mut dyn Write, Area)->Result<()> + 'l>),
    Many(Size, &'l LayoutFn<'l>, Vec<Layout<'l>>),
    One(&'l dyn TUI<'l>),
}

impl<'l> Layout<'l> {
    pub fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        match self {
            Layout::None                   => Ok(()),
            Layout::CustomFn(render)       => render(term, area),
            Layout::CustomBox(render)      => render(term, area),
            Layout::Many(_, render, items) => render(items, term, area),
            Layout::One(item)              => (*item).layout(area.1)?.render(term, area),
        }
    }
}

impl <'l, T: TUI<'l>> From<&'l T> for Layout<'l> {
    fn from (widget: &'l T) -> Self {
        Layout::One(widget)
    }
}

pub struct Collect<'l>(pub Vec<Layout<'l>>);

impl<'l> FnOnce<(Layout<'l>, )> for Collect<'l> {
    type Output = ();
    extern "rust-call" fn call_once (self, _: (Layout<'l>,)) -> Self::Output { unreachable!() }
}

impl<'l> FnMut<(Layout<'l>, )> for Collect<'l> {
    extern "rust-call" fn call_mut (&mut self, args: (Layout<'l>,)) -> Self::Output {
        self.0.push(args.0)
    }
}

impl<'l, T: TUI<'l>> FnOnce<(&'l T, )> for Collect<'l> {
    type Output = ();
    extern "rust-call" fn call_once (self, _: (&'l T,)) -> Self::Output { unreachable!() }
}

impl<'l, T: TUI<'l>> FnMut<(&'l T, )> for Collect<'l> {
    extern "rust-call" fn call_mut (&mut self, args: (&'l T,)) -> Self::Output {
        self.0.push(Layout::One(args.0))
    }
}

#[cfg(test)]
mod test {
    use std::str::from_utf8;
    use crate::*;

    #[derive(Default)]
    struct One;

    tui! {
        <'a> One {}
    }
    
    #[derive(Default)]
    struct Something {
        a: One,
        b: One,
        c: One,
        d: One,
        e: One,
        f: One,
    }

    tui! {
        <'a> Something {
            <'b> layout (self, max) {
                Ok(Layout::columns(|add|{
                    add(&self.a);
                    add(Layout::rows(|add|{
                        add(&self.b);
                        add(&self.c);
                    }));
                    add(Layout::rows(|add|{
                        add(&self.d);
                        add(&self.e);
                        add(&self.f);
                    }));
                }))
            }
        }
    }

    #[test]
    fn test_row_column () {
        let something = Something::default();
        assert_rendered!(something == "\n1x1+5+5");
    }

}
