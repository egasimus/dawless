use std::io::{Write, Result};
use crossterm::{QueueableCommand, cursor::*, style::*};

const KEYS: [&'static str; 6] = [
  "▀█ ",
  "▀█ ",
  "▀█ ",
  "██ ",
  "▄█ ",
  "▄█ ",
];

pub fn demo (term: &mut dyn Write, col1: u16) -> Result<()> {
    let row1 = 10;
    term.queue(SetBackgroundColor(Color::AnsiValue(234)))?
        .queue(SetForegroundColor(Color::AnsiValue(255)))?;
    for i in 0..13 {
        term.queue(MoveTo(col1, row1 + i))?
            .queue(Print(KEYS[((i + 3) % 6) as usize]))?;
    }
    term.queue(SetForegroundColor(Color::AnsiValue(233)))?;
    let grid = "█               █               █               █               ";
    for i in 0..13 {
        term.queue(MoveTo(col1 + 3, row1 + i))?
            .queue(Print(grid))?;
    }
    term.queue(SetForegroundColor(Color::AnsiValue(200)))?
        .queue(MoveTo(col1 + 15, 16))?.queue(Print("▀ ▄▄"))?
        .queue(MoveTo(col1 + 19, 17))?.queue(Print("▄▄  ▄▄"))?
        .queue(MoveTo(col1 + 25, 18))?.queue(Print("▄▄▄▄  "))?;
    Ok(())
}
