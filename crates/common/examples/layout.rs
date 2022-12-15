use std::cell::RefCell;
use dawless_common::*;
use crossterm::{
    event::{poll, read, Event, KeyEvent, KeyCode},
    terminal::{size},
    style::{Color}
};

static THEME: Theme = Theme {
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

fn main () -> Result<()> {
    let mut term = std::io::stdout();
    setup(&mut term)?;
    loop {
        APP.with(|app| {
            let app = app.borrow();
            let (screen_cols, screen_rows) = size().unwrap();
            let size = app.layout().min_size().clip(Point(screen_cols, screen_rows)).unwrap();
            let Point(cols, rows) = size;
            let x = (screen_cols - cols) / 2;
            let y = (screen_rows - rows) / 2;
            let space = Space(Point(x, y), size);
            app.render(&mut term, &space).unwrap();
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

impl TUI for App {
    fn layout (&self) -> Layout {
        Layout::Column(vec![&self.component1, &self.component2])
    }
    fn render (&self, term: &mut dyn Write, space: &Space) -> Result<()> {
        let theme = Theme { bg: Color::AnsiValue(234), ..THEME };
        let Space(Point(x, y), Point(w, h)) = *space;
        let title = format!("A {w}x{h}+{x}+{y}").into();
        Frame { theme, title, focused: false }.render(term, space)?;
        self.layout().render(term, &space.inset(1).offset(Point(0, 1)))
    }
}

impl TUI for Component {
    fn layout (&self) -> Layout {
        Layout::Row(vec![&self.subcomponent1, &self.subcomponent2])
    }
    fn render (&self, term: &mut dyn Write, space: &Space) -> Result<()> {
        let theme = Theme { bg: Color::AnsiValue(235), ..THEME };
        let Space(Point(x, y), Point(w, h)) = *space;
        let title = format!("B {w}x{h}+{x}+{y}").into();
        Frame { theme, title,  focused: false }.render(term, space)?;
        self.layout().render(term, &space.inset(1).offset(Point(0, 1)))
    }
}

impl TUI for Subcomponent {
    fn layout (&self) -> Layout {
        Layout::Solid(Point(0, 0))
    }
    fn render (&self, term: &mut dyn Write, space: &Space) -> Result<()> {
        let theme = Theme { bg: Color::AnsiValue(236), ..THEME };
        let Space(Point(x, y), Point(w, h)) = *space;
        let title = format!("C {w}x{h}+{x}+{y}").into();
        Frame { theme, title, focused: false }.render(term, space)
    }
}
