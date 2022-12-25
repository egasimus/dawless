use crate::*;

/// Represents the act of drawing something to the output buffer,
/// provided a minimum area is available.
#[derive(Clone)]
pub struct Thunk<'a> {
    /// The minimum size that this thunk needs to render
    pub min_size: Size,
    /// A list of the items contained in this thunk
    pub items: Vec<LayoutItem<'a>>,
    /// A function that takes the list of items in this thunk, a writable output buffer,
    /// and a rectangular area, and draws the items something into that area of the buffer
    /// in a particular layout.
    pub render_fn: fn(&Vec<LayoutItem<'a>>, &mut dyn Write, Area)->Result<()>,
}

impl<'a> Thunk<'a> {
    pub const NIL: Self = Self { min_size: Size::MIN, items: vec![], render_fn: render_nil };
    /// Render this thunk.
    pub fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        (self.render_fn)(&self.items, term, area)
    }
}

impl<'a, T: TUI> From<&'a T> for Thunk<'a> {
    fn from (item: &'a T) -> Self {
        Self {
            min_size: item.min_size(),
            items: vec![item.into()],
            render_fn: render_one,
        }
    }
}

/// A leaf of the layout tree, containing either a widget or a thunk,
/// alongside sizing, padding, and scrolling preferences.
#[derive(Clone, Debug)]
pub struct LayoutItem<'a> {
    pub content:  LayoutContent<'a>,
    pub min_size: Size,
    pub offset:   Point,
    pub padding:  Size,
    pub scrolls:  bool
}

/// This trait is used to bypass the Sized bound that makes `dyn Into<LayoutItem<'a>>` impossible.
trait IntoLayout<'a> {
    fn into_layout (self) -> LayoutItem<'a>;
}

impl<'a, T: TUI> IntoLayout<'a> for &'a T {
    fn into_layout (self) -> LayoutItem<'a> {
        LayoutItem::from(self)
    }
}

impl<'a> IntoLayout<'a> for Thunk<'a> {
    fn into_layout (self) -> LayoutItem<'a> {
        LayoutItem::from(self)
    }
}

/// Add a widget to the layout.
impl<'a, T: TUI> From<&'a T> for LayoutItem<'a> {
    fn from (item: &'a T) -> LayoutItem<'a> {
        let content = LayoutContent::Item(item);
        LayoutItem {
            content,
            min_size: item.min_size(),
            offset:   Point::MIN,
            padding:  Size::MIN,
            scrolls:  false
        }
    }
}

/// Add a thunk to the layout.
impl<'a> From<Thunk<'a>> for LayoutItem<'a> {
    fn from (thunk: Thunk<'a>) -> LayoutItem<'a> {
        let min_size = thunk.min_size;
        let content = LayoutContent::Thunk(thunk);
        LayoutItem {
            content,
            min_size,
            offset:  Point::MIN,
            padding: Size::MIN,
            scrolls: false
        }
    }
}

impl<'a> LayoutItem<'a> {
    pub fn min_size (&self) -> Size {
        self.content.min_size()
    }
    pub fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        match &self.content {
            LayoutContent::Item(item) => item.render(term, area),
            LayoutContent::Thunk(thunk) => thunk.render(term, area)
        }
    }
}


/// The content of a layout item.
#[derive(Clone, Debug)]
pub enum LayoutContent<'a> {
    /// A single widget.
    Item(&'a dyn TUI),
    /// A collection of widgets with attached layout renderer.
    Thunk(Thunk<'a>)
}

impl <'a> LayoutContent<'a> {
    /// Get the minimum size needed to render this layout item.
    #[inline] pub fn min_size (&self) -> Size {
        match self {
            Self::Item(item)   => item.min_size(),
            Self::Thunk(thunk) => thunk.min_size,
        }
    }
}


/// A callable object passed into the layout closure. Calling it collects the layout item.
#[derive(Debug, Default)]
pub struct Define<'a> {
    items: Vec<LayoutItem<'a>>
}

impl<'a> Define<'a> {
    pub fn collect (mut items: impl FnMut(&mut Define<'a>)) -> Vec<LayoutItem<'a>> {
        let mut define = Self::default();
        items(&mut define);
        define.items
    }
    pub fn pad (&mut self, amount: Unit) -> &mut Self {
        let index = self.items.len();
        self.items[index].min_size.0 += amount * 2;
        self.items[index].min_size.1 += amount * 2;
        self
    }
}

impl<'a, T: TUI> FnOnce<(&'a T,)> for Define<'a> {
    type Output = ();
    extern "rust-call" fn call_once (self, _args: (&T,)) -> Self::Output {
        unreachable!()
    }
}

impl<'a, T: TUI> FnMut<(&'a T,)> for Define<'a> {
    extern "rust-call" fn call_mut (&mut self, args: (&'a T,)) -> Self::Output {
        self.items.push(args.0.into());
    }
}

impl<'a> FnOnce<(Thunk<'a>,)> for Define<'a> {
    type Output = ();
    extern "rust-call" fn call_once (self, _args: (Thunk,)) -> Self::Output {
        unreachable!()
    }
}

impl<'a> FnMut<(Thunk<'a>,)> for Define<'a> {
    extern "rust-call" fn call_mut (&mut self, args: (Thunk<'a>,)) -> Self::Output {
        self.items.push(args.0.into());
    }
}


/// Empty render function.
pub fn render_nil <'a> (
    _items: &Vec<LayoutItem<'a>>, _write: &mut dyn Write, _area: Area
) -> Result<()> {
    Ok(())
}

/// Collect widgets in a row thunk.
pub fn row <'a> (items: impl FnMut(&mut Define<'a>)) -> Thunk<'a> {
    let mut min_size = Size::MIN;
    let items = Define::collect(items);
    for item in items.iter() { min_size = min_size.expand_row(item.min_size()) }
    Thunk { items, min_size, render_fn: render_row }
}

/// Render the items from a row thunk.
pub fn render_row <'a> (
    items: &Vec<LayoutItem<'a>>, write: &mut dyn Write, area: Area
) -> Result<()> {
    let mut x = area.0.x();
    for item in items.iter() {
        let size = item.min_size();
        let area = Area(Point(x, area.0.y()), size);
        item.render(write, area)?;
        x = x + size.width();
    }
    Ok(())
}

/// Create a thunk containing one item.
pub fn one <'a, T: TUI> (item: &'a T) -> Thunk<'a> {
    Thunk { items: vec![item.into()], min_size: item.min_size(), render_fn: render_one }
}

pub fn render_one <'a> (
    items: &Vec<LayoutItem<'a>>, write: &mut dyn Write, area: Area
) -> Result<()> {
    items[0].render(write, area)
}

/// Collect widgets in a column thunk.
pub fn col <'a> (items: impl FnMut(&mut Define<'a>)) -> Thunk<'a> {
    let mut min_size = Size::MIN;
    let items = Define::collect(items);
    for item in items.iter() { min_size = min_size.expand_column(item.min_size()) }
    Thunk { items, min_size, render_fn: render_col }
}

/// Render a column thunk.
pub fn render_col <'a> (
    items: &Vec<LayoutItem<'a>>, write: &mut dyn Write, area: Area
) -> Result<()> {
    let mut y = area.0.y();
    for item in items.iter() {
        let size = item.min_size();
        let area = Area(Point(area.0.x(), y), size);
        item.render(write, area)?;
        y = y + size.height();
    }
    Ok(())
}

/// Collect widgets in a stack thunk.
pub fn stack <'a> (items: impl FnMut(&mut Define<'a>)) -> Thunk<'a> {
    let mut min_size = Size::MIN;
    let items = Define::collect(items);
    for item in items.iter() { min_size = min_size.stretch(item.min_size()) }
    Thunk { items, min_size, render_fn: render_stack }
}

/// Render a stack thunk.
pub fn render_stack <'a> (
    items: &Vec<LayoutItem<'a>>, write: &mut dyn Write, area: Area
) -> Result<()> {
    for item in items.iter() { item.render(write, area)?; }
    Ok(())
}

/// Wrap thunk into padding
pub fn grow <'a> (size: Size, thunk: Thunk<'a>) -> Thunk<'a> {
    let min_size = thunk.min_size + size;
    Thunk { items: vec![thunk.into()], min_size, render_fn: render_pad }
}

/// Wrap thunk into padding
pub fn shrink <'a> (size: Size, thunk: Thunk<'a>) -> Thunk<'a> {
    let min_size = thunk.min_size - size;
    Thunk { items: vec![thunk.into()], min_size, render_fn: render_pad }
}

/// Wrap thunk into padding
pub fn pad <'a> (padding: Size, thunk: Thunk<'a>) -> Thunk<'a> {
    let min_size = thunk.min_size + padding + padding;
    Thunk { items: vec![thunk.into()], min_size, render_fn: render_pad }
}

/// Render a stack thunk.
pub fn render_pad <'a> (
    items: &Vec<LayoutItem<'a>>, write: &mut dyn Write, area: Area
) -> Result<()> {
    items[0].render(write, Area(area.0 + Point(1, 1), area.1))?;
    Ok(())
}

#[cfg(test)]
mod test {
    use std::str::from_utf8;
    use crate::{*, layout::*};

    struct One;

    impl<'a> TUI for One {
        fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
            write!(term, "\n{}", Area(area.0, self.min_size()))
        }
        fn min_size (&self) -> Size {
            Size(1, 1)
        }
    }

    #[test]
    fn test_row_col () {

        assert_rendered!(One == "\n1x1+5+5");

        let layout = row(|add| { add(&One); add(&One); });
        assert_eq!(layout.min_size, Size(2, 1));
        assert_rendered!(layout == "\n1x1+5+5\n1x1+6+5");

        let layout = col(|add| { add(&One); add(&One); });
        assert_eq!(layout.min_size, Size(1, 2));
        assert_rendered!(layout == "\n1x1+5+5\n1x1+5+6");

        let layout = col(|add| {
            add(row(|add| { add(&One); add(&One); }));
            add(row(|add| { add(&One); add(row(|add| { add(&One); })); }));
        });
        assert_eq!(layout.min_size, Size(2, 2));
        assert_rendered!(layout == "\n1x1+5+5\n1x1+6+5\n1x1+5+6\n1x1+6+6");

        let layout = stack(|add| {
            add(&One);
            add(row(|add| { add(&One); add(&One); }));
            add(col(|add| { add(&One); add(&One); }));
        });
        assert_eq!(layout.min_size, Size(2, 2));
        assert_rendered!(layout == "\n1x1+5+5\n1x1+5+5\n1x1+6+5\n1x1+5+5\n1x1+5+6");

    }

}
