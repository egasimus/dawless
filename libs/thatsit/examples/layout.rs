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
            let layout = app.layout();
            let min_size = layout.min_size();
            let screen_size: Size = size().unwrap().into();
            if let Err(e) = min_size.fits_in(screen_size) {
                write_error(&mut term, format!("{e}").as_str()).unwrap();
            } else {
                let max_size = layout.max_size();
                let size = screen_size.crop_to(max_size);
                let x = (screen_size.0 - size.0) / 2;
                let y = (screen_size.0 - size.0) / 2;
                app.render(&mut term, Area(Point(x, y), size)).unwrap();
            }
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
        Layout::Layers(Sizing::AUTO, vec![
            Layout::Item(Sizing::AUTO, &self.frame),
            Layout::Column(Sizing::AUTO, vec![
                Layout::Item(Sizing::AUTO, &self.component1),
                Layout::Item(Sizing::AUTO, &self.component2)
            ])
        ])
    }
}

impl TUI for Component {
    fn layout (&self) -> Layout {
        Layout::Layers(Sizing::AUTO, vec![
            Layout::Item(Sizing::AUTO, &self.frame),
            Layout::Column(Sizing::AUTO, vec![
                Layout::Item(Sizing::AUTO, &self.subcomponent1),
                Layout::Item(Sizing::AUTO, &self.subcomponent2)
            ])
        ])
    }
}

impl TUI for Subcomponent {
    fn layout (&self) -> Layout {
        Layout::Item(Sizing::AUTO, &self.frame)
    }
}

