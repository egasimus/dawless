#![feature(unboxed_closures, fn_traits, type_alias_impl_trait, unsized_fn_params)]

//! `thatsit` is a toy TUI framework based on `crossterm`, containing a basic layout engine.
//! Its main design goal is **brevity**, of both API and implementation.

opt_mod::module_flat!(units);
opt_mod::module_flat!(scroll);
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

pub type LayoutFn<'a> = &'a dyn Fn(&mut dyn Write, Area)->Result<()>;

/// A wrapped render function
pub struct Layout<'a>(pub LayoutFn<'a>);

impl<'a> Layout<'a> {
    /// Empty layout, renders nothing
    pub const NIL: Self = Self(&|term, area|Ok(()));
}

impl<'a> Layout<'a> {
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        (self.0)(term, area)
    }
}

/// A terminal UI widget
pub trait TUI {
    /// Handle input events.
    fn handle (&mut self, _event: &Event) -> Result<bool> { Ok(false) }
    /// Describe this widget out of renderable elements
    fn layout <'a> (&'a self, max: Size) -> Result<Layout<'a>> {
        Ok(Layout(&|term, area|{
            self.render(term, Area(Point(0, 0), max))?;
            Ok(())
        }))
    }
    /// Render this widget by directly emitting draw commands
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        Ok(())
    }
}

/// Box widgets to transfer ownership where you don't want to specify the type.
impl TUI for Box<dyn TUI> {
    fn handle (&mut self, event: &Event) -> Result<bool> {
        (*self).deref_mut().handle(event)
    }
    fn layout <'a> (&'a self, max: Size) -> Result<Layout<'a>> {
        (*self).deref().layout(max) 
    }
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        (*self).deref().render(term, area)
    }
}

/// Optional widgets can be hidden by setting them to `None`
/// (losing their state)
impl<T: TUI> TUI for Option<T> {
    fn handle (&mut self, event: &Event) -> Result<bool> {
        match self { Some(x) => x.handle(event), None => Ok(false) }
    }
    fn layout <'a> (&'a self, max: Size) -> Result<Layout<'a>> {
        match self { Some(x) => x.layout(max), None => Ok(Layout::NIL) }
    }
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        match self { Some(x) => x.render(term, area), None => Ok(()) }
    }
}

/// `RefCell<T> where T: TUI` transparently wraps widgets
impl<T: TUI> TUI for RefCell<T> {
    fn handle (&mut self, event: &Event) -> Result<bool> {
        self.borrow_mut().handle(event)
    }
    fn layout <'a> (&'a self, max: Size) -> Result<Layout<'a>> {
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
    fn layout <'a> (&'a self, max: Size) -> Result<Layout<'a>> {
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

impl<'a> Debug for Layout<'a> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Layout")
    }
}

/// A callable struct passed into the layout closure. Calling it collects the layout item.
#[derive(Debug, Default)]
pub struct Collect<'a> { items: Vec<ThunkItem<'a>> }

impl<'a> FnOnce<(&'a dyn TUI,)> for Collect<'a> {
    type Output = ();
    extern "rust-call" fn call_once (self, _args: (&'a dyn TUI,)) -> Self::Output {
        unreachable!()
    }
}

impl<'a> FnMut<(&'a dyn TUI,)> for Collect<'a> {
    /// Add a referenced widget to the layout.
    extern "rust-call" fn call_mut (&mut self, args: (&'a dyn TUI,)) -> Self::Output {
        self.items.push(ThunkItem::Ref(args.0));
    }
}

impl<'a, T: TUI> FnOnce<(T,)> for Collect<'a> {
    type Output = ();
    extern "rust-call" fn call_once (self, _args: (T,)) -> Self::Output {
        unreachable!()
    }
}

impl<'a, T: TUI> FnMut<(T,)> for Collect<'a> {
    /// Add an owned widget to the layout.
    extern "rust-call" fn call_mut (&mut self, args: (T,)) -> Self::Output {
        self.items.push(ThunkItem::Box(Box::new(args.0)));
    }
}

impl<'a> FnOnce<(LayoutFn<'a>,)> for Collect<'a> {
    type Output = ();
    extern "rust-call" fn call_once (self, _args: (LayoutFn<'a>,)) -> Self::Output {
        unreachable!()
    }
}

impl<'a> FnMut<(LayoutFn<'a>,)> for Collect<'a> {
    /// Add an owned widget to the layout.
    extern "rust-call" fn call_mut (&mut self, args: (LayoutFn<'a>,)) -> Self::Output {
        self.items.push(ThunkItem::Fn(Layout(args.0)));
    }
}

impl<'a> Collect<'a> {
    /// Pass an instance into the layout closure to collect the items.
    pub fn collect (mut items: &'a dyn FnMut(Collect<'a>)) -> Vec<ThunkItem<'a>> {
        let mut define = Self::default();
        items(define);
        define.items
    }
}

/// A leaf of the layout tree, containing either a widget or a thunk,
/// alongside sizing, padding, and scrolling preferences.
#[derive(Debug)]
pub enum ThunkItem<'a> {
    /// A reference to a single widget.
    Ref(&'a dyn TUI),
    /// An owned single widget.
    Box(Box<dyn TUI>),
    /// A layout/render function
    Fn(Layout<'a>)
}

impl<'a> ThunkItem<'a> {
    pub fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        match &self {
            Self::Ref(item) => item.render(term, area),
            Self::Box(item) => item.render(term, area),
            //Self::Thunk(thunk) => thunk.render(term, area)
        }
    }
}

struct Columns<'a>(&'a dyn FnMut(Collect<'a>));

impl<'a> TUI for Columns<'a> {
    fn layout <'b> (&'b self, max: Size) -> Result<Layout<'b>> {
        Ok(Layout(&|term, area|{
            let area = Area(Point(0, 0), max);
            let mut y = area.0.y();
            let max_y = area.0.y() + area.1.height();
            for item in Collect::collect(self.0).iter() {
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
        }))
    }
}

struct Rows<'a>(&'a dyn FnMut(Collect<'a>));

impl<'a> TUI for Rows<'a> {
    fn layout <'b> (&'b self, max: Size) -> Result<Layout<'b>> {
        Ok(Layout(&|term, area|{
            let mut x = area.0.x();
            for item in Collect::collect(self.0).iter() {
                let size = Size::MIN;//item.min_size();
                let area = Area(Point(x, area.0.y()), size);
                item.render(term, area)?;
                x = x + size.width();
            }
            Ok(())
        }))
    }
}

struct Layers<'a>(&'a dyn FnMut(Collect<'a>));

impl<'a> TUI for Layers<'a> {
    fn layout <'b> (&'b self, max: Size) -> Result<Layout<'b>> {
        Ok(Layout(&|term, area|{
            let mut x = area.0.x();
            for item in Collect::collect(self.0).iter() {
                item.render(term, area)?;
            }
            Ok(())
        }))
    }
}

#[cfg(test)]
mod test {
    use std::str::from_utf8;
    use crate::*;

    #[derive(Default)]
    struct One;

    impl<'a> TUI for One {
        fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
            write!(term, "\n{}", area)
        }
    }
    
    #[derive(Default)]
    struct Something {
        a: One,
        b: One,
        c: One
    }

    impl TUI for Something {
        fn layout <'a> (&'a self, max: Size) -> Result<Layout<'a>> {
            Columns(&|column|{
                column(self.a);
                column(Rows(&|row|{ row(self.b); row(self.c); }));
            }).layout(max)
        }
    }

    #[test]
    fn test_row_column () {
        let something = Something::default();
        assert_rendered!(something == "\n1x1+5+5");
    }

}
