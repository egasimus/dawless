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

    fn size (&self) -> Size {
        let len = self.text.len() as u16;
        Size { min_w: Some(len), max_w: Some(len), min_h: Some(1), max_h: Some(1) }
    }

    fn focus (&mut self, focus: bool) -> bool {
        self.focused = focus;
        true
    }
    fn layout (&mut self, Space(Point(x, y),_): &Space) -> Result<Space> {
        self.col = *x;
        self.row = *y;
        Ok(Space(Point(*x, *y), Point(self.text.len() as u16, 1)))
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
