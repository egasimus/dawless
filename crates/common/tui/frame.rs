use super::*;

#[derive(Default, Debug)]
pub struct Frame {
    pub space:   Space,
    pub theme:   Theme,
    pub title:   String,
    pub focused: bool,
}

impl TUI for Frame {
    fn render (&self, term: &mut dyn Write) -> Result<()> {
        let Self {
            theme: Theme { bg, fg, hi, .. },
            space: Space(Point(x, y), Point(w, h)),
            ..
        } = *self;

        term.queue(ResetColor)?
            .queue(SetForegroundColor(bg))?
            .queue(MoveTo(x, y))?
            .queue(Print("▄".repeat(w as usize)))?
            .queue(ResetColor)?
            .queue(SetBackgroundColor(bg))?;

        let background = " ".repeat(w as usize);
        for row in y+1..y+h {
            term.queue(MoveTo(x, row))?.queue(Print(&background))?;
        }

        term.queue(SetBackgroundColor(if self.focused { hi } else { bg }))?
            .queue(SetForegroundColor(if self.focused { bg } else { fg }))?
            .queue(MoveTo(x, y))?
            .queue(Print(" "))?
            .queue(MoveTo(x+1, y))?
            .queue(SetAttribute(Attribute::Bold))?
            .queue(SetAttribute(Attribute::Underlined))?
            .queue(Print(&self.title))?
            .queue(SetAttribute(Attribute::Reset))?
            .queue(MoveTo(x+1+self.title.len() as u16, y))?
            .queue(SetBackgroundColor(bg))?
            .queue(SetForegroundColor(fg))?
            .queue(Print(" "))?;

        Ok(())
    }
}
