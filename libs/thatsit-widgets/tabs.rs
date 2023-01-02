use std::io::Result;
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

impl std::fmt::Debug for dyn TabsTheme {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "dyn[TabsTheme]")
    }
}

#[derive(Debug)]
pub struct TabsLeft<T: Widget> {
    pub pages:   FocusList<(String, T)>,
    pub open:    bool,
    pub entered: bool,
    pub theme:   &'static dyn TabsTheme
}

impl<T: Widget> Default for TabsLeft<T> {
    fn default () -> Self {
        Self {
            pages:   FocusList::new(vec![]),
            open:    false,
            entered: false,
            theme:   &DefaultTabsTheme
        }
    }
}

impl<T: Widget> TabsLeft<T> {
    /// Create a new selector with vertical tabs from a list of `(Button, Widget)` pairs.
    pub fn new (pages: Vec<(String, T)>) -> Self {
        let mut tabs = Self::default();
        tabs.pages.replace(pages);
        tabs.pages.select_next();
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
        //self.tabs.get_mut().map(|button|button.pressed = true);
        //self.pages.0.index = self.tabs.items.index;
        true
    }
    /// Move the focus to the tabs
    pub fn exit (&mut self) -> bool {
        self.entered = false;
        //self.tabs.get_mut().pressed = false;
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
}

impl<T: Widget> Widget for TabsLeft<T> {

    impl_render!(self, out, area => {
        let selected = self.pages.selected();
        Stacked::x(|column|{
            column(Stacked::y(|row|{
                for (index, (label, _)) in self.pages.iter().enumerate() {
                    let label = label.clone();
                    if let Some(selected) = selected && selected == index {
                        row(Styled(&|s: String|s.with(Color::Yellow).bold(), label));
                    } else {
                        row(Styled(&|s: String|s.with(Color::White), label));
                    }
                }
            }));
            if self.open && let Some((_,page)) = self.pages.get() {
                column(page);
            }
        }).render(out, area)
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
                KeyCode::Up    => { self.pages.select_prev() },
                KeyCode::Down  => { self.pages.select_next() },
                KeyCode::Enter => { self.enter() },
                KeyCode::Esc   => { self.close() }
            })
        })
    });

}
