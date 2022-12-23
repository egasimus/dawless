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
pub trait TUI: Sync {
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
    fn min_size (&self)
        -> Size { (*self).deref().min_size() }
    fn max_size (&self)
        -> Size { (*self).deref().max_size() }
    fn render (&self, term: &mut dyn Write, area: Area)
        -> Result<()> { (*self).deref().render(term, area) }
    fn handle (&mut self, event: &Event)
        -> Result<bool> { (*self).deref_mut().handle(event) }
    fn focus (&mut self, focus: bool)
        -> bool { (*self).deref_mut().focus(focus) }
    fn focused (&self)
        -> bool { (*self).deref().focused() }
    fn layout <'a> (&'a self)
        -> Thunk<'a> { (*self).deref().layout() }
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

/// How flexible is the sizing of a layout item
//#[derive(Copy, Clone, Debug)]
//pub enum Sizing<'a> {
    ///// Allocate no space for this item
    //None,
    ///// Always use the item's minimum size
    //Min,
    ///// Always use the item's maximum size
    //Max,
    //Grow(Unit),
    //Fixed(Size),
    //Range(Size, Size),
    //Pad(Unit, &'a Sizing<'a>),
    //Scroll(&'a Scrollbar, &'a Sizing<'a>)
//}

//impl<'a> Sizing<'a> {
    ///// Default sizing
    //pub const AUTO: Self = Self::Grow(1);
    ///// Get the size along the significant axis
    //pub fn get_size (&self, axis: impl Fn(Size)->Unit, layout: &Layout) -> Option<Unit> {
        //match self {
            //Self::Min  =>
                //Some(axis(layout.min_size())),
            //Self::Max  =>
                //Some(axis(layout.max_size())),
            //Self::Fixed(size) =>
                //Some(axis(*size)),
            //Self::Range(min, _) =>
                //Some(axis(*min)),
            //Self::Pad(padding, sizing) =>
                //Some(padding * 2 + sizing.get_size(axis, layout).unwrap_or(0)),
            //Self::Scroll(_, sizing) =>
                //sizing.get_size(axis, layout),
            //_ => None
        //}
    //}
    ///// Check whether the sizing contains a scroll
    //pub fn can_scroll (&self) -> bool {
        //match self {
            //Self::Scroll(_, _) => true,
            //Self::Pad(_, sizing) => sizing.can_scroll(),
            //_ => false
        //}
    //}
//}

///// A layout item
//#[derive(Clone)]
//pub enum Layout<'a> {
    ///// A single item
    //Item(Sizing<'a>, &'a dyn TUI<'a>),
    ///// Render items on top of each other
    //Layers(Sizing<'a>, Vec<Layout<'a>>),
    ///// Render items in a vertical column
    //Column(Sizing<'a>, Vec<Layout<'a>>),
    ///// Render items in a horizontal row
    //Row(Sizing<'a>, Vec<Layout<'a>>),
    ///// Render items in a grid
    //Grid(Sizing<'a>, Vec<(Layout<'a>, Size)>),
//}

//impl<'a> Layout<'a> {
    ///// Empty layout slot
    //pub const NIL: Self = Layout::Item(Sizing::None, BLANK);
//}

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
    ///// Apply padding
    //pub fn apply_padding (mut self, sizing: &Sizing) -> Self {
        //if let Sizing::Pad(padding, _) = sizing {
            //self.0 = self.0.saturating_add(padding * 2);
            //self.1 = self.1.saturating_add(padding * 2);
        //}
        //self
    //}
}

impl Area {
    #[inline] pub fn x (self) -> Unit { self.0.x() }
    #[inline] pub fn y (self) -> Unit { self.0.y() }
    #[inline] pub fn width (self) -> Unit { self.1.width() }
    #[inline] pub fn height (self) -> Unit { self.1.height() }
    ///// Apply padding
    //pub fn apply_padding (mut self, sizing: &Sizing) -> Self {
        //if let Sizing::Pad(padding, _) = sizing {
            //self.0.0 += padding;
            //self.0.1 += padding;
            //self.1.0 += padding * 2;
            //self.1.1 += padding * 2;
        //}
        //self
    //}
}

//impl<'a> Layout<'a> {
    ///// Get the sizing option of a layout (None if the layout is None)
    //#[inline] pub fn sizing (&self) -> Sizing {
        //*match self {
            //Self::Item(sizing, _)   => sizing,
            //Self::Layers(sizing, _) => sizing,
            //Self::Row(sizing, _)    => sizing,
            //Self::Column(sizing, _) => sizing,
            //Self::Grid(sizing, _)   => sizing
        //}
    //}
    ///// Get the preferred size for a layout
    //#[inline] pub fn size (&self) -> Option<Size> {
        //match self.sizing() {
            //Sizing::None => Some(Size::MIN),
            //Sizing::Min => Some(self.min_size()),
            //Sizing::Max => Some(self.max_size()),
            //Sizing::Fixed(size) => Some(size),
            //_ => None
        //}
    //}
//}

//impl<'a> TUI<'a> for Layout<'a> {
    //fn min_size (&self) -> Size {
        //match self {
            //Self::Item(_, item) => {
                //item.min_size()
            //},
            //Self::Layers(_, items) => {
                //items.iter().fold(Size::MIN, |size, layer|size.stretch(layer.min_size()))
            //},
            //Self::Row(_, items) => {
                //items.iter().fold(Size::MIN, |size, layer|size.expand_row(layer.min_size()))
            //},
            //Self::Column(_, items) => {
                //items.iter().fold(Size::MIN, |size, layer|size.expand_column(layer.min_size()))
            //},
            //Self::Grid(_, _) => unimplemented!()
        //}.apply_padding(&self.sizing())
    //}
    //fn max_size (&self) -> Size {
        //match self {
            //Self::Item(_, item) => {
                //item.max_size()
            //},
            //Self::Layers(_, items) => {
                //items.iter().fold(Size::MIN, |size, layer|size.stretch(layer.max_size()))
            //},
            //Self::Row(_, items) => {
                //items.iter().fold(Size::MIN, |size, layer|size.expand_row(layer.max_size()))
            //},
            //Self::Column(_, items) => {
                //items.iter().fold(Size::MIN, |size, layer|size.expand_column(layer.max_size()))
            //},
            //Self::Grid(_, _) => unimplemented!()
        //}.apply_padding(&self.sizing())
    //}
    //fn render (&self, term: &mut dyn Write, rect: Area) -> Result<()> {
        //let rect = rect.apply_padding(&self.sizing());
        //Ok(match self {
            //Self::Item(_, item) => {
                //item.render(term, rect)?
            //},
            //Self::Layers(_, items) => {
                //for item in items.iter() { item.render(term, rect)?; }
            //},
            //Self::Column(sizing, elements) => {
                //let mut flex = Flex::new(Size::height, rect.height());
                //let (sizes, scroll) = flex.apply(elements, sizing.can_scroll())?;
                //let width = match sizing {
                    //Sizing::Min => self.min_size(), Sizing::Max => self.max_size(), _ => rect.1
                //}.width();
                //let mut y = rect.y();
                //for (index, element) in elements.iter().enumerate() {
                    //let h = sizes[index];
                    //element.render(term, Area(Point(rect.x(), y), Size(match element.sizing() {
                        //Sizing::Min => element.min_size().width(),
                        //_ => width
                    //}, h)))?;
                    //y = y + h;
                //}
            //},
            //Self::Row(sizing, elements) => {
                //let mut flex = Flex::new(Size::width, rect.width());
                //let (sizes, scroll) = flex.apply(elements, sizing.can_scroll())?;
                //let height = match sizing {
                    //Sizing::Min => self.min_size(), Sizing::Max => self.max_size(), _ => rect.1
                //}.height();
                //let mut x = rect.x();
                //for (index, element) in elements.iter().enumerate() {
                    //let w = sizes[index];
                    //element.render(term, Area(Point(x, rect.y()), Size(w, match element.sizing() {
                     //Sizing::Min => element.min_size().height(),
                        //_ => height
                    //})))?;
                    //x = x + w;
                //}
            //},
            //Self::Grid(_, _) => {
                //unimplemented!()
            //},
        //})
    //}
//}

//// Distributes space between widgets
//struct Flex<A: Fn(Size) -> Unit> {
    //axis:        A,
    //remaining:   Unit,
    //denominator: Unit
//}

//impl<A: Fn(Size)->Unit> Flex<A> {
    //fn new (axis: A, remaining: Unit) -> Self {
        //Self { axis, remaining, denominator: 0 }
    //}
    //fn prepare (&mut self, layout: &Layout<'_>) -> Result<Unit> {
        //let sizing = layout.sizing();
        //Ok(match sizing.get_size(&self.axis, layout) {
            //Some(size) => size,
            //None => {
                //if let Sizing::Grow(proportion) = sizing {
                    //self.denominator += proportion;
                //}
                //0
            //}
        //})
    //}
    //fn apply (&mut self, layouts: &Vec<Layout>, can_scroll: bool) -> Result<(Vec<Unit>, bool)> {
        //let mut scroll = false;
        //for layout in layouts.iter() {
            //let taken = self.prepare(layout)?;
            //if taken > self.remaining {
                //if can_scroll {
                    //scroll = true;
                    //break
                //} else {
                    //panic!("{:?} {}", layouts, can_scroll);
                    //return Err(Error::new(ErrorKind::Other, "not enough space"))
                //}
            //}
            //self.remaining = self.remaining - taken;
        //}
        //let mut sizes = vec![];
        //for layout in layouts.iter() {
            //let sizing = layout.sizing();
            //sizes.push(match sizing.get_size(&self.axis, layout) {
                //Some(size) => size,
                //None => {
                    //if let Sizing::Grow(proportion) = sizing {
                        //self.remaining * proportion / self.denominator
                    //} else {
                        //0
                    //}
                //}
            //});
        //}
        //Ok((sizes, scroll))
    //}
//}
