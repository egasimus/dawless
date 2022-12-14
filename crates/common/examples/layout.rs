use std::cell::RefCell;
use dawless_common::*;
use crossterm::{
    event::{poll, read, Event, KeyEvent, KeyCode},
    terminal::{size},
    style::{Color}
};

static THEME: &'static Theme = &Theme {
    bg: Color::AnsiValue(232),
    fg: Color::White,
    hi: Color::Yellow
};

thread_local!(static APP: RefCell<App> = RefCell::new(App::init()));

fn main () -> Result<()> {
    let mut term = std::io::stdout();
    setup(&mut term)?;
    loop {
        APP.with(|app| {
            let mut app = app.borrow_mut();
            let (screen_cols, screen_rows) = size().unwrap();
            let Point(cols, rows) = app.size().clip(Point(screen_cols, screen_rows)).unwrap();
            let space = Space(Point::null(), Point(cols, rows));
            println!("SPACE {space:?}");
            app.layout(&space).unwrap();
            app.offset((screen_cols - cols) / 2, (screen_rows - rows) / 2);
            app.render(&mut term).unwrap();
        });
        term.flush()?;
        if poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code: KeyCode::Char('q'), .. }) = &read()? {
                break
            }
        }
    }
    teardown(&mut term)
}

#[derive(Default)]
struct App {
    space:      Space,
    component1: Component,
    component2: Component,
}

impl App {
    const fn init () -> Self {
        let space = Space(Point(0, 0), Point(0, 0));
        Self {
            space,
            component1: Component {
                space,
                subcomponent1: Subcomponent { space },
                subcomponent2: Subcomponent { space }
            },
            component2: Component {
                space,
                subcomponent1: Subcomponent { space },
                subcomponent2: Subcomponent { space }
            },
        }
    }
}

impl TUI for App {
    fn size (&self) -> Size {
        self.component1.size().add_w(self.component2.size()).inc_w(3)
    }
    fn layout (&mut self, Space(_, Point(w, h)): &Space) -> Result<Space> {
        let Point(w, h) = self.size().clip(Point(*w, *h))?;
        let space = Space::new(0, 0, w, h);
        let component1 = self.component1.layout(&space)?;
        let component2 = self.component2.layout(&component1.right(1))?;
        self.space = component1.join(&component2);
        Ok(self.space)
    }
    fn offset (&mut self, dx: u16, dy: u16) {
        self.space = self.space.offset(Point(dx, dy));
        for child in [&mut self.component1, &mut self.component2] {
            child.offset(dx, dy)
        }
    }
    fn render (&self, term: &mut dyn Write) -> Result<()> {
        Frame {
            space:   self.space,
            theme:   Theme { bg: Color::AnsiValue(234), ..*THEME },
            title:   "App".into(),
            focused: false
        }.render(term)?;
        for child in [&self.component1, &self.component2] {
            child.render(term)?;
        }
        Ok(())
    }
}

#[derive(Default)]
struct Component {
    space:         Space,
    subcomponent1: Subcomponent,
    subcomponent2: Subcomponent
}

impl TUI for Component {
    fn size (&self) -> Size {
        self.subcomponent1.size().add_h(self.subcomponent2.size()).inc_h(3)
    }
    fn layout (&mut self, space: &Space) -> Result<Space> {
        let component1 = self.subcomponent1.layout(space)?;
        let component2 = self.subcomponent2.layout(&component1.below(1))?;
        self.space = component1.join(&component2);
        Ok(self.space)
    }
    fn offset (&mut self, dx: u16, dy: u16) {
        self.space = self.space.offset(Point(dx, dy));
        for child in [&mut self.subcomponent1, &mut self.subcomponent2] {
            child.offset(dx, dy)
        }
    }
    fn render (&self, term: &mut dyn Write) -> Result<()> {
        Frame {
            space:   self.space,//.add(1, 1, -2, -1),
            theme:   Theme { bg: Color::AnsiValue(235), ..*THEME },
            title:   "Component".into(),
            focused: false
        }.render(term)?;
        for child in [&self.subcomponent1, &self.subcomponent2] {
            child.render(term)?;
        }
        Ok(())
    }
}

#[derive(Default)]
struct Subcomponent {
    space: Space,
}

impl TUI for Subcomponent {
    fn size (&self) -> Size {
        Size::from_fixed((20, 5).into())
    }
    fn offset (&mut self, dx: u16, dy: u16) {
        self.space = self.space.offset(Point(dx, dy));
    }
    fn layout (&mut self, Space(offset, max_size): &Space) -> Result<Space> {
        println!("MAX SIZE {max_size:?}, SIZE {:?}", self.size());
        self.space = Space(*offset, self.size().clip(*max_size)?);
        Ok(self.space)
    }
    fn render (&self, term: &mut dyn Write) -> Result<()> {
        Frame {
            space:   self.space,//.add(2, 2, -3, -2),
            theme:   Theme { bg: Color::AnsiValue(236), ..*THEME },
            title:   "Subcomponent".into(),
            focused: false
        }.render(term)?;
        Ok(())
    }
}
