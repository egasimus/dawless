use dawless_common::*;
use crossterm::{
    event::{poll, read, Event, KeyEvent, KeyCode},
    terminal::{size}
};

fn main () -> Result<()> {
    let mut term = std::io::stdout();
    setup(&mut term)?;
    let mut app = App::default();
    loop {
        let (cols, rows) = size()?;
        app.layout(&Space::new(0, 0, cols, rows))?;
        app.render(&mut term)?;
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
    space: Space,
    theme: Theme,
    frame: Frame,
    component1: Component,
    component2: Component,
}

impl TUI for App {
    fn render (&self, term: &mut dyn Write) -> Result<()> {
        self.frame.render(term);
        Ok(())
    }
}

#[derive(Default)]
struct Component {
    space: Space,
    theme: Theme,
    subcomponent1: Subcomponent,
    subcomponent2: Subcomponent
}

impl TUI for Component {
    fn render (&self, term: &mut dyn Write) -> Result<()> { Ok(()) }
}

#[derive(Default)]
struct Subcomponent {
    space: Space,
    theme: Theme,
}

impl TUI for Subcomponent {
    fn render (&self, term: &mut dyn Write) -> Result<()> { Ok(()) }
}
