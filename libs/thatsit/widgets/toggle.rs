use crate::*;

#[derive(Default, Debug)]
pub struct Toggle<T: TUI, U: TUI> {
    pub theme:  Theme,
    pub closed: T,
    pub open:   U,
    state: bool,
}

impl<T: TUI, U: TUI> Toggle<T, U> {
    pub fn new (closed: T, open: U) -> Self {
        Self {
            theme:  Theme::default(),
            state: false,
            closed,
            open
        }
    }
    pub fn toggle (&mut self) {
        self.state = !self.state
    }
    pub fn get (&mut self) -> bool {
        self.state
    }
    pub fn set (&mut self, value: bool) {
        self.state = value
    }
    pub fn closed (&self) -> &T {
        &self.closed
    }
    pub fn closed_mut (&mut self) -> &mut T {
        &mut self.closed
    }
    pub fn open (&mut self) -> &U {
        &self.open
    }
    pub fn open_mut (&mut self) -> &mut U {
        &mut self.open
    }
}

impl<T: TUI, U: TUI> TUI for Toggle<T, U> {
    fn min_size (&self) -> Size {
        if self.state {
            self.open.min_size()
        } else {
            self.closed.min_size()
        }
    }
    fn max_size (&self) -> Size {
        if self.state {
            self.open.max_size()
        } else {
            self.closed.max_size()
        }
    }
    fn focus (&mut self, focus: bool) -> bool {
        if self.state {
            self.open.focus(focus)
        } else {
            self.closed.focus(focus)
        }
    }
    fn focused (&self) -> bool {
        if self.state {
            self.open.focused()
        } else {
            self.closed.focused()
        }
    }
    fn render (&self, term: &mut dyn Write, rect: Area) -> Result<()> {
        if self.state {
            self.open.render(term, rect)
        } else {
            self.closed.render(term, rect)
        }
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        if self.state {
            self.open.handle(event)
        } else {
            self.closed.handle(event)
        }
        //Ok(match event {
            //Event::Key(KeyEvent { code: KeyCode::Enter, .. }) => {
                //self.state = !self.state;
                //true
            //},
            //Event::Key(KeyEvent { code: KeyCode::Char(' '), .. }) => {
                //self.state = !self.state;
                //true
            //},
            //_ => false
        //})
    }
}

#[derive(Debug, Default)]
pub struct Collapsible(pub Toggle<Button, Box<dyn TUI>>);

impl TUI for Collapsible {
    fn min_size (&self) -> Size {
        self.0.min_size()
    }
    fn max_size (&self) -> Size {
        self.0.max_size()
    }
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        self.0.render(term, area)
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        Ok(match_key!((event) {
            KeyCode::Enter => { self.0.toggle(); true }
        }))
    }
    fn focus (&mut self, focus: bool) -> bool {
        self.0.focus(focus)
    }
    fn focused (&self) -> bool {
        self.0.focused()
    }
    fn layout <'a> (&'a self) -> Thunk<'a> {
        self.0.layout()
    }
}
