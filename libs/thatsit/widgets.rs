use crate::*;

#[derive(Debug, Default, Copy, Clone)]
/// An empty widget
pub struct Spacer(Size);

/// An empty widget
pub const BLANK: &'static Spacer = &Spacer(Size::MIN);

/// A 1x1 empty widget
pub const SPACE: &'static Spacer = &Spacer(Size(1, 1));

impl TUI for Spacer {
    fn min_size (&self) -> Size { self.0 }
    fn max_size (&self) -> Size { self.0 }
}

/// A line of text
#[derive(Debug, Default)]
pub struct Text(pub String);

impl Text {
    pub fn set (&mut self, text: String) -> &mut Self { self.0 = text; self }
    pub fn fg (self, color: Color) -> Foreground<Text> { Foreground(color, self) }
}

impl TUI for Text {
    fn min_size (&self) -> Size { Size(self.0.len() as u16, 1) }
    fn max_size (&self) -> Size { self.min_size() }
    fn render (&self, term: &mut dyn Write, Area(Point(x, y), _): Area) -> Result<()> {
        term.queue(MoveTo(x, y))?.queue(Print(&self.0))?;
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Foreground<T: TUI>(Color, T);

impl<T: TUI> TUI for Foreground<T> {
    fn layout <'a> (&'a self) -> Thunk<'a> { self.1.layout() }
    fn min_size (&self) -> Size { self.1.min_size() }
    fn max_size (&self) -> Size { self.1.max_size() }
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        term.queue(SetForegroundColor(self.0))?;
        self.1.render(term, area)
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
    pub fn new (text: impl Into<String>) -> Self { Self { text: text.into(), ..Self::default() } }
}

impl TUI for Label {
    fn min_size (&self) -> Size { Size(self.text.len() as u16, 1) }
    fn max_size (&self) -> Size { self.min_size() }
    fn focus (&mut self, focus: bool) -> bool { self.focused = focus; true }
    fn render (&self, term: &mut dyn Write, Area(Point(x, y), _): Area) -> Result<()> {
        let Theme { fg, hi, .. } = self.theme;
        term.queue(SetForegroundColor(if self.focused { hi } else { fg }))?
            .queue(MoveTo(x, y))?
            .queue(Print(&self.text))?;
        Ok(())
    }
}


/// A debug widget
pub struct DebugBox { pub bg: Color }

impl TUI for DebugBox {
    fn min_size (&self) -> Size { Size(16, 3) }
    fn max_size (&self) -> Size { Size::MAX }
    fn render (&self, term: &mut dyn Write, Area(Point(x, y), Size(w, h)): Area) -> Result<()> {
        let background = " ".repeat(w as usize);
        term.queue(SetBackgroundColor(self.bg))?
            .queue(SetForegroundColor(Color::AnsiValue(234)))?;
        for row in y..y+h { term.queue(MoveTo(x, row))?.queue(Print(&background))?; }
        let text = format!("{w}x{h}+{x}+{y}");
        term.queue(MoveTo(x, y))?.queue(Print(&text))?;
        Ok(())
    }
}

/// A background rectangle of a fixed color
#[derive(Debug)]
pub struct Background(pub Color);

impl TUI for Background {
    fn render (&self, term: &mut dyn Write, Area(Point(x, y), Size(w, h)): Area) -> Result<()> {
        let background  = " ".repeat(w as usize);
        term.queue(ResetColor)?.queue(SetBackgroundColor(self.0))?;
        for y in y..y+h {
            term.queue(MoveTo(x, y))?.queue(Print(&background))?;
        }
        Ok(())
    }
}

/// A widget that switches between two states
#[derive(Default, Debug)]
pub struct Toggle<T: TUI, U: TUI> {
    pub closed: T,
    pub open:   U,
    state: bool,
}

impl<T: TUI, U: TUI> Toggle<T, U> {
    pub fn new (closed: T, open: U) -> Self { Self { state: false, closed, open } }
    pub fn toggle (&mut self) { self.state = !self.state }
    pub fn get (&mut self) -> bool { self.state }
    pub fn set (&mut self, value: bool) { self.state = value }
    pub fn closed (&self) -> &T { &self.closed }
    pub fn closed_mut (&mut self) -> &mut T { &mut self.closed }
    pub fn open (&mut self) -> &U { &self.open }
    pub fn open_mut (&mut self) -> &mut U { &mut self.open }
    pub fn current (&self) -> &dyn TUI {
        if self.state { &self.open } else { &self.closed }
    }
    pub fn current_mut (&mut self) -> &mut dyn TUI {
        if self.state { &mut self.open } else { &mut self.closed }
    }
}

impl<T: TUI, U: TUI> From<Toggle<T, U>> for bool {
    fn from (it: Toggle<T, U>) -> Self { it.state }
}

impl<'a, T: TUI, U: TUI> From<&'a Toggle<T, U>> for bool {
    fn from (it: &'a Toggle<T, U>) -> Self { it.state }
}

impl<T: TUI, U: TUI> TUI for Toggle<T, U> {
    fn min_size (&self) -> Size { self.current().min_size() }
    fn max_size (&self) -> Size { self.current().max_size() }
    fn focus (&mut self, focus: bool) -> bool { self.current_mut().focus(focus) }
    fn focused (&self) -> bool { self.current().focused() }
    fn layout <'a> (&'a self) -> Thunk<'a> { self.current().layout() }
    fn render (&self, term: &mut dyn Write, rect: Area) -> Result<()> {
        self.current().render(term, rect)
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        self.current_mut().handle(event)
    }
}

/// A button that can turn into another widget
#[derive(Debug, Default)]
pub struct Collapsible(pub Toggle<Button, Box<dyn TUI>>);

impl Collapsible {
    pub fn expand (&mut self) { self.0.set(true) }
    pub fn collapse (&mut self) { self.0.set(false) }
}

impl TUI for Collapsible {
    fn min_size (&self) -> Size {
        if (&self.0).into() {
            self.0.closed.min_size().stretch(self.0.open.min_size())
        } else {
            self.0.closed.min_size()
        }
    }
    fn max_size (&self) -> Size { self.0.max_size() }
    fn focus (&mut self, focus: bool) -> bool { self.0.focus(focus) }
    fn focused (&self) -> bool { self.0.focused() }
    fn layout <'a> (&'a self) -> Thunk<'a> { self.0.layout() }
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        self.0.render(term, area)
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        Ok(self.0.handle(event)? || match_key!((event) {
            KeyCode::Enter => { self.0.toggle(); true }
        }))
    }
}

#[derive(Default)]
pub struct Button {
    pub theme:   Theme,
    pub focused: bool,
    pub text:    String,
    pub action:  Option<Box<dyn FnMut() -> Result<bool>>>
}

impl std::fmt::Debug for Button {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[Button ({}-{}): {}]", self.min_size(), self.max_size(), self.text)
    }
}

impl Button {
    pub fn new (text: impl Into<String>, action: Option<Box<dyn FnMut() -> Result<bool>>>) -> Self {
        Self { text: text.into(), action, ..Self::default() }
    }
}

impl TUI for Button {
    fn min_size (&self) -> Size { Size(self.text.len() as u16 + 6, 3) }
    fn max_size (&self) -> Size { self.min_size() }
    fn focus (&mut self, focus: bool) -> bool { self.focused = focus; true }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        Ok(if_key!(event => Enter => {
            if let Some(action) = &mut self.action {
                (action)()?
            } else {
                false
            }
        }))
    }
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        let Theme { fg, hi, .. } = self.theme;
        Outset(0).render(term, area)?;
        let Area(Point(x, y), _) = area;
        term.queue(ResetColor)?
            //.queue(SetBackgroundColor(if self.focused { Color::AnsiValue(240) } else { Color::AnsiValue(238) }))?
            .queue(SetForegroundColor(if self.focused { hi } else { fg }))?
            .queue(MoveTo(x+3, y+1))?.queue(Print(&self.text))?;
        Ok(())
    }
}

/// An inset border
#[derive(Default, Debug, Copy, Clone)]
pub struct Inset(
    /// The amount of padding between the border and the content
    pub Unit
);

pub trait Border {
    fn around <'a> (&'a self, thunk: Thunk<'a>) -> Thunk<'a>;
}

impl Border for Inset {
    fn around <'a> (&'a self, thunk: Thunk<'a>) -> Thunk<'a> {
        let padding   = Size(self.0*2, self.0*2);
        let min_size  = thunk.min_size + padding;
        let items     = vec![self.into(), pad(padding, thunk.into()).into()];
        let render_fn = render_stack;
        Thunk { min_size, items, render_fn }
    }
}

impl<'a> TUI for Inset {
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        let Area(Point(x, y), Size(w, h)) = area;
        let bg = Color::AnsiValue(235);
        Background(bg).render(term, Area(Point(x, y), Size(w, h-1)))?;
        let top_edge    = "▇".repeat((w) as usize);
        let bottom_edge = "▁".repeat((w) as usize);
        let left_edge   = "▊";
        let right_edge  = "▎";
        term.queue(ResetColor)?
            .queue(SetBackgroundColor(Color::AnsiValue(16)))?
            .queue(SetForegroundColor(bg))?
            .queue(MoveTo(x, y))?
            .queue(Print(&top_edge))?;
        for y in y..y+h {
            term.queue(MoveTo(x, y))?.queue(Print(&left_edge))?;
        }
        term.queue(SetBackgroundColor(bg))?
            .queue(SetForegroundColor(Color::AnsiValue(240)))?
            .queue(MoveTo(x+1, y+h-1))?.queue(Print(&bottom_edge))?;
        for y in y..y+h {
            term.queue(MoveTo(x+w, y))?.queue(Print(&right_edge))?;
        }
        Ok(())
    }
}

/// An outset border
#[derive(Default, Debug, Copy, Clone)]
pub struct Outset(
    /// The amount of padding between the border and the content
    pub Unit
);

impl Border for Outset {
    fn around <'a> (&'a self, thunk: Thunk<'a>) -> Thunk<'a> {
        let padding   = Size(self.0*2, self.0*2);
        let min_size  = thunk.min_size + padding;
        let items     = vec![self.into(), pad(padding, thunk.into()).into()];
        let render_fn = render_stack;
        Thunk { min_size, items, render_fn }
    }
}

impl<'a> TUI for Outset {
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        let Area(Point(x, y), Size(w, h)) = area;
        let bg = Color::AnsiValue(235);
        Background(bg).render(term, Area(Point(x, y), Size(w, h-1)))?;
        let top_edge    = "▇".repeat(w as usize);
        let bottom_edge = "▁".repeat(w as usize);
        let right_edge  = "▎";
        let left_edge   = "▊";
        term.queue(ResetColor)?
            .queue(SetBackgroundColor(Color::AnsiValue(240)))?
            .queue(SetForegroundColor(bg))?
            .queue(MoveTo(x, y))?
            .queue(Print(&top_edge))?;
        for y in y..y+h {
            term.queue(MoveTo(x, y))?.queue(Print(&left_edge))?;
        }
        term.queue(SetBackgroundColor(bg))?
            .queue(SetForegroundColor(Color::AnsiValue(16)))?
            .queue(MoveTo(x+1, y+h-1))?.queue(Print(&bottom_edge))?;
        for y in y..y+h {
            term.queue(MoveTo(x+w, y))?.queue(Print(&right_edge))?;
        }
        Ok(())
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
