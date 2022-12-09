use super::*;

pub fn render_frame (
    term: &mut dyn Write, col1: u16, row1: u16, cols: u16, rows: u16,
    bg: Color, title: Option<(Color, Color, &str)>
) -> Result<()> {

    term.queue(ResetColor)?
        .queue(SetForegroundColor(bg))?
        .queue(MoveTo(col1, row1))?
        .queue(Print("▄".repeat(cols as usize)))?
        .queue(ResetColor)?
        .queue(SetBackgroundColor(bg))?;

    let background = " ".repeat(cols as usize);
    for row in row1+1..row1+rows {
        term.queue(MoveTo(col1, row))?.queue(Print(&background))?;
    }

    if let Some((bg, fg, text)) = title {
        term.queue(SetBackgroundColor(bg))?
            .queue(SetForegroundColor(fg))?
            .queue(MoveTo(col1, row1))?
            .queue(Print(" "))?
            .queue(MoveTo(col1+1, row1))?
            .queue(SetAttribute(Attribute::Bold))?
            .queue(SetAttribute(Attribute::Underlined))?
            .queue(Print(text))?
            .queue(SetAttribute(Attribute::Reset))?
            .queue(MoveTo(col1+1+text.len() as u16, row1))?
            .queue(SetBackgroundColor(bg))?
            .queue(SetForegroundColor(fg))?
            .queue(Print(" "))?;
    }

    Ok(())

}

pub struct Frame <'a> {
    pub rect:    Rect,
    pub theme:   Theme,
    pub title:   &'a str,
    pub focused: bool,
}

impl<'a> TUI for Frame<'a> {
    fn render (&self, term: &mut dyn Write) -> Result<()> {
        let Theme { bg, fg, hi } = self.theme;
        let (col1, row1, cols, rows) = self.rect;
        render_frame(term, col1, row1, cols, rows, bg, Some((
            if self.focused { hi } else { bg },
            if self.focused { bg } else { hi },
            &self.title
        )))
    }
}

impl<'a> FnOnce<(&mut dyn Write,)> for Frame<'a> {
    type Output = Result<()>;
    extern "rust-call" fn call_once (self, args: (&mut dyn Write,)) -> Self::Output {
        self.render(args.0)
    }
}
impl<'a> FnMut<(&mut dyn Write,)> for Frame<'a> {
    extern "rust-call" fn call_mut (&mut self, args: (&mut dyn Write,)) -> Self::Output {
        self.render(args.0)
    }
}
impl<'a> Fn<(&mut dyn Write,)> for Frame<'a> {
    extern "rust-call" fn call (&self, args: (&mut dyn Write,)) -> Self::Output {
        self.render(args.0)
    }
}
impl<'a> FnOnce<(&Event,)> for Frame<'a> {
    type Output = Result<bool>;
    extern "rust-call" fn call_once (mut self, args: (&Event,)) -> Self::Output {
        self.handle(args.0)
    }
}
