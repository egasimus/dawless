use crate::tui::{*, layout::*, widgets::*};

#[derive(Default, Debug)]
pub struct Toggle<T: TUI, U: TUI> {
    pub space:  Space,
    pub theme:  Theme,
    pub toggle: bool,
    pub closed: T,
    pub open:   U
}

impl<T: TUI, U: TUI> Toggle<T, U> {
    pub fn new (closed: T, open: U) -> Self {
        Self {
            space:   Space::default(),
            theme:  Theme::default(),
            toggle: false,
            closed,
            open
        }
    }
}

impl<T: TUI, U: TUI> TUI for Toggle<T, U> {
    fn size (&self) -> Size {
        if self.toggle {
            self.open.size()
        } else {
            self.closed.size()
        }
    }
    fn focus (&mut self, focus: bool) -> bool {
        if self.toggle {
            self.open.focus(focus)
        } else {
            self.closed.focus(focus)
        }
    }
    fn render (&self, term: &mut dyn Write, space: &Space) -> Result<()> {
        if self.toggle {
            self.open.render(term, space)
        } else {
            self.closed.render(term, space)
        }
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        Ok(match event {
            Event::Key(KeyEvent { code: KeyCode::Enter, .. }) => {
                self.toggle = !self.toggle;
                true
            },
            Event::Key(KeyEvent { code: KeyCode::Char(' '), .. }) => {
                self.toggle = !self.toggle;
                true
            },
            _ => false
        })
    }
}
