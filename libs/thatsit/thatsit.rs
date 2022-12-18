//! `thatsit` is a toy TUI framework based on `crossterm`, containing a basic layout engine.
//! Its main design goal is **brevity**, of both API and implementation.

opt_mod::module_flat!(widgets);
opt_mod::module_flat!(themes);
opt_mod::module_flat!(display);
opt_mod::module_flat!(default);
opt_mod::module_flat!(ops);

pub use std::io::{Result, Error, ErrorKind, Write};

pub use crossterm;

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

pub use crossterm::QueueableCommand;

pub fn setup (term: &mut dyn Write) -> Result<()> {
    term.execute(EnterAlternateScreen)?
        .execute(Hide)?;
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
    /// Return the layout of the children of this component.
    fn layout (&self) -> Layout { Layout::default() }
    /// Return the minimum size for this component.
    fn min_size (&self) -> Size { self.layout().min_size() }
    /// Return the minimum size for this component.
    fn max_size (&self) -> Size { self.layout().max_size() }
    /// Draw to the terminal.
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        self.layout().render(term, area)
    }
    /// Handle input events.
    fn handle (&mut self, _event: &Event) -> Result<bool> { Ok(false) }
    /// Handle focus changes.
    fn focus (&mut self, _focus: bool) -> bool { false }
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
#[derive(Copy, Clone, Debug)]
pub enum Sizing<'a> {
    /// Allocate no space for this item
    None,
    /// Always use the item's minimum size
    Min,
    /// Always use the item's maximum size
    Max,
    Grow(Unit),
    Fixed(Size),
    Range(Size, Size),
    Pad(Unit, &'a Sizing<'a>)
}

/// A layout item
#[derive(Clone)]
pub enum Layout<'a> {
    /// Empty layout slot
    None,
    /// A single item
    Item(Sizing<'a>, &'a dyn TUI),
    /// Render items on top of each other
    Layers(Sizing<'a>, Vec<Layout<'a>>),
    /// Render items in a vertical column
    Column(Sizing<'a>, Vec<Layout<'a>>),
    /// Render items in a horizontal row
    Row(Sizing<'a>, Vec<Layout<'a>>),
    /// Render items in a grid
    Grid(Sizing<'a>, Vec<(Layout<'a>, Size)>),
}

impl Point {
    pub const NUL: Self = Self(0, 0);
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
    pub fn stretch (self, other: Self) -> Self {
        Self(self.0.max(other.0), self.1.max(other.1))
    }
    /// Grow width, stretch height
    pub fn expand_row (self, other: Self) -> Self {
        Self(self.0.saturating_add(other.0), self.1.max(other.1))
    }
    /// Stretch width, grow height
    pub fn expand_column (self, other: Self) -> Self {
        Self(self.0.max(other.0), self.1.saturating_add(other.1))
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
    /// Limit the size to the other size
    pub fn crop_to (self, other: Self) -> Self {
        Self(self.0.min(other.0), self.1.min(other.1))
    }
}

impl Area {
    #[inline] pub fn x (self) -> Unit { self.0.x() }
    #[inline] pub fn y (self) -> Unit { self.0.y() }
    #[inline] pub fn width (self) -> Unit { self.1.width() }
    #[inline] pub fn height (self) -> Unit { self.1.height() }
}

impl<'a> Sizing<'a> {
    pub const AUTO: Self = Self::Grow(1);
}

impl<'a> Layout<'a> {
    pub fn sizing (&self) -> Sizing {
        *match self {
            Self::None              => &Sizing::None,
            Self::Item(sizing, _)   => sizing,
            Self::Layers(sizing, _) => sizing,
            Self::Row(sizing, _)    => sizing,
            Self::Column(sizing, _) => sizing,
            Self::Grid(sizing, _)   => sizing
        }
    }
    pub fn size (&self) -> Option<Size> {
        match self.sizing() {
            Sizing::None => Some(Size::MIN),
            Sizing::Min => Some(self.min_size()),
            Sizing::Max => Some(self.max_size()),
            Sizing::Fixed(size) => Some(size),
            _ => None
        }
    }
}

impl<'a> TUI for Layout<'a> {
    fn min_size (&self) -> Size {
        match self {
            Self::None => Size(0, 0),
            Self::Item(sizing, item) => {
                let mut size = item.min_size();
                if let Sizing::Pad(padding, _) = sizing {
                    size.0 += padding * 2;
                    size.1 += padding * 2;
                }
                size
            },
            Self::Layers(_, layers) => {
                let mut size = Size::MIN;
                for layer in layers.iter() { size = size.stretch(layer.min_size()); }
                size
            },
            Self::Row(_, items) => {
                let mut size = Size::MIN;
                for item in items.iter() { size = size.expand_row(item.min_size()); }
                size
            },
            Self::Column(sizing, items) => {
                let mut size = Size::MIN;
                for item in items.iter() { size = size.expand_column(item.min_size()); }
                if let Sizing::Pad(padding, _) = sizing {
                    size.0 += padding * 2;
                    size.1 += padding * 2;
                }
                size
            },
            Self::Grid(_, _) => unimplemented!()
        }
    }
    fn max_size (&self) -> Size {
        match self {
            Self::None => Size(0, 0),
            Self::Item(sizing, item) => {
                let mut size = item.max_size();
                if let Sizing::Pad(padding, _) = sizing {
                    size.0 += padding * 2;
                    size.1 += padding * 2;
                }
                size
            },
            Self::Layers(sizing, layers) => {
                let mut size = Size::MIN;
                for layer in layers.iter() { size = size.stretch(layer.max_size()); }
                if let Sizing::Pad(padding, _) = sizing {
                    size.0 += padding * 2;
                    size.1 += padding * 2;
                }
                size
            },
            Self::Row(sizing, items) => {
                let mut size = Size::MIN;
                for item in items.iter() { size = size.expand_row(item.max_size()); }
                if let Sizing::Pad(padding, _) = sizing {
                    size.0 += padding * 2;
                    size.1 += padding * 2;
                }
                size
            },
            Self::Column(sizing, items) => {
                let mut size = Size::MIN;
                for item in items.iter() { size = size.expand_column(item.max_size()); }
                if let Sizing::Pad(padding, _) = sizing {
                    size.0 += padding * 2;
                    size.1 += padding * 2;
                }
                size
            },
            Self::Grid(_, _) => unimplemented!()
        }
    }
    fn render (&self, term: &mut dyn Write, rect: Area) -> Result<()> {
        Ok(match self {
            Self::None => (),
            Self::Item(sizing, element) => {
                let mut rect = rect;
                if let Sizing::Pad(padding, _) = sizing {
                    rect.0.0 += padding;
                    rect.0.1 += padding;
                    rect.1.0 += padding * 2;
                    rect.1.1 += padding * 2;
                }
                element.render(term, rect)?
            },
            Self::Layers(_, layers) => {
                for layer in layers.iter() {
                    layer.render(term, rect)?;
                }
            },
            Self::Column(sizing, elements) => {
                let mut flex = Flex::new(Size::height, rect.height());
                let sizes = flex.apply(elements)?;
                let width = match sizing {
                    Sizing::Min => self.min_size().width(),
                    Sizing::Max => self.max_size().width(),
                    _ => rect.height()
                };
                let mut rect = rect;
                if let Sizing::Pad(padding, _) = sizing {
                    rect.0.0 += padding;
                    rect.0.1 += padding;
                    rect.1.0 += padding * 2;
                    rect.1.1 += padding * 2;
                }
                let mut y = rect.y();
                for (index, element) in elements.iter().enumerate() {
                    let h = sizes[index];
                    element.render(term, Area(Point(rect.x(), y), Size(width, h)))?;
                    y = y + h;
                }
            },
            Self::Row(sizing, elements) => {
                let mut flex = Flex::new(Size::width, rect.width());
                let sizes = flex.apply(elements)?;
                let height = match sizing {
                    Sizing::Min => self.min_size().height(),
                    Sizing::Max => self.max_size().height(),
                    _ => rect.height()
                };
                let mut rect = rect;
                if let Sizing::Pad(padding, _) = sizing {
                    rect.0.0 += padding;
                    rect.0.1 += padding;
                    rect.1.0 += padding * 2;
                    rect.1.1 += padding * 2;
                }
                let mut x = rect.x();
                for (index, element) in elements.iter().enumerate() {
                    let w = sizes[index];
                    element.render(term, Area(Point(x, rect.y()), Size(w, height)))?;
                    x = x + w;
                }
            },
            Self::Grid(_, _) => {
                unimplemented!()
            },
        })
    }
}

/// Distributes space between widgets
struct Flex<A: Fn(Size) -> Unit> {
    axis:        A,
    remaining:   Unit,
    denominator: Unit
}

impl<A: Fn(Size)->Unit> Flex<A> {
    fn new (axis: A, remaining: Unit) -> Self {
        Self { axis, remaining, denominator: 0 }
    }
    fn prepare (&mut self, layout: &Layout<'_>) -> Result<Unit> {
        let mut taken = 0;
        let mut sizing = layout.sizing();
        if let Sizing::Pad(padding, actual_sizing) = sizing {
            taken  = taken + padding * 2;
            sizing = *actual_sizing;
        }
        match sizing {
            Sizing::None => {},
            Sizing::Min => taken += (self.axis)(layout.min_size()),
            Sizing::Max => taken += (self.axis)(layout.max_size()),
            Sizing::Fixed(size) => taken += (self.axis)(size),
            Sizing::Range(min, _) => taken += (self.axis)(min),
            Sizing::Grow(proportion) => self.denominator += proportion,
            Sizing::Pad(_, _) => return Err(Error::new(ErrorKind::Other, "don't nest padding")),
        };
        Ok(taken)
    }
    fn apply (&mut self, layouts: &Vec<Layout>) -> Result<Vec<Unit>> {
        for layout in layouts.iter() {
            let taken = self.prepare(layout)?;
            if taken > self.remaining {
                return Err(Error::new(ErrorKind::Other, "not enough space"))
            }
            self.remaining = self.remaining - taken;
        }
        let mut sizes = vec![];
        for layout in layouts.iter() {
            let mut sizing = layout.sizing();
            let mut size = 0;
            if let Sizing::Pad(padding, actual_sizing) = sizing {
                size  = size + padding * 2;
                sizing = *actual_sizing;
            }
            size = size + match sizing {
                Sizing::None             => 0,
                Sizing::Min              => (self.axis)(layout.min_size()),
                Sizing::Max              => (self.axis)(layout.max_size()),
                Sizing::Fixed(area)      => (self.axis)(area),
                Sizing::Range(min, _)    => (self.axis)(min),
                Sizing::Grow(proportion) => self.remaining * proportion / self.denominator,
                Sizing::Pad(_, _) => {
                    return Err(Error::new(ErrorKind::Other, "don't nest padding"))
                }
            };
            sizes.push(size);
        }
        Ok(sizes)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const ITEM: &'static Layout = &Layout::Item(Sizing::Grow(1), &Blank {});
    const SCREEN: Point  = Point(100, 100);

    #[test]
    fn test_min_size () {
        let layout = ITEM;
        assert_eq!(layout.min_size(), Size::MIN);
        assert_eq!(layout.max_size(), Size::MAX);

        let layout = Layout::Item(Sizing::Grow(1), layout);
        assert_eq!(layout.min_size(), Size::MIN);
        assert_eq!(layout.max_size(), Size::MAX);

        let layout = Layout::Item(Sizing::Fixed(Size(10, 20)), &layout);
        assert_eq!(layout.min_size(), Size(10, 20));
        assert_eq!(layout.max_size(), Size(10, 20));

        let layout = Layout::Item(Sizing::Grow(1), &layout);
        assert_eq!(layout.min_size(), Size(10, 20));
        assert_eq!(layout.max_size(), Size(10, 20));

        let layout = Layout::Column(Sizing::Grow(1), vec![
            Layout::Item(Sizing::Fixed(Size(10, 20)), ITEM),
            Layout::Item(Sizing::Fixed(Size(20, 10)), ITEM)
        ]);
        assert_eq!(layout.min_size(), Size(20, 30));
        assert_eq!(layout.max_size(), Size(20, 30));

        let layout = Layout::Column(Sizing::Grow(1), vec![
            Layout::Item(Sizing::Fixed(Size(10, 20)), ITEM),
            Layout::Item(Sizing::Fixed(Size(20, 10)), ITEM)
        ]);
        assert_eq!(layout.min_size(), Size(30, 20));
    }

}
