use super::*;

#[derive(Default, Debug)]
pub struct Toggle<T: TUI, U: TUI> {
    pub rect:   Rect,
    pub theme:  Theme,
    pub toggle: bool,
    pub closed: T,
    pub open:   U
}

impl<T: TUI, U: TUI> Toggle<T, U> {
    pub fn new (closed: T, open: U) -> Self {
        Self {
            rect:   Rect::default(),
            theme:  Theme::default(),
            toggle: false,
            closed,
            open
        }
    }
}

impl<T: TUI, U: TUI> TUI for Toggle<T, U> {
    fn focus (&mut self, focus: bool) -> bool {
        if self.toggle {
            self.open.focus(focus)
        } else {
            self.closed.focus(focus)
        }
    }
    fn layout (&mut self, x: u16, y: u16, w: u16, h: u16) -> Result<()> {
        self.open.layout(x, y, w, h)?;
        self.closed.layout(x, y, w, h)?;
        Ok(())
    }
    fn render (&self, term: &mut dyn Write) -> Result<()> {
        if self.toggle {
            self.open.render(term)
        } else {
            self.closed.render(term)
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
