use crate::*;

type RenderFn<'a> = &'a (dyn Fn(&mut dyn Write, Area)->Result<()> + Sync);

#[derive(Clone)]
pub struct Thunk<'a> {
    pub min_size: Size,
    pub render_fn: RenderFn<'a>,
}

impl<'a> TUI for Thunk<'a> {}

impl<'a> std::fmt::Debug for Thunk<'a> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(render: {})", self.min_size)
    }
}
#[derive(Default, Clone, Debug)]
pub struct LayoutItem<'a> {
    pub item:    &'a dyn TUI,
    pub sizing:  Sizing<'a>,
    pub padding: usize,
    pub scrolls: bool
}
impl<'a> LayoutItem<'a> {
    fn collect (mut items: impl FnMut(&mut Define)) -> Vec<&'a dyn TUI> {
        let mut define = Define::default();
        items(&mut define);
        define.items
    }
    pub fn new (item: &'a dyn TUI) -> Self {
        Self { item, ..LayoutItem::default() }
    }
}
#[derive(Default)]
pub struct Define<'a> {
    items: Vec<&'a dyn TUI>
}
impl<'a, T: TUI> FnOnce<(&'a T,)> for Define<'a> {
    type Output = ();
    extern "rust-call" fn call_once (self, _args: (&'a T,)) -> Self::Output {
        unreachable!()
    }
}
impl<'a, T: TUI> FnMut<(&'a T,)> for Define<'a> {
    extern "rust-call" fn call_mut (&mut self, args: (&'a T,)) -> Self::Output {
        self.items.push(args.0.clone());
        ()
    }
}
impl<'a> FnOnce<(Thunk<'a>,)> for Define<'a> {
    type Output = ();
    extern "rust-call" fn call_once (self, _args: (Thunk<'a>,)) -> Self::Output {
        unreachable!()
    }
}
impl<'a> FnMut<(Thunk<'a>,)> for Define<'a> {
    extern "rust-call" fn call_mut (&mut self, args: (Thunk<'a>,)) -> Self::Output {
        self.items.push(args.0.clone());
        ()
    }
}
pub fn row <'a> (items: impl FnMut(&mut Define)) -> Thunk<'a> {
    let mut min_size = Size::MIN;
    let items = LayoutItem::collect(items);
    for item in items.iter() { min_size = min_size.expand_row(item.min_size()) }
    Thunk { min_size, render_fn: &|_write,_area|{Ok(())} }
}
pub fn col <'a> (items: impl FnMut(&mut Define)) -> Thunk<'a> {
    let mut min_size = Size::MIN;
    let items = LayoutItem::collect(items);
    for item in items.iter() { min_size = min_size.expand_column(item.min_size()) }
    Thunk { min_size, render_fn: &|_write,_area|{Ok(())} }
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
            add(&row(|add| {
                add(&One);
                add(&One);
            }));
            add(&row(|add| {
                add(&One);
                add(&One);
            }));
        }).min_size, Size(2, 2));

    }

}
