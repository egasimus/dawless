use crate::*;

/// A list of sequentially selectable items
#[derive(Debug)]
pub struct FocusList<T: TUI> {
    /// The currently focused item
    pub index: usize,
    /// The list of items
    pub items: Vec<T>,
}

impl<T: TUI> Default for FocusList<T> {
    fn default () -> Self { Self::new(vec![]) }
}

impl<T: TUI> FocusList<T> {
    pub fn new (items: Vec<T>) -> Self { Self { items, index: 0 } }
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
        self.get_mut().handle(event)
    }
}

/// A vertical list of focusable items
#[derive(Debug, Default)]
pub struct FocusColumn<T: TUI> {
    /// A focus list of the contained widgets
    pub items:  FocusList<T>,
    /// A scroll offset
    pub offset: usize
}

impl<T: TUI> FocusColumn<T> {
    pub fn new (items: Vec<T>) -> Self {
        Self { items: FocusList::new(items), offset: 0 }
    }
    pub fn get (&self) -> &T { &self.items.get() }
    pub fn get_mut (&mut self) -> &mut T { self.items.get_mut() }
    pub fn replace (&mut self, items: Vec<T>) -> &mut Self { self.items.replace(items); self }
    pub fn len (&self) -> usize { self.items.len() }
    pub fn next (&mut self) -> bool { self.items.next() }
    pub fn prev (&mut self) -> bool { self.items.prev() }
    pub fn index (&self) -> usize { self.items.index }
}

impl<T: TUI> TUI for FocusColumn<T> {
    fn layout <'a> (&'a self, _: Size) -> Result<Thunk<'a>> {
        Ok(col_stretch(|add|{ for item in self.items.items.iter() { add(&*item); } }))
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        Ok(self.items.handle(event)? || match_key!((event) {
            KeyCode::Up   => { self.prev() },
            KeyCode::Down => { self.next() }
        }))
    }
}

/// A horizontal list of focusable items
#[derive(Debug, Default)]
pub struct FocusRow<T: TUI> {
    /// A focus list of the contained widgets
    pub items: FocusList<T>,
    /// A scroll offset
    pub offset: usize
}

impl<T: TUI> FocusRow<T> {
    pub fn new (items: Vec<T>) -> Self { Self { items: FocusList::new(items), offset: 0 } }
    pub fn get (&self) -> &T { &self.items.get() }
    pub fn get_mut (&mut self) -> &mut T { self.items.get_mut() }
    pub fn replace (&mut self, items: Vec<T>) -> &mut Self { self.items.replace(items); self }
    pub fn len (&self) -> usize { self.items.len() }
    pub fn index (&self) -> usize { self.items.index }
}

impl<T: TUI> TUI for FocusRow<T> {
    fn layout <'a> (&'a self, _: Size) -> Result<Thunk<'a>> {
        Ok(row(|add|{ for item in self.items.items.iter() { add(&*item); } }))
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        Ok(self.items.handle(event)? || match_key!((event) {
            KeyCode::Left  => { self.items.prev() },
            KeyCode::Right => { self.items.next() }
        }))
    }
}

/// A stack of focusable items, rendering one at a time
#[derive(Debug, Default)]
pub struct FocusStack<T: TUI>(pub FocusList<T>);

impl<T: TUI> FocusStack<T> {
    pub fn new (items: Vec<T>) -> Self { Self(FocusList::new(items)) }
    pub fn get (&self) -> &T { &self.0.get() }
    pub fn get_mut (&mut self) -> &mut T { self.0.get_mut() }
    pub fn replace (&mut self, items: Vec<T>) -> &mut Self { self.0.replace(items); self }
    pub fn len (&self) -> usize { self.0.len() }
}

impl<T: TUI> TUI for FocusStack<T> {
    fn layout <'a> (&'a self, max: Size) -> Result<Thunk<'a>> {
        self.get().layout(max)
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        Ok(self.0.handle(event)? || self.get_mut().handle(event)? || false)
    }
}

#[derive(Default, Debug)]
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
        if tabs.len() > 0 { tabs.items.items[0].focus(true); pages.0.items[0].focus(true); }
        Self { tabs, pages, open: false, entered: false }
    }
    /// Add a tab/page pair.
    pub fn add (&mut self, label: &str, page: T) {
        self.tabs.items.items.push(Button::new(String::from(label), None));
        self.pages.0.items.push(page);
    }
    /// Show and focus the active page
    pub fn enter (&mut self) -> bool {
        self.open = true;
        self.entered = true;
        self.tabs.get_mut().pressed = true;
        self.pages.0.index = self.tabs.items.index;
        true
    }
    /// Move the focus to the tabs
    pub fn exit (&mut self) -> bool {
        self.entered = false;
        self.tabs.get_mut().pressed = false;
        true
    }
    /// Show the active page
    pub fn open (&mut self) -> bool { self.open = true; true }
    /// Hide the pages
    pub fn close (&mut self) -> bool { self.open = false; true }
    /// Currently selected tab
    pub fn index (&self) -> usize { self.tabs.index() }
    /// Number of tabs
    pub fn len (&self) -> usize {
        let len = self.tabs.len();
        if len != self.pages.len() { panic!("tabs and pages went out of sync") }
        len
    }
}

impl<T: TUI> TUI for TabbedVertical<T> {
    fn layout <'a> (&'a self, _: Size) -> Result<Thunk<'a>> {
        Ok(row(|add|{ add(&self.tabs); if self.open { add(&self.pages); } }))
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        Ok(if self.entered {
            self.pages.get_mut().handle(event)? || if event == &key!(Esc) {
                self.exit()
            } else {
                false
            }
        } else {
            self.tabs.handle(event)? || match_key!((event) {
                KeyCode::Enter => { self.enter() },
                KeyCode::Esc   => { self.close() }
            })
        })
    }
}
