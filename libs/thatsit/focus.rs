use crate::*;

/// A list of sequentially selectable items
#[derive(Debug)]
pub struct Focus<T: TUI> {
    /// The currently focused item
    pub index: usize,
    /// The key to switch to the next item
    next: KeyCode,
    /// The key to switch to the previous item
    prev: KeyCode,
    /// The list of items
    pub items: Vec<T>,
}

impl<T: TUI> Default for Focus<T> {
    fn default () -> Self {
        Self { items: vec![], index: 0, next: KeyCode::Down, prev: KeyCode::Up }
    }
}

impl<T: TUI> Focus<T> {
    pub fn vertical (items: Vec<T>) -> Focus<T> {
        Self { items, index: 0, next: KeyCode::Down, prev: KeyCode::Up }
    }
    pub fn horizontal (items: Vec<T>) -> Focus<T> {
        Self { items, index: 0, next: KeyCode::Right, prev: KeyCode::Left }
    }
    pub fn get (&self) -> &T {
        &self.items[self.index]
    }
    pub fn get_mut (&mut self) -> &mut T {
        &mut self.items[self.index]
    }
    pub fn replace (&mut self, items: Vec<T>) -> &mut Self {
        self.items = items;
        self
    }
    pub fn unfocus (&mut self) {
        if let Some(item) = self.items.get_mut(self.index) { item.focus(false); }
    }
    pub fn focus (&mut self, index: usize) -> bool {
        if self.items.get(self.index).is_some() {
            self.index = index;
            self.items[self.index].focus(true);
            true
        } else {
            false
        }
    }
    pub fn next (&mut self) {
        self.unfocus();
        self.index = if self.index >= self.items.len() - 1 { 0 } else { self.index + 1 };
        self.focus(self.index);
    }
    pub fn prev (&mut self) {
        self.unfocus();
        self.index = if self.index == 0 { self.items.len() - 1 } else { self.index - 1 };
        self.focus(self.index);
    }
    pub fn handle (&mut self, event: &Event) -> Result<bool> {
        Ok(match_key!((event) {
            self.next => { self.next(); true },
            self.prev => { self.prev(); true }
        }))
    }
}

#[derive(Debug, Default)]
pub struct FocusColumn<T: TUI>(pub Focus<T>);

impl<T: TUI> FocusColumn<T> {
    pub fn new (items: Vec<T>) -> Self { Self(Focus::vertical(items)) }
    pub fn get (&self) -> &T { &self.0.get() }
    pub fn get_mut (&mut self) -> &mut T { self.0.get_mut() }
    pub fn replace (&mut self, items: Vec<T>) -> &mut Self { self.0.replace(items); self }
}

impl<T: TUI> TUI for FocusColumn<T> {
    fn layout <'a> (&'a self) -> Thunk<'a> {
        col(|add|{ for item in self.0.items.iter() { add(&*item); } })
    }
    fn min_size (&self) -> Size {
        let mut size = Size::MIN;
        for item in self.0.items.iter() { size = size.expand_column(item.min_size()) }
        size
    }
    fn max_size (&self) -> Size {
        let mut size = Size::MIN;
        for item in self.0.items.iter() { size = size.expand_column(item.max_size()) }
        size
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        Ok(self.0.handle(event)? || self.get_mut().handle(event)? || false)
    }
}

#[derive(Debug, Default)]
pub struct FocusRow<T: TUI>(pub Focus<T>);

impl<T: TUI> FocusRow<T> {
    pub fn new (items: Vec<T>) -> Self { Self(Focus::horizontal(items)) }
    pub fn get (&self) -> &T { &self.0.get() }
    pub fn get_mut (&mut self) -> &mut T { self.0.get_mut() }
    pub fn replace (&mut self, items: Vec<T>) -> &mut Self { self.0.replace(items); self }
}

impl<T: TUI> TUI for FocusRow<T> {
    fn layout <'b> (&'b self) -> Thunk<'b> {
        row(|add|{ for item in self.0.items.iter() { add(&*item); } })
    }
    fn min_size (&self) -> Size {
        let mut size = Size::MIN;
        for item in self.0.items.iter() { size = size.expand_row(item.min_size()) }
        size
    }
    fn max_size (&self) -> Size {
        let mut size = Size::MIN;
        for item in self.0.items.iter() { size = size.expand_row(item.max_size()) }
        size
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        Ok(self.0.handle(event)? || self.get_mut().handle(event)? || false)
    }
}

