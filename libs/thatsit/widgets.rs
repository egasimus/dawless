use crate::*;

#[derive(Debug, Default)]
/// An empty widget
pub struct Blank;

/// An instance of the empty widget
pub const BLANK: &'static Blank = &Blank;

impl TUI for Blank {}

/// A debug widget
pub struct DebugBox { pub bg: Color }

impl TUI for DebugBox {
    fn min_size (&self) -> Size { Size(16, 3) }
    fn max_size (&self) -> Size { Size::MAX }
    fn render (&self, term: &mut dyn Write, Area(Point(x, y), Size(w, h)): Area) -> Result<()> {
        let min = self.min_size();
        let max = self.max_size();
        let background = " ".repeat(w as usize);
        term.queue(SetBackgroundColor(self.bg))?
            .queue(SetForegroundColor(Color::AnsiValue(234)))?;
        for row in y..y+h {
            term.queue(MoveTo(x, row))?.queue(Print(&background))?;
        }
        let text = format!("{w}x{h}+{x}+{y}");
        let pad = w.saturating_sub(text.len() as u16) / 2;
        //term.queue(MoveTo(x+pad, y+1))?
        term.queue(MoveTo(x, y))?
            .queue(Print(&text))?;
        Ok(())
    }
}

/// A line of text
#[derive(Default, Debug)]
pub struct Label {
    pub theme:   Theme,
    pub focused: bool,
    pub text:    String
}

impl Label {
    pub fn new (text: impl Into<String>) -> Self {
        Self { text: text.into(), ..Self::default() }
    }
}

impl TUI for Label {
    fn min_size (&self) -> Size {
        Size(self.text.len() as u16, 1)
    }
    fn max_size (&self) -> Size {
        self.min_size()
    }
    fn focus (&mut self, focus: bool) -> bool {
        self.focused = focus;
        true
    }
    fn render (&self, term: &mut dyn Write, Area(Point(x, y), _): Area) -> Result<()> {
        let Theme { fg, hi, .. } = self.theme;
        term.queue(SetForegroundColor(if self.focused { hi } else { fg }))?
            .queue(MoveTo(x, y))?
            .queue(Print(&self.text))?;
        Ok(())
    }
}

/// An inset border
#[derive(Default, Debug)]
pub struct Inset(
    /// The amount of padding between the border and the content
    pub Unit
);

impl Inset {
    pub fn around <'a> (&'a self, thunk: Thunk<'a>) -> Thunk<'a> {
        let padding = self.0;
        let padding_1 = Size(padding, padding * 2);
        let padding_2 = Size(padding * 2, padding);
        Thunk {
            min_size: thunk.min_size + padding_1,
            items: vec![self.into(), pad(padding_2, thunk.into()).into()],
            render_fn: render_stack,
        }
    }
}

impl<'a> TUI for Inset {
    fn render (&self, term: &mut dyn Write, Area(Point(x, y), Size(w, h)): Area) -> Result<()> {
        let top_edge    = "▇".repeat((w) as usize);
        let bottom_edge = "▁".repeat((w) as usize);
        let left_edge   = "▊";
        let right_edge  = "▎";
        let background  = " ".repeat((w-1) as usize);
        let bg = &Color::AnsiValue(235);
        term.queue(ResetColor)?
            .queue(SetBackgroundColor(Color::AnsiValue(16)))?
            .queue(SetForegroundColor(*bg))?
            .queue(MoveTo(x, y))?
            .queue(Print(&top_edge))?;
        for y in y..y+h {
            term.queue(MoveTo(x, y))?
                .queue(Print(&left_edge))?;
        }
        term.queue(SetBackgroundColor(*bg))?
            .queue(SetForegroundColor(Color::AnsiValue(240)))?
            .queue(MoveTo(x+1, y+h-1))?.queue(Print(&bottom_edge))?;
        for y in y..y+h {
            term.queue(MoveTo(x+w, y))?
                .queue(Print(&right_edge))?;
        }
        for y in y+1..y+h-1 {
            term.queue(MoveTo(x+1, y))?
                .queue(Print(&background))?;
        }
        Ok(())
    }
}

/// A list of sequentially selectable items
#[derive(Debug)]
pub struct Focus<T: TUI> {
    /// The currently focused item
    index: usize,
    /// The key to switch to the next item
    next: KeyCode,
    /// The key to switch to the previous item
    prev: KeyCode,
    /// The list of items
    pub items: Vec<T>,
}

impl<T: TUI> Default for Focus<T> {
    fn default () -> Self {
        Self { items: vec![], index: 0, next: KeyCode::Down, prev: KeyCode::Up }
    }
}

impl<T: TUI> Focus<T> {
    pub fn vertical (items: Vec<T>) -> Focus<T> {
        Self { items, index: 0, next: KeyCode::Down, prev: KeyCode::Up }
    }
    pub fn horizontal (items: Vec<T>) -> Focus<T> {
        Self { items, index: 0, next: KeyCode::Right, prev: KeyCode::Left }
    }
    pub fn focus (&mut self, index: usize) -> bool {
        if self.items.get(self.index).is_some() {
            self.index = index;
            self.items[self.index].focus(true);
            true
        } else {
            false
        }
    }
    pub fn get (&self) -> &T {
        &self.items[self.index]
    }
    pub fn get_mut (&mut self) -> &mut T {
        &mut self.items[self.index]
    }
    pub fn unfocus (&mut self) {
        if let Some(item) = self.items.get_mut(self.index) { item.focus(false); }
    }
    pub fn next (&mut self) {
        self.unfocus();
        self.index = if self.index >= self.items.len() - 1 { 0 } else { self.index + 1 };
        self.focus(self.index);
    }
    pub fn prev (&mut self) {
        self.unfocus();
        self.index = if self.index == 0 { self.items.len() - 1 } else { self.index - 1 };
        self.focus(self.index);
    }
    pub fn handle (&mut self, event: &Event) -> Result<bool> {
        Ok(match_key!((event) {
            self.next => { self.next(); true },
            self.prev => { self.prev(); true }
        }))
    }
}

#[derive(Debug, Default)]
pub struct FocusColumn<T: TUI>(pub Focus<T>);

impl<T: TUI> FocusColumn<T> {
    pub fn new (items: Vec<T>) -> Self { Self(Focus::vertical(items)) }
}

impl<T: TUI> TUI for FocusColumn<T> {
    fn layout <'b> (&'b self) -> Thunk<'b> {
        col(|add|{ for item in self.0.items.iter() { add(&*item); } })
    }
    fn min_size (&self) -> Size {
        let mut size = Size::MIN;
        for item in self.0.items.iter() { size = size.expand_column(item.min_size()) }
        size
    }
    fn max_size (&self) -> Size {
        let mut size = Size::MIN;
        for item in self.0.items.iter() { size = size.expand_column(item.max_size()) }
        size
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        Ok(self.0.handle(event)? || self.0.get_mut().handle(event)? || false)
    }
}

#[derive(Debug, Default)]
pub struct FocusRow<T: TUI>(pub Focus<T>);

impl<T: TUI> FocusRow<T> {
    pub fn new (items: Vec<T>) -> Self { Self(Focus::horizontal(items)) }
}

impl<T: TUI> TUI for FocusRow<T> {
    fn layout <'b> (&'b self) -> Thunk<'b> {
        row(|add|{ for item in self.0.items.iter() { add(&*item); } })
    }
    fn min_size (&self) -> Size {
        let mut size = Size::MIN;
        for item in self.0.items.iter() { size = size.expand_row(item.min_size()) }
        size
    }
    fn max_size (&self) -> Size {
        let mut size = Size::MIN;
        for item in self.0.items.iter() { size = size.expand_row(item.max_size()) }
        size
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        Ok(self.0.handle(event)? || self.0.get_mut().handle(event)? || false)
    }
}

#[derive(Default, Debug)]
pub struct Toggle<T: TUI, U: TUI> {
    pub closed: T,
    pub open:   U,
    state: bool,
}

impl<T: TUI, U: TUI> Toggle<T, U> {
    pub fn new (closed: T, open: U) -> Self {
        Self { state: false, closed, open }
    }
    pub fn toggle (&mut self) {
        self.state = !self.state
    }
    pub fn get (&mut self) -> bool {
        self.state
    }
    pub fn set (&mut self, value: bool) {
        self.state = value
    }
    pub fn closed (&self) -> &T {
        &self.closed
    }
    pub fn closed_mut (&mut self) -> &mut T {
        &mut self.closed
    }
    pub fn open (&mut self) -> &U {
        &self.open
    }
    pub fn open_mut (&mut self) -> &mut U {
        &mut self.open
    }
    pub fn current (&self) -> &dyn TUI {
        if self.state { &self.open } else { &self.closed }
    }
    pub fn current_mut (&mut self) -> &mut dyn TUI {
        if self.state { &mut self.open } else { &mut self.closed }
    }
}

impl<T: TUI, U: TUI> Into<bool> for Toggle<T, U> {
    fn into (self: Toggle<T, U>) -> bool {
        self.state
    }
}

impl<'a, T: TUI, U: TUI> Into<bool> for &'a Toggle<T, U> {
    fn into (self: &'a Toggle<T, U>) -> bool {
        self.state
    }
}

impl<T: TUI, U: TUI> TUI for Toggle<T, U> {
    fn min_size (&self) -> Size {
        self.current().min_size()
    }
    fn max_size (&self) -> Size {
        self.current().max_size()
    }
    fn focus (&mut self, focus: bool) -> bool {
        self.current_mut().focus(focus)
    }
    fn focused (&self) -> bool {
        self.current().focused()
    }
    fn layout <'a> (&'a self) -> Thunk<'a> {
        self.current().layout()
    }
    fn render (&self, term: &mut dyn Write, rect: Area) -> Result<()> {
        self.current().render(term, rect)
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        self.current_mut().handle(event)
    }
}

#[derive(Debug, Default)]
pub struct Collapsible(pub Toggle<Button, Box<dyn TUI>>);

impl TUI for Collapsible {
    fn min_size (&self) -> Size {
        if (&self.0).into() {
            self.0.closed.min_size().stretch(self.0.open.min_size())
        } else {
            self.0.closed.min_size()
        }
    }
    fn max_size (&self) -> Size {
        self.0.max_size()
    }
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        self.0.render(term, area)
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        Ok(match_key!((event) {
            KeyCode::Enter => { self.0.toggle(); true }
        }))
    }
    fn focus (&mut self, focus: bool) -> bool {
        self.0.focus(focus)
    }
    fn focused (&self) -> bool {
        self.0.focused()
    }
    fn layout <'a> (&'a self) -> Thunk<'a> {
        self.0.layout()
    }
}

#[derive(Default)]
pub struct Button {
    pub theme:   Theme,
    pub focused: bool,
    pub text:    String,
    pub action:  Option<Box<dyn FnMut() -> ()>>
}

impl std::fmt::Debug for Button {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[Button ({}-{}): {}]", self.min_size(), self.max_size(), self.text)
    }
}

impl Button {
    pub fn new (text: impl Into<String>, action: Option<Box<dyn FnMut() -> ()>>) -> Self {
        Self { text: text.into(), action, ..Self::default() }
    }
}

impl TUI for Button {
    fn min_size (&self) -> Size {
        Size(self.text.len() as u16 + 6, 3)
    }
    fn max_size (&self) -> Size {
        self.min_size()
    }
    fn focus (&mut self, focus: bool) -> bool {
        self.focused = focus;
        true
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        Ok(if_key!(event => KeyCode::Enter => {
            if let Some(action) = &mut self.action {
                (action)();
            }
            true
        }))
    }
    fn render (&self, term: &mut dyn Write, Area(Point(x, y), _): Area) -> Result<()> {
        let Theme { fg, hi, .. } = self.theme;
        let w           = self.text.len() as u16 + 4;
        let top_edge    = "▇".repeat(w as usize);
        let bottom_edge = "▁".repeat(w as usize);
        let right_edge  = "▎";
        let left_edge   = "▊";
        let background  = " ".repeat(w as usize);
        let bg          = Color::AnsiValue(235);
        term.queue(ResetColor)?
            .queue(SetBackgroundColor(if self.focused { Color::AnsiValue(240) } else { Color::AnsiValue(238) }))?
            .queue(SetForegroundColor(bg))?
            .queue(MoveTo(x,     y+0))?.queue(Print(&left_edge))?
            .queue(MoveTo(x,     y+1))?.queue(Print(&left_edge))?
            .queue(MoveTo(x,     y+2))?.queue(Print(&left_edge))?
            .queue(SetForegroundColor(if self.focused { Color::AnsiValue(236) } else { bg }))?
            .queue(MoveTo(x+1,   y+0))?.queue(Print(&top_edge))?
            .queue(SetBackgroundColor(if self.focused { Color::AnsiValue(236) } else { bg }))?
            .queue(MoveTo(x+1,   y+1))?.queue(Print(&background))?
            .queue(SetForegroundColor(if self.focused { hi } else { fg }))?
            .queue(MoveTo(x+3,   y+1))?.queue(Print(&self.text))?
            .queue(SetForegroundColor(self.theme.bg))?
            .queue(MoveTo(x+1,   y+2))?.queue(Print(&bottom_edge))?
            .queue(SetBackgroundColor(bg))?
            .queue(MoveTo(x+w+1, y+0))?.queue(Print(&right_edge))?
            .queue(MoveTo(x+w+1, y+1))?.queue(Print(&right_edge))?
            .queue(MoveTo(x+w+1, y+2))?.queue(Print(&right_edge))?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct List <T> {
    pub theme: Theme,
    pub index: usize,
    pub items: Vec<(Label, T)>,
    pub scrollbar: Scrollbar,
    pub focused: bool,
}

impl<T> Default for List<T> {
    fn default () -> Self { Self::empty() }
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

#[derive(Default, Debug)]
pub struct Accordion <T: TUI> {
    pub theme: Theme,
    pub index: usize,
    pub items: Vec<Toggle<Label, T>>,
    pub focused: bool,
    pub entered: bool
}

impl <T: TUI> Accordion<T> {
    pub fn add (&mut self, text: &str, item: T) -> &mut Self {
        let label = Label { theme: self.theme, focused: self.items.len() == 0, text: text.into() };
        self.items.push(Toggle::new(label, item));
        self
    }
    pub fn get (&self) -> &Toggle<Label, T> {
        &self.items[self.index]
    }
    pub fn get_mut (&mut self) -> &mut Toggle<Label, T> {
        &mut self.items[self.index]
    }
}

impl<T: TUI> TUI for Accordion<T> {
    //fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        //let mut items = vec![];
        //for item in self.items.iter() {
            ////items.push(Layout::Item(Sizing::Min, item));
        //}
        //Layout::Column(Sizing::Range(self.min_size(), self.max_size()), items)
            //.render(term, area)
    //}
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
            if_key!(event => KeyCode::Esc => {
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

#[derive(Debug, Default)]
pub struct Scrollbar {
    pub theme:  Theme,
    pub length: usize,
    pub offset: usize
}

impl TUI for Scrollbar {
    fn min_size (&self) -> Size {
        Size(1, 3)
    }
    fn max_size (&self) -> Size {
        Size(1, Unit::MAX)
    }
    fn render (&self, term: &mut dyn Write, Area(Point(x, y), Size(_, h)): Area) -> Result<()> {
        //let layout = Layout::Item(Sizing::Fixed(Size(1, 1)), &Blank {});
        //let Self { theme: Theme { fg, hi, .. }, length, offset } = *self;
        //let h = h as usize;
        //for index in 0..h {
            //let scroll_offset = (offset * h) / length;
            //let scroll_index  = (index  * h) / length;
            //term.queue(SetForegroundColor(if scroll_offset == scroll_index { hi } else { fg }))?
                //.queue(MoveTo(x, y + index as u16))?
                //.queue(Print("▒"))?;
        //}
        Ok(())
    }
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

#[cfg(test)]
mod test {
    use std::str::from_utf8;
    use crate::{*, layout::*};

    #[test]
    fn test_frame () {
        let frame = Inset;
        assert_rendered!(frame == "\u{1b}[0m\u{1b}[38;5;232m\u{1b}[6;6H▄▄▄▄▄▄▄▄▄▄\u{1b}[15;6H▀▀▀▀▀▀▀▀▀▀\u{1b}[0m\u{1b}[48;5;232m\u{1b}[7;6H          \u{1b}[8;6H          \u{1b}[9;6H          \u{1b}[10;6H          \u{1b}[11;6H          \u{1b}[12;6H          \u{1b}[13;6H          \u{1b}[14;6H          \u{1b}[48;5;232m\u{1b}[38;5;15m\u{1b}[6;6H \u{1b}[6;7H\u{1b}[1m\u{1b}[4m\u{1b}[0m\u{1b}[6;7H\u{1b}[48;5;232m\u{1b}[38;5;15m ");
    }
}
