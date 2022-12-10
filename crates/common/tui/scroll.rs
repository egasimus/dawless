use super::*;

pub struct Scrollbar {
    pub rect:   Rect,
    pub theme:  Theme,
    pub length: usize,
    pub offset: usize
}

impl TUI for Scrollbar {
    fn render (&self, term: &mut dyn Write) -> Result<()> {
        let Self { rect, theme, length, offset, .. } = *self;
        let Theme { fg, hi, .. } = theme;
        let Rect { x, y, h, .. } = rect;
        let h = h as usize;
        for index in 0..h {
            let scroll_offset = (offset * h) / length;
            let scroll_index  = (index  * h) / length;
            term.queue(SetForegroundColor(if scroll_offset == scroll_index { hi } else { fg }))?
                .queue(MoveTo(x, y + index as u16))?
                .queue(Print("▒"))?;
        }
        Ok(())
    }
}

pub fn handle_scroll (length: usize, index: usize, height: usize, offset: usize) -> usize {
    if index < offset {
        let diff = offset - index;
        usize::max(offset - diff, 0)
    } else if index >= offset + height {
        let diff = index - (offset + height) + 1;
        usize::min(offset + diff, length)
    } else {
        offset
    }
}
