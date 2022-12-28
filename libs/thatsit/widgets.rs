use crate::*;

/// The empty widget
#[derive(Debug, Default, Copy, Clone)]
pub struct Spacer(pub Size);

/// A global instance of the 0x0 empty widget
pub const BLANK: &'static Spacer = &Spacer(Size::MIN);

/// A global instance of the 1x1 empty widget
pub const SPACE: &'static Spacer = &Spacer(Size(1, 1));

impl TUI for Spacer {
    fn layout <'a> (&'a self, _: Size) -> Result<Thunk<'a>> { Ok(self.0.into()) }
}

/// A debug widget that displays its size and position on a colored background
pub struct DebugBox { pub bg: Color }

impl TUI for DebugBox {
    fn layout <'a> (&'a self, _: Size) -> Result<Thunk<'a>> { Ok(Size(16, 3).into()) }
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

impl TUI for String {
    fn layout <'a> (&'a self, _: Size) -> Result<Thunk<'a>> {
        Ok(Size(self.len() as u16, 1).into())
    }
    fn render (&self, term: &mut dyn Write, Area(Point(x, y), _): Area) -> Result<()> {
        term.queue(MoveTo(x, y))?.queue(Print(&self))?;
        Ok(())
    }
}

/// A line of text
#[derive(Debug, Default)]
pub struct Text(pub String);

impl Text {
    pub fn set (&mut self, text: String) -> &mut Self { self.0 = text; self }
    pub fn fg (self, color: Color) -> Foreground<Text> { Foreground(color, self) }
}

impl TUI for Text {
    fn layout <'a> (&'a self, _: Size) -> Result<Thunk<'a>> {
        Ok(Size(self.0.len() as u16, 1).into())
    }
    fn render (&self, term: &mut dyn Write, Area(Point(x, y), _): Area) -> Result<()> {
        term.queue(MoveTo(x, y))?.queue(Print(&self.0))?;
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Foreground<T: TUI>(Color, T);

impl<T: TUI> TUI for Foreground<T> {
    fn layout <'a> (&'a self, max: Size) -> Result<Thunk<'a>> { Ok(self.1.layout(max)?) }
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        term.queue(SetForegroundColor(self.0))?;
        self.1.render(term, area)
    }
}

/// A background rectangle of a fixed color
#[derive(Debug)]
pub struct Background(pub Color);

impl TUI for Background {
    fn render (&self, term: &mut dyn Write, Area(Point(x, y), Size(w, h)): Area) -> Result<()> {
        let background  = " ".repeat(w as usize);
        term.queue(ResetColor)?.queue(SetBackgroundColor(self.0))?;
        for y in y..y+h { term.queue(MoveTo(x, y))?.queue(Print(&background))?; }
        Ok(())
    }
}

/// A widget that switches between two states
#[derive(Default, Debug)]
pub struct Toggle<T: TUI, U: TUI> {
    /// The widget displayed when `state == false`
    pub closed: T,
    /// The widget displayed when `state == true`
    pub open: U,
    /// Which widget to display
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
    pub fn current (&self) -> &dyn TUI { if self.state { &self.open } else { &self.closed } }
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
    fn layout <'a> (&'a self, max: Size) -> Result<Thunk<'a>> { self.current().layout(max) }
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
    fn layout <'a> (&'a self, max: Size) -> Result<Thunk<'a>> {
        let layout = self.0.layout(max)?;
        if self.0.state { layout.min_size.stretch(self.0.closed.layout(max)?.min_size); }
        Ok(layout)
    }
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
    pub pressed: bool,
    pub action:  Option<Box<dyn FnMut() -> Result<bool>>>
}

impl Debug for Button {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Button[{}; {:?}]", self.text, self.layout(Size::MAX))
    }
}

impl Button {
    pub fn new (
        text:   impl Into<String>,
        action: Option<Box<dyn FnMut() -> Result<bool>>>
    ) -> Self {
        Self { text: text.into(), action, ..Self::default() }
    }
}

impl TUI for Button {
    fn layout <'a> (&'a self, _: Size) -> Result<Thunk<'a>> {
        Ok(Size(self.text.len() as u16 + 6, 3).into())
    }
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
        if self.pressed {
            Inset(0).render(term, area)?;
        } else {
            Outset(0).render(term, area)?;
        }
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
    fn around <'a> (&'a self, mut thunk: Thunk<'a>) -> Thunk<'a> {
        let padding   = Size(self.0, self.0);
        let min_size  = thunk.min_size + padding + padding;
        if self.0 > 0 { thunk = pad(padding, thunk.into()) }
        let items = vec![self.into(), thunk.into()];
        let render_fn = render_stack;
        Thunk { min_size, items, render_fn }
    }
}

impl<'a> TUI for Inset {
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        let Area(Point(x, y), Size(w, h)) = area;
        if w == 0 || h == 0 { return Ok(()) }
        let bg = Color::AnsiValue(235);
        Background(bg).render(term, Area(Point(x, y), Size(w, h)))?;
        let top_edge    = "▇".repeat((w - 1) as usize);
        let bottom_edge = "▁".repeat((w - 1) as usize);
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
            term.queue(MoveTo(x+w-1, y))?.queue(Print(&right_edge))?;
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
    fn around <'a> (&'a self, mut thunk: Thunk<'a>) -> Thunk<'a> {
        let padding = Size(self.0, self.0);
        let min_size = thunk.min_size + padding + padding;
        if self.0 > 0 { thunk = pad(padding, thunk.into()) }
        let items = vec![self.into(), thunk.into()];
        let render_fn = render_stack;
        Thunk { min_size, items, render_fn }
    }
}

impl<'a> TUI for Outset {
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        let Area(Point(x, y), Size(w, h)) = area;
        if w == 0 || h == 0 { return Ok(()) }
        let bg = Color::AnsiValue(235);
        Background(bg).render(term, Area(Point(x, y), Size(w, h)))?;
        let top_edge    = "▇".repeat((w - 1) as usize);
        let bottom_edge = "▁".repeat((w - 1) as usize);
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
            term.queue(MoveTo(x+w-1, y))?.queue(Print(&right_edge))?;
        }
        Ok(())
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Centered;

impl Border for Centered {
    fn around <'a> (&'a self, thunk: Thunk<'a>) -> Thunk<'a> {
        let min_size  = thunk.min_size;
        let items     = vec![thunk.into()];
        let render_fn = render_centered;
        Thunk { min_size, items, render_fn }
    }
}

impl<'a> TUI for Centered {}

pub fn render_centered <'a> (
    items: &Vec<LayoutItem<'a>>, write: &mut dyn Write, area: Area
) -> Result<()> {
    let size = items[0].min_size;
    items[0].render(write, Area(Point(
        (area.1.0.saturating_sub(size.0)) / 2,
        (area.1.1.saturating_sub(size.1)) / 2
    ), size))
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
