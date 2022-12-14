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

thread_local!(static APP: RefCell<App> = RefCell::new(App {
    component1: Component {
        subcomponent1: Subcomponent,
        subcomponent2: Subcomponent,
    },
    component2: Component {
        subcomponent1: Subcomponent,
        subcomponent2: Subcomponent,
    },
}));

#[derive(Default)]
struct App {
    component1: Component,
    component2: Component,
}

#[derive(Default)]
struct Component {
    subcomponent1: Subcomponent,
    subcomponent2: Subcomponent
}

#[derive(Default)]
struct Subcomponent;

impl TUI for App {
    fn size (&self) -> Size {
        self.component1.size().add_w(self.component2.size()).inc_w(3)
    }
    fn render (&self, term: &mut dyn Write, space: &Space) -> Result<()> {
        let theme = Theme { bg: Color::AnsiValue(234), ..*THEME };
        let title = "App".into();
        Frame { theme, title, focused: false }.render(term, space)?;
        Layout::column(&[
            (1, &self.component1),
            (1, &self.component2)
        ]).render(term, space)
    }
}

impl TUI for Component {
    fn size (&self) -> Size {
        self.subcomponent1.size().add_h(self.subcomponent2.size()).inc_h(3)
    }
    fn render (&self, term: &mut dyn Write, space: &Space) -> Result<()> {
        let theme = Theme { bg: Color::AnsiValue(235), ..*THEME };
        let title = "Component".into();
        Frame { theme, title,  focused: false }.render(term, space)?;
        Layout::row(&[
            (1, &self.subcomponent1),
            (1, &self.subcomponent2),
        ]).render(term, space)
    }
}

impl TUI for Subcomponent {
    fn size (&self) -> Size {
        Size::from_fixed(Point(20, 5))
    }
    fn render (&self, term: &mut dyn Write, space: &Space) -> Result<()> {
        let theme = Theme { bg: Color::AnsiValue(236), ..*THEME };
        let title = "Subcomponent".into();
        Frame { theme, title, focused: false }.render(term, space)
    }
}

fn main () -> Result<()> {
    let mut term = std::io::stdout();
    setup(&mut term)?;
    loop {
        APP.with(|app| {
            let mut app = app.borrow_mut();
            let (screen_cols, screen_rows) = size().unwrap();
            let size = app.size().clip(Point(screen_cols, screen_rows)).unwrap();
            let Point(cols, rows) = size;
            let upper_left = Point((screen_cols - cols) / 2, (screen_rows - rows) / 2);
            app.render(&mut term, &Space(upper_left, size)).unwrap();
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
