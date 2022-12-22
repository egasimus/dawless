use crate::*;

/// A function that takes a writable output buffer and a rectangular area,
/// and draws something into that area of the buffer.
type RenderFn<'a> = &'a (dyn Fn(&mut dyn Write, Area)->Result<()> + Sync);


/// Represents the act of drawing something to the output buffer,
/// provided a minimum area is available.
#[derive(Clone)]
pub struct Thunk<'a> {
    pub min_size:  Size,
    pub items:     Vec<LayoutItem<'a>>,
    pub render_fn: fn(&'a Vec<LayoutItem<'a>>, &mut dyn Write, Area)->Result<()>,
}

impl<'a> TUI<'a> for Thunk<'a> {}

impl<'a> std::fmt::Debug for Thunk<'a> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(render: {})", self.min_size)
    }
}


/// A leaf of the layout tree, containing either a widget or a thunk,
/// alongside sizing, padding, and scrolling preferences.
#[derive(Clone, Debug)]
pub struct LayoutItem<'a> {
    pub content: LayoutContent<'a>,
    pub sizing:  Sizing<'a>,
    pub padding: usize,
    pub scrolls: bool
}

impl<'a> LayoutItem<'a> {
    pub fn item (item: &'a dyn TUI<'a>) -> Self {
        Self {
            content: LayoutContent::Item(item),
            sizing:  Sizing::Min,
            padding: 0,
            scrolls: false
        }
    }
    pub fn thunk (thunk: Thunk<'a>) -> Self {
        Self {
            content: LayoutContent::Thunk(thunk),
            sizing:  Sizing::Min,
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
    pub fn render (&'a self, term: &mut dyn Write, area: Area) -> Result<()> {
        match &self.content {
            LayoutContent::Item(item) => item.render(term, area),
            LayoutContent::Thunk(thunk) => (thunk.render_fn)(&thunk.items, term, area)
        }
    }
}


/// The content of a layout item.
#[derive(Clone, Debug)]
pub enum LayoutContent<'a> {
    Item(&'a dyn TUI<'a>),
    Thunk(Thunk<'a>)
}

impl <'a> LayoutContent<'a> {
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

impl<'a, T: TUI<'a>> FnOnce<(&'a T,)> for Define<'a> {
    type Output = ();
    extern "rust-call" fn call_once (self, _args: (&'a T,)) -> Self::Output {
        unreachable!()
    }
}

impl<'a, T: TUI<'a>> FnMut<(&'a T,)> for Define<'a> {
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
    _items: &'a Vec<LayoutItem<'a>>, _write: &mut dyn Write, _area: Area
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
    items: &'a Vec<LayoutItem<'a>>, write: &mut dyn Write, area: Area
) -> Result<()> {
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
    items: &'a Vec<LayoutItem<'a>>, write: &mut dyn Write, area: Area
) -> Result<()> {
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
    items: &'a Vec<LayoutItem<'a>>, write: &mut dyn Write, area: Area
) -> Result<()> {
    Ok(())
}


#[cfg(test)]
mod test {
    use crate::{*, layout::*};

    struct One;

    impl TUI for One {
        fn min_size (&self) -> Size {
            Size(1, 1)
        }
    }

    #[test]
    fn test_row_col () {

        assert_eq!(row(|add| {
            add(&One);
            add(&One);
        }).min_size, Size(2, 1));

        assert_eq!(col(|add| {
            add(&One);
            add(&One);
        }).min_size, Size(1, 2));

        assert_eq!(col(|add| {
            add(row(|add| { add(&One); add(&One); }));
            add(row(|add| { add(row(|add| { add(&One); })); add(&One); }));
        }).min_size, Size(2, 2));

        assert_eq!(stack(|add| {
            add(&One);
            add(row(|add| { add(&One); add(&One); }));
            add(col(|add| { add(&One); add(&One); }));
        }).min_size, Size(2, 2));

    }

}

