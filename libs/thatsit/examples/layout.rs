use std::cell::RefCell;
use thatsit::{*, crossterm::{
    event::{poll, read, Event, KeyEvent, KeyCode},
    terminal::{size},
    style::{Color}
}};

thread_local!(static APP: RefCell<App> = RefCell::new(App {
    frame: Inset(1),
    component1: Component {
        frame: Inset(1),
        subcomponent1: Subcomponent { frame: Inset(1), },
        subcomponent2: Subcomponent { frame: Inset(1), },
    },
    component2: Component {
        frame: Inset(1),
        subcomponent1: Subcomponent { frame: Inset(1), },
        subcomponent2: Subcomponent { frame: Inset(1), },
    },
}));

#[derive(Default)]
struct App {
    frame: Inset,
    component1: Component,
    component2: Component,
}

#[derive(Default)]
struct Component {
    frame: Inset,
    subcomponent1: Subcomponent,
    subcomponent2: Subcomponent
}

#[derive(Default)]
struct Subcomponent {
    frame: Inset,
}

fn main () -> Result<()> {
    let term = &mut std::io::stdout();
    setup(term, true)?;
    loop {
        APP.with(|app| {
            let app = app.borrow();
            //let layout = app.layout();
            let screen_size: Size = size().unwrap().into();
            match app.layout(screen_size) {
                Ok(layout) => if let Err(error) = layout.render(
                    term, Area(Point::MIN, screen_size)
                ) {
                    write_error(term, format!("{error}").as_str()).unwrap();
                },
                Err(error) => {
                    write_error(term, format!("{error}").as_str()).unwrap();
                }
            }
        });
        term.flush()?;
        if poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code: KeyCode::Char('q'), .. }) = &read()? {
                break
            }
        }
    }
    teardown(term)?;
    Ok(())
}

impl TUI for App {
    tui! {
        layout (self, max) {
            Ok(Layout::layers(|add|{
                add(&self.frame);
                Layout::columns(|add|{
                    add(&self.component1);
                    add(&self.component2);
                });
            }))
        }
    }
}

impl TUI for Component {
    tui! {
        layout (self, max) {
            Ok(Layout::layers(|add|{
                add(&self.frame);
                Layout::columns(|add|{
                    add(&self.subcomponent1);
                    add(&self.subcomponent2);
                });
            }))
        }
    }
}

impl TUI for Subcomponent {
    tui! {
        layout (self, max) {
            Ok(Layout::layers(|add|{ add(&self.frame); }))
        }
    }
}

