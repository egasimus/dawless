use crate::*;

#[derive(Default, Debug)]
pub struct Accordion <T: TUI> {
    pub theme: Theme,
    pub index: usize,
    pub items: Vec<Toggle<Label, T>>,
    pub focused: bool
}

impl <T: TUI> Accordion <T> {
    pub fn add (&mut self, text: &str, item: T) -> &mut Self {
        let label = Label { theme: self.theme, focused: self.items.len() == 0, text: text.into() };
        self.items.push(Toggle::new(label, item));
        self
    }
    pub fn len (&self) -> usize {
        self.items.len()
    }
    pub fn width (&self) -> u16 {
        let mut max_len = 0;
        for item in self.items.iter() {
            let len = item.closed.text.len();
            if len > max_len {
                max_len = len
            }
        }
        max_len as u16
    }
}

impl <T: TUI> TUI for Accordion <T> {
    fn layout (&self) -> Layout {
        let mut items = vec![];
        for item in self.items.iter() {
            items.push(Layout::Item(Sizing::Fixed(Size(self.width(), 1)), item));
        }
        Layout::Column(Sizing::Range(self.min_size(), self.max_size()), items)
    }
    fn min_size (&self) -> Size {
        Size(self.width(), self.len() as u16)
    }
    fn max_size (&self) -> Size {
        Size(self.width(), self.len() as u16)
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        Ok(match_key!(event => [
            KeyCode::Up => {
                self.items[self.index].focus(false);
                self.index = if self.index == 0 {
                    self.items.len() - 1
                } else {
                    self.index - 1
                };
                self.items[self.index].focus(true);
                true
            },
            KeyCode::Down => {
                self.items[self.index].focus(false);
                self.index = if self.index >= self.items.len() - 1 {
                    0
                } else {
                    self.index + 1
                };
                self.items[self.index].focus(true);
                true
            },
            KeyCode::Enter => {
                self.items[self.index].toggle();
                true
            }
        ]))
    }
}
