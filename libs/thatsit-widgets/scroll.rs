/// TODO

use crate::*;

#[derive(Debug, Default)]
pub struct ScrollVertical {
    pub length: usize,
    pub offset: usize
}

impl Widget for ScrollVertical {
    impl_render!(self, _out, _area => {
        //let Area(x, y, _, h) = area;
        //let layout = Layout::Item(Sizing::Fixed(Size(1, 1)), &Blank {});
        //let Self { theme: Theme { fg, hi, .. }, length, offset } = *self;
        //let h = h as usize;
        //for index in 0..h {
            //let scroll_offset = (offset * h) / length;
            //let scroll_index  = (index  * h) / length;
            //term.queue(SetForegroundColor(if scroll_offset == scroll_index { hi } else { fg }))?
                //.queue(MoveTo(x, y + index as u16))?
                //.queue(Print("▒"))?;
        //}
        Ok((0, 0))
    });
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

