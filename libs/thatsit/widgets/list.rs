use crate::*;

#[derive(Default, Debug)]
pub struct List <T> {
    pub theme: Theme,
    pub index: usize,
    pub items: Vec<(Label, T)>
}

impl <T> List <T> {
    pub fn add (&mut self, text: &str, value: T) -> &mut Self {
        let label = Label { theme: self.theme, focused: false, text: text.into() };
        self.items.push((label, value));
        self
    }
    pub fn replace (&mut self, items: Vec<(String, T)>) -> &mut Self {
        self.items = vec![];
        for (text, value) in items {
            self.add(text.as_str(), value);
        }
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
            let len = label.text.len();
            if len > max_len {
                max_len = len
            }
        }
        max_len as u16
    }
}

impl <T: Sync> TUI for List <T> {
    fn layout (&self) -> Layout {
        let mut items = vec![];
        for (label, _) in self.items.iter() {
            items.push(Layout::Item(Sizing::Fixed(Size(self.width(), 1)), label));
        }
        Layout::Column(Sizing::Range(self.min_size(), self.max_size()), items)
    }
    fn min_size (&self) -> Size {
        Size(self.width(), 3)
    }
    fn max_size (&self) -> Size {
        Size(self.width(), self.len() as u16)
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
