use crate::*;

/// A window border widget
#[derive(Default, Debug)]
pub struct Frame {
    pub theme:   Theme,
    pub title:   String,
    pub focused: bool,
}

impl<'a> TUI for Frame {
    fn render (&self, term: &mut dyn Write, Area(Point(x, y), Size(w, h)): Area) -> Result<()> {
        let Self { theme: Theme { bg, fg, hi, .. }, title, focused } = self;
        let top_edge    = "▇".repeat((w+2) as usize);
        let bottom_edge = "▁".repeat((w+2) as usize);
        let left_edge   = "▊";
        let right_edge  = "▎";
        let background  = " ".repeat((w+2) as usize);
        let bg = &Color::AnsiValue(235);
        term.queue(ResetColor)?
            .queue(SetBackgroundColor(Color::AnsiValue(16)))?
            .queue(SetForegroundColor(*bg))?
            .queue(MoveTo(x-1, y-1))?
            .queue(Print(&top_edge))?;
        for y in y-1..y+h+1 {
            term.queue(MoveTo(x-2, y))?
                .queue(Print(&left_edge))?;
        }
        term.queue(SetBackgroundColor(*bg))?
            .queue(SetForegroundColor(Color::AnsiValue(240)))?
            .queue(MoveTo(x-1, y+h))?.queue(Print(&bottom_edge))?;
        for y in y-1..y+h+1 {
            term.queue(MoveTo(x+w+1, y))?
                .queue(Print(&right_edge))?;
        }
        for y in y..y+h {
            term.queue(MoveTo(x-1, y))?
                .queue(Print(&background))?;
        }
            ;
            //.queue(SetBackgroundColor(*bg))?
            //.queue(SetForegroundColor(if *focused { *hi } else { *fg }))?
            //.queue(MoveTo(x, y))?
            //.queue(Print(" "))?
            //.queue(MoveTo(x+1, y))?
            //.queue(SetAttribute(Attribute::Bold))?
            //.queue(SetAttribute(Attribute::Underlined))?
            //.queue(Print(&title))?
            //.queue(SetAttribute(Attribute::Reset))?
            //.queue(MoveTo(x+1+title.len() as u16, y))?
            //.queue(SetBackgroundColor(*bg))?
            //.queue(SetForegroundColor(*fg))?
            //.queue(Print(" "))?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::str::from_utf8;
    use crate::{*, layout::*};

    #[test]
    fn test_frame () {
        let frame = Frame::default();
        assert_rendered!(frame == "\u{1b}[0m\u{1b}[38;5;232m\u{1b}[6;6H▄▄▄▄▄▄▄▄▄▄\u{1b}[15;6H▀▀▀▀▀▀▀▀▀▀\u{1b}[0m\u{1b}[48;5;232m\u{1b}[7;6H          \u{1b}[8;6H          \u{1b}[9;6H          \u{1b}[10;6H          \u{1b}[11;6H          \u{1b}[12;6H          \u{1b}[13;6H          \u{1b}[14;6H          \u{1b}[48;5;232m\u{1b}[38;5;15m\u{1b}[6;6H \u{1b}[6;7H\u{1b}[1m\u{1b}[4m\u{1b}[0m\u{1b}[6;7H\u{1b}[48;5;232m\u{1b}[38;5;15m ");
    }
}
