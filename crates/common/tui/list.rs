use super::*;

#[derive(Default)]
pub struct List <T> {
    pub rect:  Rect,
    pub theme: Theme,
    pub index: usize,
    pub items: Vec<(String, T)>
}

impl <T> List <T> {
    pub fn add (&mut self, label: &str, value: T) -> &mut Self {
        self.items.push((label.into(), value));
        self
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

impl <T: Sync> TUI for List <T> {

    fn layout (&mut self, col1: u16, row1: u16, cols: u16, rows: u16) -> Result<()> {
        let mut max_len = 0;
        for (label, _) in self.items.iter() {
            let len = label.len();
            if len > max_len {
                max_len = len
            }
        }
        self.rect = (col1, row1, u16::min(cols, max_len as u16), 0);
        Ok(())
    }

    fn render (&self, term: &mut dyn Write) -> Result<()> {
        let Theme { bg, fg, hi } = self.theme;
        let (col1, row1, cols, ..) = self.rect;
        for (index, item) in self.items.iter().enumerate() {
            let fg = if index == self.index { hi } else { fg };
            term.queue(SetBackgroundColor(bg))?
                .queue(SetForegroundColor(fg))?
                .queue(MoveTo(col1, row1 + (index as u16)))?
                .queue(Print(format!(" {:<0width$} ▶ ", item.0, width = cols as usize)))?;
        }
        Ok(())
    }

    fn handle (&mut self, event: &Event) -> Result<bool> {
        handle_list_select(event, self.items.len(), &mut self.index)
    }

}

pub fn handle_list_select (event: &Event, length: usize, index: &mut usize) -> Result<bool> {
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
