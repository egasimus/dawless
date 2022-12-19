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
}

impl <T: TUI> TUI for Accordion <T> {
    fn layout (&self) -> Layout {
        let mut items = vec![];
        for item in self.items.iter() {
            items.push(Layout::Item(Sizing::Min, item));
        }
        Layout::Column(Sizing::Range(self.min_size(), self.max_size()), items)
    }
    fn min_size (&self) -> Size {
        let mut size = Size::MIN;
        for item in self.items.iter() {
            size = size.expand_column(item.min_size())
        }
        size
    }
    fn max_size (&self) -> Size {
        let mut size = Size::MIN;
        for item in self.items.iter() {
            size = size.expand_column(item.max_size())
        }
        size
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
