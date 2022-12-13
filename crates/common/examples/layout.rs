use dawless_common::*;
use crossterm::{
    event::{poll, read, Event, KeyEvent, KeyCode},
    terminal::{size},
    style::{Color}
};

fn main () -> Result<()> {
    let mut term = std::io::stdout();
    setup(&mut term)?;
    let mut app = App::default();
    loop {
        let (cols, rows) = size()?;
        let space = Space::new(0, 0, cols, rows);
        let taken = app.layout(&space)?;
        app.offset(space.w / 2 - taken.w / 2, space.h / 2 - taken.h / 2);
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
    component1: Component,
    component2: Component,
}

impl TUI for App {
    fn layout (&mut self, space: &Space) -> Result<Space> {
        let component1 = self.component1.layout(space)?;
        let component2 = self.component2.layout(&component1.right(0))?;
        self.space = component1.join(&component2);
        Ok(self.space)
    }
    fn offset (&mut self, dx: u16, dy: u16) {
        self.space = self.space.offset(dx, dy);
        for child in [&mut self.component1, &mut self.component2] {
            child.offset(dx, dy)
        }
    }
    fn render (&self, term: &mut dyn Write) -> Result<()> {
        Frame {
            space:   self.space,
            theme:   Theme { bg: Color::AnsiValue(234), ..self.theme },
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
    space: Space,
    theme: Theme,
    subcomponent1: Subcomponent,
    subcomponent2: Subcomponent
}

impl TUI for Component {
    fn layout (&mut self, space: &Space) -> Result<Space> {
        let component1 = self.subcomponent1.layout(space)?;
        let component2 = self.subcomponent2.layout(&component1.below(0))?;
        self.space = component1.join(&component2);
        Ok(self.space)
    }
    fn offset (&mut self, dx: u16, dy: u16) {
        self.space = self.space.offset(dx, dy);
        for child in [&mut self.subcomponent1, &mut self.subcomponent2] {
            child.offset(dx, dy)
        }
    }
    fn render (&self, term: &mut dyn Write) -> Result<()> {
        Frame {
            space:   self.space,//.add(1, 1, -2, -1),
            theme:   Theme { bg: Color::AnsiValue(235), ..self.theme },
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
    theme: Theme,
}

impl TUI for Subcomponent {
    fn render (&self, term: &mut dyn Write) -> Result<()> {
        Frame {
            space:   self.space,//.add(2, 2, -3, -2),
            theme:   Theme { bg: Color::AnsiValue(236), ..self.theme },
            title:   "Subcomponent".into(),
            focused: false
        }.render(term)?;
        Ok(())
    }
    fn layout (&mut self, space: &Space) -> Result<Space> {
        self.space = Space::new(space.x, space.y, 20, 5);
        Ok(self.space)
    }
    fn offset (&mut self, dx: u16, dy: u16) {
        self.space = self.space.offset(dx, dy);
    }
}
