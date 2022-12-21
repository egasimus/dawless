use crate::*;

#[derive(Default, Debug)]
pub struct Accordion <'a, T: TUI<'a>> {
    pub theme: Theme,
    pub index: usize,
    pub items: Vec<Toggle<'a, Label, T>>,
    pub focused: bool,
    pub entered: bool
}

impl <'a, T: TUI<'a>> Accordion<'a, T> {
    pub fn add (&mut self, text: &str, item: T) -> &mut Self {
        let label = Label { theme: self.theme, focused: self.items.len() == 0, text: text.into() };
        self.items.push(Toggle::new(label, item));
        self
    }
    pub fn get (&self) -> &Toggle<'a, Label, T> {
        &self.items[self.index]
    }
    pub fn get_mut (&mut self) -> &mut Toggle<'a, Label, T> {
        &mut self.items[self.index]
    }
}

impl<'a, T: TUI<'a>> TUI<'a> for Accordion<'a, T> {
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        let mut items = vec![];
        for item in self.items.iter() {
            //items.push(Layout::Item(Sizing::Min, item));
        }
        Layout::Column(Sizing::Range(self.min_size(), self.max_size()), items)
            .render(term, area)
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
        Ok(if self.entered {
            is_key!(event => KeyCode::Esc => {
                self.items[self.index].set(false);
                self.entered = false;
                true
            }) || self.items[self.index].handle(event)?
        } else {
            match_key!((event) {

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
                    self.items[self.index].set(true);
                    self.entered = true;
                    true
                }

            })
        })
    }
}
