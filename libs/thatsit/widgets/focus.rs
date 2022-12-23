use crate::*;

#[derive(Debug, Default)]
pub struct FocusColumn<T: TUI> {
    pub theme:   Theme,
    pub index:   usize,
    pub items:   Vec<T>,
    pub focus:   bool,
    pub focused: bool,
}

impl<T: TUI> TUI for FocusColumn<T> {
    fn layout <'a> (&'a self) -> Thunk<'a> {
        col(|add|{
            for item in self.items.iter() {
                add(item);
            }
        })
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
        Ok(
            self.handle_select(event)?    ||
            self.get_mut().handle(event)? ||
            false
        )
    }
}

impl<T: TUI> FocusColumn<T> {
    pub fn add (&mut self, item: T) -> &mut Self {
        self.items.push(item);
        self
    }
    pub fn get (&self) -> &T {
        &self.items[self.index]
    }
    pub fn get_mut (&mut self) -> &mut T {
        &mut self.items[self.index]
    }
    fn handle_select (&mut self, event: &Event) -> Result<bool> {
        Ok(match_key!((event) {
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
            }
        }))
    }
}
