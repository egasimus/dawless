use super::*;

pub fn render_directory_listing (
    term: &mut dyn Write, col1: u16, row1: u16, pad: usize,
    items: &Vec<(String, (String, bool))>,
    selected: usize,
) -> Result<()> {
    let bg = Color::AnsiValue(232);
    let fg = Color::White;
    let hi = Color::Yellow;
    for (index, (_, (path, is_dir))) in items.iter().enumerate() {
        term.queue(SetAttribute(if *is_dir { Attribute::Bold } else { Attribute::Reset }))?
            .queue(SetBackgroundColor(bg))?
            .queue(SetForegroundColor(if selected == index { hi } else { fg }))?
            .queue(MoveTo(col1, row1 + index as u16))?
            .queue(Print(format!("{} {:<0width$}",
                if *is_dir { "■" } else { " " },
                path,
                width = pad as usize
            )))?;
    }
    Ok(())
}
