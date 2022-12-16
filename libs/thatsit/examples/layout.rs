use std::cell::RefCell;
use thatsit::{*, crossterm::{
    event::{poll, read, Event, KeyEvent, KeyCode},
    terminal::{size},
    style::{Color}
}};

thread_local!(static APP: RefCell<App> = RefCell::new(App {
    frame: bg(234),
    component1: Component {
        frame: bg(235),
        subcomponent1: Subcomponent { frame: bg(236), },
        subcomponent2: Subcomponent { frame: bg(236), },
    },
    component2: Component {
        frame: bg(235),
        subcomponent1: Subcomponent { frame: bg(236), },
        subcomponent2: Subcomponent { frame: bg(236), },
    },
}));

const fn bg (color: u8) -> Frame {
    Frame {
        theme: Theme { bg: Color::AnsiValue(color), ..THEME },
        title: String::new(),
        focused: false
    }
}

const THEME: Theme = Theme {
    bg: Color::AnsiValue(232),
    fg: Color::White,
    hi: Color::Yellow
};

#[derive(Default)]
struct App {
    frame: Frame,
    component1: Component,
    component2: Component,
}

#[derive(Default)]
struct Component {
    frame: Frame,
    subcomponent1: Subcomponent,
    subcomponent2: Subcomponent
}

#[derive(Default)]
struct Subcomponent {
    frame: Frame,
}

fn main () -> Result<()> {
    let mut term = std::io::stdout();
    setup(&mut term)?;
    loop {
        APP.with(|app| {
            let app = app.borrow();
            let (screen_cols, screen_rows) = size().unwrap();
            let size = app.layout().size().clip(Point(screen_cols, screen_rows)).unwrap();
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
        Layout::Layers(Sizing::Auto, vec![
            Layout::Item(Sizing::Auto, &self.frame),
            Layout::Column(Sizing::Auto, vec![
                Layout::Item(Sizing::Auto, &self.component1),
                Layout::Item(Sizing::Auto, &self.component2)
            ])
        ])
    }
}

impl TUI for Component {
    fn layout (&self) -> Layout {
        Layout::Layers(Sizing::Auto, vec![
            Layout::Item(Sizing::Auto, &self.frame),
            Layout::Column(Sizing::Auto, vec![
                Layout::Item(Sizing::Auto, &self.subcomponent1),
                Layout::Item(Sizing::Auto, &self.subcomponent2)
            ])
        ])
    }
}

impl TUI for Subcomponent {
    fn layout (&self) -> Layout {
        Layout::Blank(Sizing::Fixed(Point(30, 20)))
    }
    fn render (&self, term: &mut dyn Write, space: &Space) -> Result<()> {
        let theme = Theme { bg: Color::AnsiValue(236), ..THEME };
        let Space(Point(x, y), Point(w, h)) = *space;
        let title = format!("C {w}x{h}+{x}+{y}").into();
        Frame { theme, title, focused: false }.render(term, space)
    }
}

