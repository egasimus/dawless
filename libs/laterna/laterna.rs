use std::io::Write;
use thatsit::{*, layouts::*, engines::tui::{*, crossterm::{QueueableCommand, cursor::*, style::*}}};

pub const KEYS: [&'static str; 6] = [
  "▀█ ",
  "▀█ ",
  "▀█ ",
  "██ ",
  "▄█ ",
  "▄█ ",
];

pub struct PianoRoll(pub Vec<(usize, usize)>);

impl<W: Write> Output<TUI<W>, [u16;2]> for PianoRoll {

    fn render (&self, engine: &mut TUI<W>) -> Result<Option<[u16;2]>> {
        let area = engine.area;
        let grid = "█               █               █               █               ";

        for i in 0..13 {
            engine.output
                .queue(SetBackgroundColor(Color::AnsiValue(234)))?
                .queue(SetForegroundColor(Color::AnsiValue(255)))?
                .queue(MoveTo(area.x(), area.y() + i))?
                .queue(Print(KEYS[((i + 3) % 6) as usize]))?
                .queue(SetForegroundColor(Color::AnsiValue(233)))?
                .queue(MoveTo(area.x() + 3, area.y() + i))?
                .queue(Print(grid))?;
        }

        engine.output.queue(SetForegroundColor(Color::AnsiValue(200)))?;

        for (time, pitch) in self.0.iter() {
            let x = area.x() + 3 + *time as u16;
            let y = area.y() + (*pitch as u16 + *time as u16) % 12;
            let c = if pitch % 2 == 1 {"▀"} else {"▄"};
            engine.output.queue(MoveTo(x, y))?.queue(Print(c))?;
        }

        engine.output
            .queue(SetForegroundColor(Color::Green))?
            //.queue(SetForegroundColor(Color::AnsiValue(200)))?
            .queue(MoveTo(area.x() + 15, area.y() + 6))?.queue(Print("▀ ▄▄"))?
            .queue(MoveTo(area.x() + 19, area.y() + 7))?.queue(Print("▄▄  ▄▄"))?
            .queue(MoveTo(area.x() + 25, area.y() + 8))?.queue(Print("▄▄▄▄  "))?;

        Ok::<Option<[u16;2]>, Box<dyn std::error::Error>>(Some(area.size()))
    }

}

//pub fn demo (out: &mut dyn Write, area.x(): u16) -> Result<()> {
    //let area.y() = 10;
    //out.queue(SetBackgroundColor(Color::AnsiValue(234)))?
        //.queue(SetForegroundColor(Color::AnsiValue(255)))?;
    //for i in 0..13 {
        //out.queue(MoveTo(area.x(), area.y() + i))?
            //.queue(Print(KEYS[((i + 3) % 6) as usize]))?;
    //}
    //out.queue(SetForegroundColor(Color::AnsiValue(233)))?;
    //let grid = "█               █               █               █               ";
    //for i in 0..13 {
        //out.queue(MoveTo(area.x() + 3, area.y() + i))?
            //.queue(Print(grid))?;
    //}
    //out.queue(SetForegroundColor(Color::AnsiValue(200)))?
        //.queue(MoveTo(area.x() + 15, 16))?.queue(Print("▀ ▄▄"))?
        //.queue(MoveTo(area.x() + 19, 17))?.queue(Print("▄▄  ▄▄"))?
        //.queue(MoveTo(area.x() + 25, 18))?.queue(Print("▄▄▄▄  "))?;
    //Ok(())
//}
