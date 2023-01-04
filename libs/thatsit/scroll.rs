use std::cell::Cell;

#[derive(Default, Debug)]
/// The scroll state of a scrollable list
pub struct ScrollState {
    /// The size of the visible area
    pub size:   Cell<usize>,
    /// How far into the list is the first visible item
    pub offset: usize,
    /// How many items are there in total
    pub total:  usize
}

impl ScrollState {
    pub fn to (&mut self, index: usize) {
        let ScrollState { offset, total, .. } = *self;
        let size = self.size.get();
        self.offset = if index < offset {
            let diff = offset - index;
            usize::max(offset - diff, 0)
        } else if index >= offset + size {
            let diff = index - (offset + size) + 1;
            usize::min(offset + diff, total)
        } else {
            offset
        }
    }
}
