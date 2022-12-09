use super::*;

pub struct Menu <T> {
    pub index: usize,
    pub items: Vec<(String, T)>
}

impl <T> Menu <T> {
    pub fn new (items: Vec<(String, T)>) -> Self {
        Self {
            index: 0,
            items
        }
    }
    pub fn get (&self) -> Option<&T> {
        self.items.get(self.index).map(|x| &x.1)
    }
    pub fn get_mut (&mut self) -> Option<&mut T> {
        self.items.get_mut(self.index).map(|x| &mut x.1)
    }
    pub fn len (&self) -> usize {
        self.items.len()
    }
}

impl <T: Sync> TUI for Menu <T> {
    fn render (&self, term: &mut dyn Write, col1: u16, row1: u16, cols: u16, _rows: u16) -> Result<()> {
        let bg = Color::AnsiValue(232);
        let fg = Color::White;
        let hi = Color::Yellow;
        for (index, item) in self.items.iter().enumerate() {
            term.queue(SetBackgroundColor(bg))?
                .queue(SetForegroundColor(if index == self.index { hi } else { fg }))?
                .queue(MoveTo(col1, row1 + (index as u16)))?
                .queue(Print(format!(" {:<0width$} ▶ ", item.0, width = cols as usize)))?;
        }
        Ok(())
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        handle_menu_selection(event, self.items.len(), &mut self.index)
    }
}

pub fn handle_menu_selection (event: &Event, length: usize, index: &mut usize) -> Result<bool> {
    Ok(match event {
        Event::Key(KeyEvent { code: KeyCode::Up, .. }) => {
            *index = if *index == 0 {
                length - 1
            } else {
                *index - 1
            };
            true
        },
        Event::Key(KeyEvent { code: KeyCode::Down, .. }) => {
            *index = if *index >= length - 1 {
                0
            } else {
                *index + 1
            };
            true
        },
        _ => false
    })
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

#[macro_export] macro_rules! handle_menu_focus {
    ($event:expr, $parent:expr, $child:expr, $focused:expr) => {
        Ok(match $event {
            Event::Key(KeyEvent { code: KeyCode::Left, .. }) => {
                if $focused {
                    false
                } else {
                    if $child.focus(false) {
                        $parent.focus(true);
                    }
                    true
                }
            },
            Event::Key(KeyEvent { code: KeyCode::Right, .. }) => {
                if $child.focus(true) {
                    $parent.focus(false);
                }
                true
            },
            Event::Key(KeyEvent { code: KeyCode::Esc, .. }) => {
                if $focused {
                    false
                } else {
                    if $child.focus(false) {
                        $parent.focus(true);
                    }
                    true
                }
            },
            Event::Key(KeyEvent { code: KeyCode::Enter, .. }) => {
                if $child.focus(true) {
                    $parent.focus(false);
                }
                true
            },
            _ => {
                false
            }
        })
    }
}

pub fn render_scrollbar (
    term: &mut dyn Write, col1: u16, row1: u16,
    length: usize, offset: usize, height: usize,
) -> Result<()> {
    let fg = Color::White;
    let hi = Color::Yellow;
    for index in 0..height {
        let scroll_offset = (offset * height) / length;
        let scroll_index  = (index  * height) / length;
        term.queue(SetForegroundColor(if scroll_offset == scroll_index { hi } else { fg }))?
            .queue(MoveTo(col1, row1 + index as u16))?
            .queue(Print("▒"))?;
    }
    Ok(())
}
