use super::{*, super::{*, layout::*}};

#[derive(Default, Debug)]
pub struct Label {
    pub col:     u16,
    pub row:     u16,
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
    fn size (&self) -> Size {
        let len = self.text.len() as u16;
        Size { min_w: Some(len), max_w: Some(len), min_h: Some(1), max_h: Some(1) }
    }
    fn focus (&mut self, focus: bool) -> bool {
        self.focused = focus;
        true
    }
    fn render (&self, term: &mut dyn Write, space: &Space) -> Result<()> {
        let Theme { bg, fg, hi } = self.theme;
        let Space(Point(x, y), _) = *space;
        term.queue(SetBackgroundColor(bg))?
            .queue(SetForegroundColor(if self.focused { hi } else { fg }))?
            .queue(MoveTo(x, y))?
            .queue(Print(&self.text))?;
        Ok(())
    }
}
