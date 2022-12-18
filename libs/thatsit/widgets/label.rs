use crate::*;

#[derive(Default, Debug)]
pub struct Label {
    pub theme:   Theme,
    pub focused: bool,
    pub text:    String
}

impl Label {
    pub fn new (text: impl Into<String>) -> Self {
        Self { text: text.into(), ..Self::default() }
    }
}

impl TUI for Label {
    fn layout (&self) -> Layout {
        Layout::Item(Sizing::Fixed(self.min_size()), &Blank {})
    }
    fn min_size (&self) -> Size {
        Size(self.text.len() as u16, 1)
    }
    fn max_size (&self) -> Size {
        self.min_size()
    }
    fn focus (&mut self, focus: bool) -> bool {
        self.focused = focus;
        true
    }
    fn render (&self, term: &mut dyn Write, Area(Point(x, y), _): Area) -> Result<()> {
        let Theme { bg, fg, hi } = self.theme;
        term.queue(SetBackgroundColor(bg))?
            .queue(SetForegroundColor(if self.focused { hi } else { fg }))?
            .queue(MoveTo(x, y))?
            .queue(Print(&self.text))?;
        Ok(())
    }
}
