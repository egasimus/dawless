use std::{io::Result, fmt::Debug, cell::Cell};
use thatsit::{*, crossterm::{self, event::Event, style::Color}};

pub struct DefaultTabsTheme;

impl TabsTheme for DefaultTabsTheme {}

pub trait TabsTheme {
    fn foreground (&self, focused: bool, selected: bool) -> Option<Color> {
        Some(match (focused, selected) {
            (true,  true)  => Color::White,
            (true,  false) => Color::White,
            (false, true)  => Color::White,
            (false, false) => Color::White,
        })
    }
    fn background (&self, focused: bool, selected: bool) -> Option<Color> {
        Some(match (focused, selected) {
            (true,  true)  => Color::Black,
            (true,  false) => Color::Black,
            (false, true)  => Color::Black,
            (false, false) => Color::Black,
        })
    }
}

impl Debug for dyn TabsTheme {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "dyn[TabsTheme]")
    }
}

#[derive(Debug)]
pub enum TabSide { None, Top, Right, Bottom, Left }

#[derive(Debug)]
pub struct Tabs<T: Widget> {
    pub side:    TabSide,
    pub pages:   FocusList<(String, T)>,
    pub open:    bool,
    pub entered: bool,
    pub theme:   &'static dyn TabsTheme,
    pub scroll:  ScrollState
}

impl<T: Widget> Default for Tabs<T> {
    fn default () -> Self {
        Self {
            side:    TabSide::Left,
            pages:   FocusList::new(vec![]),
            open:    false,
            entered: false,
            theme:   &DefaultTabsTheme,
            scroll:  ScrollState::default()
        }
    }
}

impl<T: Widget> Tabs<T> {
    /// Create a new selector with vertical tabs from a list of `(Button, Widget)` pairs.
    pub fn new (side: TabSide, pages: Vec<(String, T)>) -> Self {
        let mut tabs = Self { side, pages: FocusList::new(pages), ..Self::default() };
        tabs.select_next();
        tabs
    }
    /// Add a tab/page pair.
    pub fn add (&mut self, label: String, page: T) {
        self.pages.items_mut().push((label, page));
    }
    /// Show and focus the active page
    pub fn enter (&mut self) -> bool {
        self.open();
        self.entered = true;
        true
    }
    /// Move the focus to the tabs
    pub fn exit (&mut self) -> bool {
        self.entered = false;
        true
    }
    /// Show the active page
    pub fn open (&mut self) -> bool {
        self.open = true;
        true
    }
    /// Hide the pages
    pub fn close (&mut self) -> bool {
        self.open = false;
        true
    }
    /// Number of tabs
    pub fn len (&self) -> usize {
        self.pages.len()
    }
    /// The index of the selected tab
    pub fn selected (&self) -> Option<usize> {
        self.pages.selected()
    }

    pub fn select_prev (&mut self) -> bool {
        if self.pages.select_prev() {
            self.scroll.to(self.pages.selected().unwrap());
            true
        } else {
            false
        }
    }

    pub fn select_next (&mut self) -> bool {
        if self.pages.select_next() {
            self.scroll.to(self.pages.selected().unwrap());
            true
        } else {
            false
        }
    }

    pub fn layout_tabs (&self) -> Option<Stacked> {
        let selected = self.pages.selected();
        let tabs = |add: &mut Collect|{
            for (index, (label, _)) in self.pages.iter().enumerate().skip(self.scroll.offset) {
                let label = label.clone();
                if let Some(selected) = selected && selected == index {
                    add(Styled(&|s: String|s.with(Color::Yellow).bold(), label));
                } else {
                    add(Styled(&|s: String|s.with(Color::White), label));
                }
                if index as Unit >= self.scroll.size.get() as u16 {
                    break
                }
            }
        };
        match self.side {
            TabSide::None   => None,
            TabSide::Left   => Some(Stacked::y(tabs)),
            TabSide::Right  => Some(Stacked::y(tabs)),
            TabSide::Top    => Some(Stacked::x(tabs)),
            TabSide::Bottom => Some(Stacked::x(tabs)),
        }
    }

    pub fn layout_page (&self) -> Option<&T> {
        match self.pages.get() {
            Some((_, page)) => if self.open { Some(page) } else { None },
            None => None
        }
    }
}

impl<T: Widget> Widget for Tabs<T> {

    impl_render!(self, out, area => {
        self.scroll.size.set(area.h() as usize); // Record the height for scrolling
        match self.side {
            TabSide::None => self.layout_page().render(out, area),
            TabSide::Left => Some(Stacked::x(|add|{
                add(self.layout_tabs());
                if let Some(page) = self.layout_page() { add(1); add(page); }
            })).render(out, area),
            TabSide::Right => Stacked::x(|add|{
                if let Some(page) = self.layout_page() { add(page); add(1); }
                add(self.layout_tabs());
            }).render(out, area),
            TabSide::Top => Stacked::y(|add|{
                add(self.layout_tabs());
                if let Some(page) = self.layout_page() { add(1); add(page); }
            }).render(out, area),
            TabSide::Bottom => Stacked::x(|add|{
                if let Some(page) = self.layout_page() { add(page); add(1); }
                add(self.layout_tabs());
            }).render(out, area)
        }

    });

    impl_handle!(self, event => {
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
            match_key!((event) {
                KeyCode::Up    => { self.select_prev() },
                KeyCode::Down  => { self.select_next() },
                KeyCode::Enter => { self.enter() },
                KeyCode::Esc   => { self.close() }
            })
        })
    });

}
