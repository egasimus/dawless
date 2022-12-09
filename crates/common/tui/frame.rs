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
    for row in row1+1..row1+rows-1 {
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
