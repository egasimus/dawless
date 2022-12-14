use super::{*, super::{*, layout::*}};

#[derive(Default, Debug)]
pub struct List <T> {
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
        let width = self.width() as u16;
        let len = self.len() as u16;
        Size {
            min_w: Some(width),
            max_w: Some(width),
            min_h: Some(len),
            max_h: Some(len),
            ..Size::default()
        }
    }

    fn render (&self, term: &mut dyn Write, space: &Space) -> Result<()> {
        let Space(Point(x, y), Point(w, _)) = * space;
        for (index, item) in self.items.iter().enumerate() {
            let text = format!(" {:<0width$} ▶ ", item.0, width = (w - 3) as usize);
            let row  = y + index as u16;
            Label { theme: self.theme, focused: index == self.index, text }
                .render(term, space)?;
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
