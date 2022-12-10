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
    component1: Component,
    component2: Component,
}

impl TUI for App {
    fn render (&self, term: &mut dyn Write) -> Result<()> {
        Frame {
            space:   self.space,
            theme:   Theme { bg: Color::Red, ..self.theme },
            title:   "App".into(),
            focused: false
        }.render(term)?;
        self.component1.render(term)?;
        self.component2.render(term)?;
        Ok(())
    }
    fn layout (&mut self, space: &Space) -> Result<Space> {
        let component1 = self.component1.layout(space)?;
        let component2 = self.component2.layout(&component1.right(1))?;
        self.space = component1.join(&component2);
        let Space { w, h, .. } = space;
        let offset_x = w / 2 - self.space.w / 2;
        let offset_y = h / 2 - self.space.h / 2;
        self.space = self.space.offset(offset_x, offset_y);
        self.component1.space = self.component1.space.offset(offset_x, offset_y);
        self.component1.subcomponent1.space = self.component1.subcomponent1.space.offset(offset_x, offset_y);
        self.component1.subcomponent2.space = self.component1.subcomponent2.space.offset(offset_x, offset_y);
        self.component2.space = self.component2.space.offset(offset_x, offset_y);
        self.component2.subcomponent1.space = self.component2.subcomponent1.space.offset(offset_x, offset_y);
        self.component2.subcomponent2.space = self.component2.subcomponent2.space.offset(offset_x, offset_y);
        Ok(self.space)
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
    fn render (&self, term: &mut dyn Write) -> Result<()> {
        Frame {
            space:   self.space.inset(1),
            theme:   Theme { bg: Color::Green, ..self.theme },
            title:   "Component".into(),
            focused: false
        }.render(term)?;
        self.subcomponent1.render(term)?;
        self.subcomponent2.render(term)?;
        Ok(())
    }
    fn layout (&mut self, space: &Space) -> Result<Space> {
        let component1 = self.subcomponent1.layout(space)?;
        let component2 = self.subcomponent2.layout(&component1.below(1))?;
        self.space = component1.join(&component2);
        Ok(self.space)
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
            space:   self.space.inset(1),
            theme:   Theme { bg: Color::Blue, ..self.theme },
            title:   "Subcomponent".into(),
            focused: false
        }.render(term)?;
        Ok(())
    }
    fn layout (&mut self, space: &Space) -> Result<Space> {
        self.space = Space::new(space.x, space.y, 20, 5);
        Ok(self.space)
    }
}
