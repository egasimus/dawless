use super::*;

#[derive(Default, Debug)]
pub struct List <T> {
    pub space: Space,
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
    pub fn width (&self) -> u16 {
        let mut max_len = 0;
        for (label, _) in self.items.iter() {
            let len = label.len();
            if len > max_len {
                max_len = len
            }
        }
        max_len as u16
    }
}

impl <T: Sync> TUI for List <T> {

    fn size (&self) -> Size {
        let len = self.len() as u16;
        Size {
            max_w: Some(self.width()),
            min_h: Some(len),
            max_h: Some(len),
            ..Size::default()
        }
    }

    fn layout (&mut self, space: &Space) -> Result<Space> {
        let Space(Point(x, y), Point(w, _)) = *space;
        let mut max_len = 0;
        for (label, _) in self.items.iter() {
            let len = label.len();
            if len > max_len {
                max_len = len
            }
        }
        self.space = Space::new(x, y, u16::min(w, (max_len + 3) as u16), self.items.len() as u16);
        Ok(self.space)
    }

    fn render (&self, term: &mut dyn Write) -> Result<()> {
        let Self { theme, space: Space(Point(x, y), Point(w, _)), .. } = *self;
        for (index, item) in self.items.iter().enumerate() {
            let text = format!(" {:<0width$} ▶ ", item.0, width = (w - 3) as usize);
            let row  = y + index as u16;
            Label { theme, col: x, row, focused: index == self.index, text }.render(term)?;
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
