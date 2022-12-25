use crate::*;

/// A list of sequentially selectable items
#[derive(Debug)]
pub struct Focus<T: TUI> {
    /// The currently focused item
    pub index: usize,
    /// The list of items
    pub items: Vec<T>,
    /// The keys for previous and next items
    pub keys: Option<(KeyCode, KeyCode)>,
}

impl<T: TUI> Default for Focus<T> {
    fn default () -> Self { Self { items: vec![], index: 0, keys: None } }
}

impl<T: TUI> Focus<T> {
    pub fn vertical (items: Vec<T>) -> Focus<T> {
        Self { items, index: 0, keys: Some((KeyCode::Up,   KeyCode::Down)) }
    }
    pub fn horizontal (items: Vec<T>) -> Focus<T> {
        Self { items, index: 0, keys: Some((KeyCode::Left, KeyCode::Right)) }
    }
    pub fn len (&self) -> usize { self.items.len() }
    pub fn get (&self) -> &T { &self.items[self.index] }
    pub fn get_mut (&mut self) -> &mut T { &mut self.items[self.index] }
    pub fn replace (&mut self, items: Vec<T>) -> &mut Self { self.items = items; self }
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
    pub fn next (&mut self) -> bool {
        self.unfocus();
        self.index = if self.index >= self.items.len() - 1 { 0 } else { self.index + 1 };
        self.focus(self.index);
        true
    }
    pub fn prev (&mut self) -> bool {
        self.unfocus();
        self.index = if self.index == 0 { self.items.len() - 1 } else { self.index - 1 };
        self.focus(self.index);
        true
    }
    pub fn handle (&mut self, event: &Event) -> Result<bool> {
        Ok(if let Some((prev, next)) = self.keys {
            match_key!((event) { next => { self.next() }, prev => { self.prev() } })
        } else {
            false
        } || self.get_mut().handle(event)?)
    }
}

/// A vertical list of focusable items
#[derive(Debug)]
pub struct FocusColumn<T: TUI>(pub Focus<T>);

impl<T: TUI> Default for FocusColumn<T> {
    fn default () -> Self { Self(Focus::vertical(vec![])) }
}

impl<T: TUI> FocusColumn<T> {
    pub fn new (items: Vec<T>) -> Self { Self(Focus::vertical(items)) }
    pub fn get (&self) -> &T { &self.0.get() }
    pub fn get_mut (&mut self) -> &mut T { self.0.get_mut() }
    pub fn replace (&mut self, items: Vec<T>) -> &mut Self { self.0.replace(items); self }
    pub fn len (&self) -> usize { self.0.len() }
    pub fn index (&self) -> usize { self.0.index }
}

impl<T: TUI> TUI for FocusColumn<T> {
    fn handle (&mut self, event: &Event) -> Result<bool> { self.0.handle(event) }
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
}

/// A horizontal list of focusable items
#[derive(Debug)]
pub struct FocusRow<T: TUI>(pub Focus<T>);

impl<T: TUI> Default for FocusRow<T> {
    fn default () -> Self { Self(Focus::horizontal(vec![])) }
}

impl<T: TUI> FocusRow<T> {
    pub fn new (items: Vec<T>) -> Self { Self(Focus::horizontal(items)) }
    pub fn get (&self) -> &T { &self.0.get() }
    pub fn get_mut (&mut self) -> &mut T { self.0.get_mut() }
    pub fn replace (&mut self, items: Vec<T>) -> &mut Self { self.0.replace(items); self }
    pub fn len (&self) -> usize { self.0.len() }
    pub fn index (&self) -> usize { self.0.index }
}

impl<T: TUI> TUI for FocusRow<T> {
    fn handle (&mut self, event: &Event) -> Result<bool> { self.0.handle(event) }
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
}

/// A stack of focusable items, rendering one at a time
#[derive(Debug, Default)]
pub struct FocusStack<T: TUI>(pub Focus<T>);

impl<T: TUI> FocusStack<T> {
    pub fn new (items: Vec<T>) -> Self { Self(Focus::horizontal(items)) }
    pub fn get (&self) -> &T { &self.0.get() }
    pub fn get_mut (&mut self) -> &mut T { self.0.get_mut() }
    pub fn replace (&mut self, items: Vec<T>) -> &mut Self { self.0.replace(items); self }
    pub fn len (&self) -> usize { self.0.len() }
}

impl<T: TUI> TUI for FocusStack<T> {
    fn layout <'a> (&'a self) -> Thunk<'a> { self.get().layout() }
    fn min_size (&self) -> Size { self.get().min_size() }
    fn max_size (&self) -> Size { self.get().max_size() }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        Ok(self.0.handle(event)? || self.get_mut().handle(event)? || false)
    }
}

#[derive(Default)]
pub struct TabbedVertical<T: TUI> {
    pub tabs:    FocusColumn<Button>,
    pub pages:   FocusStack<T>,
    pub open:    bool,
    pub entered: bool
}

impl<T: TUI> TabbedVertical<T> {
    /// Create a new selector with vertical tabs from a list of `(Button, TUI)` pairs.
    pub fn new (pairs: Vec<(Button, T)>) -> Self {
        let mut tabs  = vec![];
        let mut pages = vec![];
        for (tab, page) in pairs { tabs.push(tab); pages.push(page); }
        let mut tabs  = FocusColumn::new(tabs);
        let mut pages = FocusStack::new(pages);
        if tabs.len() > 0 {
            tabs.0.items[0].focus(true);
            pages.0.items[0].focus(true);
        }
        Self { tabs, pages, open: false, entered: false }
    }
    /// Show and focus the active page
    pub fn enter (&mut self) -> bool {
        self.open = true;
        self.entered = true;
        self.pages.0.index = self.tabs.0.index;
        true
    }
    /// Move the focus to the tabs
    pub fn exit (&mut self) -> bool { self.entered = false; true }
    /// Show the active page
    pub fn open (&mut self) -> bool { self.open = true; true }
    /// Hide the pages
    pub fn close (&mut self) -> bool { self.open = false; true }
}

impl<T: TUI> TUI for TabbedVertical<T> {
    fn layout <'a> (&'a self) -> Thunk<'a> {
        row(|add|{ add(&self.tabs); if self.open { add(SPACE); add(&self.pages); } })
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        Ok(if self.entered {
            self.pages.get_mut().handle(event)? || if event == &key!(Esc) {
                self.exit()
            } else {
                false
            }
        } else if let Some((prev, next)) = self.tabs.0.keys {
            match_key!((event) {
                next => { self.tabs.0.next() },
                prev => { self.tabs.0.prev() },
                KeyCode::Enter => { self.enter() },
                KeyCode::Esc => { self.close() }
            })
        } else {
            false
        })
    }
}
