use crate::*;

#[derive(Debug)]
pub struct List <T> {
    pub theme: Theme,
    pub index: usize,
    pub items: Vec<(Label, T)>,
    pub scrollbar: Scrollbar,
    pub focused: bool,
}

impl<T> Default for List<T> {
    fn default () -> Self {
        Self::empty()
    }
}

impl <T> List <T> {
    pub fn empty () -> Self {
        List {
            theme: Theme::default(),
            index: 0,
            items: vec![],
            focused: false,
            scrollbar: Scrollbar { length: 1, ..Scrollbar::default() }
        }
    }
    pub fn add (&mut self, text: &str, value: T) -> &mut Self {
        let label = Label { theme: self.theme, focused: self.items.len() == 0, text: text.into() };
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

impl<T: Sync> TUI for List<T> {
    fn layout <'a> (&'a self) -> Thunk<'a> {
        col(|add| {
            for (label, _) in self.items.iter() {
                add(label);
            }
        })
    }
    //fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        //let mut items = vec![];
        //for (label, _) in self.items.iter() {
            //items.push(Layout::Item(Sizing::Fixed(Size(self.width(), 1)), label));
        //}
        //Layout::Column(
            //Sizing::Scroll(&self.scrollbar, &Sizing::Range(self.min_size(), self.max_size())),
            //items
        //).render(term, area)
    //}
    fn min_size (&self) -> Size {
        Size(self.width(), u16::max(1, self.len() as u16))
    }
    fn max_size (&self) -> Size {
        Size(self.width(), u16::max(1, self.len() as u16))
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        Ok(match_key!((event) {

            KeyCode::Up => {
                self.items[self.index].0.focus(false);
                self.index = if self.index == 0 {
                    self.items.len() - 1
                } else {
                    self.index - 1
                };
                self.items[self.index].0.focus(true);
                true
            },

            KeyCode::Down => {
                self.items[self.index].0.focus(false);
                self.index = if self.index >= self.items.len() - 1 {
                    0
                } else {
                    self.index + 1
                };
                self.items[self.index].0.focus(true);
                true
            }

        }))
    }
}

#[macro_export] macro_rules! handle_menu_focus {
    ($event:expr, $parent:expr, $child:expr, $focused:expr) => {
        {
            use ::thatsit::crossterm::event::{Event, KeyEvent, KeyCode};
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
}
