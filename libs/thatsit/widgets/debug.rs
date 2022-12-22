use crate::*;

/// A debug widget
pub struct DebugBox { pub bg: Color }

impl TUI for DebugBox {
    fn min_size (&self) -> Size {
        Size(16, 3)
    }
    fn max_size (&self) -> Size {
        Size::MAX
    }
    fn render (&self, term: &mut dyn Write, Area(Point(x, y), Size(w, h)): Area) -> Result<()> {
        let min = self.min_size();
        let max = self.max_size();
        let background = " ".repeat(w as usize);
        term.queue(SetBackgroundColor(self.bg))?
            .queue(SetForegroundColor(Color::AnsiValue(234)))?;
        for row in y..y+h {
            term.queue(MoveTo(x, row))?.queue(Print(&background))?;
        }
        let text = format!("{w}x{h}+{x}+{y}");
        let pad = w.saturating_sub(text.len() as u16) / 2;
        //term.queue(MoveTo(x+pad, y+1))?
        term.queue(MoveTo(x, y))?
            .queue(Print(&text))?;
        Ok(())
    }
}
