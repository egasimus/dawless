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
    pub fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        (self.render_fn)(&self.items, term, area)
    }
}

/// A leaf of the layout tree, containing either a widget or a thunk,
/// alongside sizing, padding, and scrolling preferences.
#[derive(Clone, Debug)]
pub struct LayoutItem<'a> {
    pub content: LayoutContent<'a>,
    //pub sizing:  Sizing<'a>,
    pub padding: usize,
    pub scrolls: bool
}

impl<'a> LayoutItem<'a> {
    pub fn item (item: &'a dyn TUI) -> Self {
        Self {
            content: LayoutContent::Item(item),
            //sizing:  Sizing::Min,
            padding: 0,
            scrolls: false
        }
    }
    pub fn thunk (thunk: Thunk<'a>) -> Self {
        Self {
            content: LayoutContent::Thunk(thunk),
            //sizing:  Sizing::Min,
            padding: 0,
            scrolls: false
        }
    }
    pub fn collect (mut items: impl FnMut(&mut Define<'a>)) -> Vec<Self> {
        let mut define = Define::default();
        items(&mut define);
        define.items
    }
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


#[derive(Clone, Debug)]
pub struct LayoutItemModifier {
    index: usize
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
    pub fn min_size (&self) -> Size {
        match self {
            Self::Item(item)   => item.min_size(),
            Self::Thunk(thunk) => thunk.min_size,
        }
    }
}


/// A callable object passed into the layout closure. Calling it collects the layout item.
#[derive(Default)]
pub struct Define<'a> {
    items: Vec<LayoutItem<'a>>
}

impl<'a, T: TUI> FnOnce<(&'a T,)> for Define<'a> {
    type Output = ();
    extern "rust-call" fn call_once (self, _args: (&'a T,)) -> Self::Output {
        unreachable!()
    }
}

impl<'a, T: TUI> FnMut<(&'a T,)> for Define<'a> {
    extern "rust-call" fn call_mut (&mut self, args: (&'a T,)) -> Self::Output {
        self.items.push(LayoutItem::item(args.0));
        ()
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
        self.items.push(LayoutItem::thunk(args.0));
        ()
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
    let items = LayoutItem::collect(items);
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

/// Collect widgets in a column thunk.
pub fn col <'a> (items: impl FnMut(&mut Define<'a>)) -> Thunk<'a> {
    let mut min_size = Size::MIN;
    let items = LayoutItem::collect(items);
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
    let items = LayoutItem::collect(items);
    for item in items.iter() { min_size = min_size.stretch(item.min_size()) }
    Thunk { items, min_size, render_fn: render_stack }
}

/// Render a stack thunk.
pub fn render_stack <'a> (
    items: &Vec<LayoutItem<'a>>, write: &mut dyn Write, area: Area
) -> Result<()> {
    for item in items.iter() {
        item.render(write, area)?;
    }
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

