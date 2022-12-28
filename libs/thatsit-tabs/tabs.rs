use std::io::Result;
use thatsit::{
    *,
    crossterm::{
        self,
        cursor::MoveTo,
        event::Event,
        style::{Print, Color, SetBackgroundColor, SetForegroundColor}
    }
};
use thatsit_focus::*;

#[derive(Default, Debug)]
pub struct TabbedVertical<T: TUI> {
    pub pages:   FocusState<(String, T)>,
    pub open:    bool,
    pub entered: bool
}

impl<T: TUI> TabbedVertical<T> {
    /// Create a new selector with vertical tabs from a list of `(Button, TUI)` pairs.
    pub fn new (pages: Vec<(String, T)>) -> Self {
        let mut pages = FocusState::new(pages);
        pages.select_next();
        Self { pages, open: false, entered: false }
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

impl<T: TUI> TUI for TabbedVertical<T> {
    fn layout <'a> (&'a self, _: Size) -> Result<Thunk<'a>> {
        Ok(row(|add|{
            add(col(|add|{ for (label, _) in self.pages.iter() { add(label); } }));
            if self.open && let Some((_,page)) = self.pages.get() { add(page); }
        }))
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
            match_key!((event) {
                KeyCode::Up    => { self.pages.select_prev() },
                KeyCode::Down  => { self.pages.select_next() },
                KeyCode::Enter => { self.enter() },
                KeyCode::Esc   => { self.close() }
            })
        })
    }
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        let layout = self.layout(area.1)?;
        layout.render(term, area)?;
        term.queue(SetBackgroundColor(Color::AnsiValue(123)))?
            .queue(SetForegroundColor(Color::AnsiValue(234)))?;
        for (label, _) in self.pages.iter() {
            term.queue(MoveTo(area.x(), area.y()))?
                .queue(Print("FOO"))?;
        }
        Ok(())
    }
}
