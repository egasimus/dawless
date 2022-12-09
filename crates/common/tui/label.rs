use super::*;

pub struct Label<'a> {
    pub bg:  Color,
    pub fg:  Color,
    pub col: u16,
    pub row: u16,
    pub text: &'a str
}

impl<'a> TUI for Label<'a> {
    fn render (&self, term: &mut dyn Write) -> Result<()> {
        term.queue(SetBackgroundColor(self.bg))?
            .queue(SetForegroundColor(self.fg))?
            .queue(MoveTo(self.col, self.row))?
            .queue(Print(self.text))?;
        Ok(())
    }
}
