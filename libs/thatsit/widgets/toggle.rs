use crate::{*, layout::*};

#[derive(Default, Debug)]
pub struct Toggle<T: TUI, U: TUI> {
    pub theme:  Theme,
    pub toggle: bool,
    pub closed: T,
    pub open:   U
}

impl<T: TUI, U: TUI> Toggle<T, U> {
    pub fn new (closed: T, open: U) -> Self {
        Self {
            theme:  Theme::default(),
            toggle: false,
            closed,
            open
        }
    }
}

impl<T: TUI, U: TUI> TUI for Toggle<T, U> {
    fn layout (&self) -> Layout {
        if self.toggle {
            self.open.layout()
        } else {
            self.closed.layout()
        }
    }
    fn focus (&mut self, focus: bool) -> bool {
        if self.toggle {
            self.open.focus(focus)
        } else {
            self.closed.focus(focus)
        }
    }
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        if self.toggle {
            self.open.render(term, area)
        } else {
            self.closed.render(term, area)
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
