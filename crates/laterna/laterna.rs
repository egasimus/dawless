use std::io::{Write, Result};
use crossterm::{QueueableCommand, cursor::*, style::*};

pub fn demo (term: &mut dyn Write, col1: u16) -> Result<()> {
    let grid = "█               █               █               █               ";
    term.queue(SetBackgroundColor(Color::AnsiValue(234)))?
        .queue(SetForegroundColor(Color::AnsiValue(255)))?
        .queue(MoveTo(col1, 10))?.queue(Print("██ "))?
        .queue(MoveTo(col1, 11))?.queue(Print("▄█ "))?
        .queue(MoveTo(col1, 12))?.queue(Print("▄█ "))?
        .queue(MoveTo(col1, 13))?.queue(Print("▀█ "))?
        .queue(MoveTo(col1, 14))?.queue(Print("▀█ "))?
        .queue(MoveTo(col1, 15))?.queue(Print("▀█ "))?
        .queue(MoveTo(col1, 16))?.queue(Print("██ "))?
        .queue(MoveTo(col1, 17))?.queue(Print("▄█ "))?
        .queue(MoveTo(col1, 18))?.queue(Print("▄█ "))?
        .queue(MoveTo(col1, 19))?.queue(Print("▀█ "))?
        .queue(MoveTo(col1, 20))?.queue(Print("▀█ "))?
        .queue(MoveTo(col1, 21))?.queue(Print("▀█ "))?
        .queue(MoveTo(col1, 22))?.queue(Print("██ "))?
        .queue(SetForegroundColor(Color::AnsiValue(233)))?
        .queue(MoveTo(col1 + 3, 10))?.queue(Print(grid))?
        .queue(MoveTo(col1 + 3, 11))?.queue(Print(grid))?
        .queue(MoveTo(col1 + 3, 12))?.queue(Print(grid))?
        .queue(MoveTo(col1 + 3, 13))?.queue(Print(grid))?
        .queue(MoveTo(col1 + 3, 14))?.queue(Print(grid))?
        .queue(MoveTo(col1 + 3, 15))?.queue(Print(grid))?
        .queue(MoveTo(col1 + 3, 16))?.queue(Print(grid))?
        .queue(MoveTo(col1 + 3, 17))?.queue(Print(grid))?
        .queue(MoveTo(col1 + 3, 18))?.queue(Print(grid))?
        .queue(MoveTo(col1 + 3, 19))?.queue(Print(grid))?
        .queue(MoveTo(col1 + 3, 20))?.queue(Print(grid))?
        .queue(MoveTo(col1 + 3, 21))?.queue(Print(grid))?
        .queue(MoveTo(col1 + 3, 22))?.queue(Print(grid))?
        .queue(SetForegroundColor(Color::AnsiValue(200)))?
        .queue(MoveTo(col1 + 15, 16))?.queue(Print("▀ ▄▄"))?
        .queue(MoveTo(col1 + 19, 17))?.queue(Print("▄▄  ▄▄"))?
        .queue(MoveTo(col1 + 25, 18))?.queue(Print("▄▄▄▄  "))?;
    Ok(())
}
