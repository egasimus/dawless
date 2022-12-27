use std::io::Result;
use thatsit::{*, crossterm::{self, *, event::Event}};
use thatsit_focus::*;

#[derive(Default, Debug)]
pub struct TabbedVertical<T: TUI> {
    pub pages:   FocusState<(String, T)>,
    pub open:    Option<usize>,
    pub entered: bool
}

impl<T: TUI> TabbedVertical<T> {
    /// Create a new selector with vertical tabs from a list of `(Button, TUI)` pairs.
    pub fn new (pages: Vec<(String, T)>) -> Self {
        let pages = FocusState::new(pages);
        pages.select_next();
        Self { pages, open: None, entered: false }
    }
    /// Add a tab/page pair.
    pub fn add (&mut self, label: String, page: T) {
        self.pages.items_mut().push((label, page));
    }
    /// Show and focus the active page
    pub fn enter (&mut self) -> bool {
        self.open = true;
        self.entered = true;
        self.tabs.get_mut().map(|button|button.pressed = true);
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
            match self.pages.get_mut() {
                Some((_, page)) => page.handle(event),
                None => Ok(false)
            }? || if event == &key!(Esc) {
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
