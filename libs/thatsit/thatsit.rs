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
    tui!(handle (self, _event) { Ok(false) });
    tui!(layout (self, _max) { Ok(Layout::empty()) });
    tui!(render (self, _term, _area) { Ok(()) });
}

impl<T: TUI + ?Sized> TUI for &T {
    tui!(handle (self, _event) { unreachable!() });
    tui!(layout (self, max) { (*self).layout(max) });
    tui!(render (self, term, area) { (*self).render(term, area) });
}

/// Box widgets to transfer ownership where you don't want to specify the type.
impl TUI for Box<dyn TUI> {
    tui!(handle (self, event) { (*self).deref_mut().handle(event) });
    tui!(layout (self, max) { (**self).layout(max) });
    tui!(render (self, term, area) { (*self).render(term, area) });
}

/// Optional widgets can be hidden by setting them to `None`
/// (losing their state)
impl<T: TUI> TUI for Option<T> {
    tui!(handle (self, event) { match self { Some(x) => x.handle(event), None => Ok(false) } });
    tui!(layout (self, max) { match self { Some(x) => x.layout(max), None => Ok(Layout::empty()) } });
    tui!(render (self, term, area) { match self { Some(x) => x.render(term, area), None => Ok(()) } });
}

/// `RefCell<T> where T: TUI` transparently wraps widgets
impl<T: TUI> TUI for RefCell<T> {
    tui!(handle (self, event) { self.borrow_mut().handle(event) });
    tui!(layout (self, max) { unsafe { self.try_borrow_unguarded() }.unwrap().layout(max) });
    tui!(render (self, term, area) { unsafe { self.try_borrow_unguarded() }.unwrap().render(term, area) });
}

/// `Rc<RefCell<T>> where T: TUI`s transparently wraps widgets
impl<T: TUI> TUI for Rc<RefCell<T>> {
    tui!(handle (self, event) { self.borrow_mut().handle(event) });
    tui!(layout (self, max) { unsafe { self.try_borrow_unguarded() }.unwrap().layout(max) });
    tui!(render (self, term, area) { unsafe { self.try_borrow_unguarded() }.unwrap().render(term, area) });
}

/// Where an unspecified widget is used, the blank one is provided.
impl<'a> Default for &'a dyn TUI { fn default () -> Self { BLANK } }

/// Where an unspecified boxed widget is used, the blank one is provided, in a box.
impl<'a> Default for Box<dyn TUI> { fn default () -> Self { Box::new(BLANK.clone()) } }

impl Debug for &mut dyn TUI {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "&mut dyn[{:?}]", "")//self.layout(Size::MAX))
    }
}

impl Debug for &dyn TUI {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "&dyn[{:?}]", "")//self.layout(Size::MAX))
    }
}

impl Debug for Box<dyn TUI> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Box<dyn[{:?}]>", "")//self.layout(Size::MAX))
    }
}

pub type LayoutFn<'l> = &'l dyn Fn(&'l [Layout<'l>], &mut dyn Write, Area)->Result<()>;

pub enum Layout<'l> {
    Many(Size, LayoutFn<'l>, Vec<Layout<'l>>),
    Ref(&'l dyn TUI),
    None
}

impl<'l> Layout<'l> {
    pub fn empty () -> Self {
        Self::None
    }
    pub fn one (item: &'l dyn TUI) -> Self {
        Self::Ref(item)
    }
    pub fn columns (define: impl Fn(&mut Collect<'l>)) -> Self {
        let mut items = Collect(vec![]);
        define(&mut items);
        let items = items.0;
        Self::Many(Size(1, items.len() as u16), &|items, term, area|{
            let mut y = area.0.y();
            let max_y = area.0.y() + area.1.height();
            for item in items.iter() {
                let size = Size::MIN;//item.min_size();
                let next_y = y + size.height();
                if next_y > max_y {
                    let msg = format!("need {} more rows", next_y - max_y);
                    return Err(Error::new(ErrorKind::Other, msg))
                }
                item.render(term, Area(Point(area.0.x(), y), size))?;
                y = y + size.height();
            }
            Ok(())
        }, items)
    }
    pub fn rows (define: impl Fn(&mut Collect<'l>)) -> Self {
        let mut items = Collect(vec![]);
        define(&mut items);
        let items = items.0;
        Self::Many(Size(1, items.len() as u16), &|items, term, area|{
            let mut x = area.0.x();
            for item in items.iter() {
                let size = Size::MIN;//item.min_size();
                let area = Area(Point(x, area.0.y()), size);
                item.render(term, area)?;
                x = x + size.width();
            }
            Ok(())
        }, items)
    }
    pub fn layers (define: impl Fn(&mut Collect<'l>)) -> Self {
        let mut items = Collect(vec![]);
        define(&mut items);
        let items = items.0;
        Self::Many(Size(1, items.len() as u16), &|items, term, area|{
            for item in items.iter() { item.render(term, area)?; }
            Ok(())
        }, items)
    }
    pub fn render (&'l self, term: &mut dyn Write, area: Area) -> Result<()> {
        match self {
            Self::Many(_, render, items) => render(items.as_slice(), term, area),
            Self::Ref(item) => item.render(term, area),
            Self::None => Ok(())
        }
    }
}

impl <'l, T: TUI> From<&'l T> for Layout<'l> {
    fn from (widget: &'l T) -> Self {
        Layout::one(widget)
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

impl<'l, T: TUI> FnOnce<(&'l T, )> for Collect<'l> {
    type Output = ();
    extern "rust-call" fn call_once (self, _: (&'l T,)) -> Self::Output { unreachable!() }
}

impl<'l, T: TUI> FnMut<(&'l T, )> for Collect<'l> {
    extern "rust-call" fn call_mut (&mut self, args: (&'l T,)) -> Self::Output {
        self.0.push(Layout::one(args.0))
    }
}

#[cfg(test)]
mod test {
    use std::str::from_utf8;
    use crate::*;

    #[derive(Default)]
    struct One;

    impl<'a> TUI for One {
        tui!(render (self, term, area) {
            write!(term, "\n{}", area)
        });
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

    impl TUI for Something {
        tui!(layout (self, max) {
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
        });
    }

    #[test]
    fn test_row_column () {
        let something = Something::default();
        assert_rendered!(something == "\n1x1+5+5");
    }

}
