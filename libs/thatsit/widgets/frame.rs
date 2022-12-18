use crate::*;

/// A window border widget
#[derive(Default, Debug)]
pub struct Frame {
    pub theme:   Theme,
    pub title:   String,
    pub focused: bool,
}

impl TUI for Frame {
    fn render (&self, term: &mut dyn Write, Area(Point(x, y), Size(w, h)): Area) -> Result<()> {
        let Self { theme: Theme { bg, fg, hi, .. }, title, focused } = self;
        term.queue(ResetColor)?
            .queue(SetForegroundColor(*bg))?
            .queue(MoveTo(x, y))?
            .queue(Print("▄".repeat(w as usize)))?;
        let background = "▀".repeat(w as usize);
        term.queue(MoveTo(x, y+h-1))?
            .queue(Print(&background))?
            .queue(ResetColor)?
            .queue(SetBackgroundColor(*bg))?;
        let background = " ".repeat(w as usize);
        for row in y+1..y+h-1 {
            term.queue(MoveTo(x, row))?
                .queue(Print(&background))?;
        }
        term.queue(SetBackgroundColor(*bg))?
            .queue(SetForegroundColor(if *focused { *hi } else { *fg }))?
            .queue(MoveTo(x, y))?
            .queue(Print(" "))?
            .queue(MoveTo(x+1, y))?
            .queue(SetAttribute(Attribute::Bold))?
            .queue(SetAttribute(Attribute::Underlined))?
            .queue(Print(&title))?
            .queue(SetAttribute(Attribute::Reset))?
            .queue(MoveTo(x+1+title.len() as u16, y))?
            .queue(SetBackgroundColor(*bg))?
            .queue(SetForegroundColor(*fg))?
            .queue(Print(" "))?;
        Ok(())
    }
}
