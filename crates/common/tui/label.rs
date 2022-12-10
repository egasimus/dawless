use super::*;

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
    fn focus (&mut self, focus: bool) -> bool {
        self.focused = focus;
        true
    }
    fn layout (&mut self, x: u16, y: u16, w: u16, h: u16) -> Result<()> {
        self.col = x;
        self.row = y;
        Ok(())
    }
    fn render (&self, term: &mut dyn Write) -> Result<()> {
        let Theme { bg, fg, hi } = self.theme;
        term.queue(SetBackgroundColor(bg))?
            .queue(SetForegroundColor(if self.focused { hi } else { fg }))?
            .queue(MoveTo(self.col, self.row))?
            .queue(Print(&self.text))?;
        Ok(())
    }
}
