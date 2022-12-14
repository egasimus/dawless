use super::{*, super::{*, layout::*}};

pub struct Scrollbar {
    pub theme:  Theme,
    pub length: usize,
    pub offset: usize
}

impl TUI for Scrollbar {

    fn size (&self) -> Size {
        Size { min_w: Some(1), max_w: Some(1), min_h: Some(3), ..Size::default() }
    }

    fn render (&self, term: &mut dyn Write, space: &Space) -> Result<()> {
        let Self {
            theme: Theme { fg, hi, .. },
            length,
            offset,
            ..
        } = *self;
        let Space(Point(x, y), Point(h, _)) = *space;
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
