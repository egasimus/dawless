use crate::*;

/// The empty widget
#[derive(Debug, Default, Copy, Clone)]
pub struct Spacer(pub Size);

/// A global instance of the 0x0 empty widget
pub const BLANK: &'static Spacer = &Spacer(Size::MIN);

/// A global instance of the 1x1 empty widget
pub const SPACE: &'static Spacer = &Spacer(Size(1, 1));

impl TUI for Spacer {
    fn layout <'a> (&'a self, max: Size) -> Result<Layout<'a>> {
        self.0.limit(max, |term, area|{Ok(())})
    }
}

/// A debug widget that displays its size and position on a colored background
pub struct DebugBox { pub bg: Color }

impl Default for DebugBox {
    fn default () -> Self {
        Self { bg: Color::AnsiValue(123) }
    }
}

impl TUI for DebugBox {
    fn layout <'a> (&'a self, max: Size) -> Result<Layout<'a>> {
        Size(16, 3).limit(max, |term, area|{
            let Area(Point(x, y), Size(w, h)) = area;
            let background = " ".repeat(w as usize);
            term.queue(SetBackgroundColor(self.bg))?
                .queue(SetForegroundColor(Color::AnsiValue(234)))?;
            for row in y..y+h { term.queue(MoveTo(x, row))?.queue(Print(&background))?; }
            let text = format!("{w}x{h}+{x}+{y}");
            term.queue(MoveTo(x, y))?.queue(Print(&text))?;
            Ok(())
        })
    }
}

impl TUI for String {
    fn layout <'a> (&'a self, max: Size) -> Result<Layout<'a>> {
        Size(self.len() as u16, 1).limit(max, |term, area| {
            let Area(Point(x, y), Size(w, h)) = area;
            term.queue(MoveTo(x, y))?.queue(Print(&self))?;
            Ok(())
        })
    }
}

impl TUI for &str {
    fn layout <'a> (&'a self, max: Size) -> Result<Layout<'a>> {
        Size(self.len() as u16, 1).limit(max, |term, area| {
            let Area(Point(x, y), Size(w, h)) = area;
            term.queue(MoveTo(x, y))?.queue(Print(&self))?;
            Ok(())
        })
    }
}

/// A line of text with optional foreground and background colors
#[derive(Copy, Clone, Debug, Default)]
pub struct Text<'a>(
    /// The text
    &'a str,
    /// The foreground
    Option<Color>,
    /// The background
    Option<Color>
);

impl<'a> Text<'a> {
    pub fn new (text: &'a str) -> Self { Self(text, None, None) }
    pub fn set (&mut self, text: &'a str) -> Self { self.0 = text; *self }
    pub fn fg (&mut self, color: Option<Color>) -> Self { self.1 = color; *self }
    pub fn bg (&mut self, color: Option<Color>) -> Self { self.2 = color; *self }
}

impl<'a> TUI for Text<'a> {
    fn layout <'l> (&'l self, max: Size) -> Result<Layout<'l>> {
        Size(self.0.len() as u16, 1).limit(max, |term, area| {
            let Area(Point(x, y), Size(w, h)) = area;
            term.queue(MoveTo(x, y))?.queue(Print(&self.0))?;
            Ok(())
        })
    }
}

/// A background rectangle of a fixed color
#[derive(Debug)]
pub struct Filled(pub Color);

impl TUI for Filled {
    fn layout <'l> (&'l self, max: Size) -> Result<Layout<'l>> {
        Ok(Layout(&|term, area| {
            let Area(Point(x, y), Size(w, h)) = area;
            let background  = " ".repeat(w as usize);
            term.queue(ResetColor)?.queue(SetBackgroundColor(self.0))?;
            for y in y..y+h { term.queue(MoveTo(x, y))?.queue(Print(&background))?; }
            Ok(())
        }))
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
    fn handle (&mut self, event: &Event) -> Result<bool> {
        self.current_mut().handle(event)
    }
    fn layout <'a> (&'a self, max: Size) -> Result<Layout<'a>> {
        self.current().layout(max)
    }
}

#[derive(Default)]
pub struct Button {
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
    fn handle (&mut self, event: &Event) -> Result<bool> {
        Ok(if_key!(event => Enter => {
            if let Some(action) = &mut self.action {
                (action)()?
            } else {
                false
            }
        }))
    }
    fn layout <'a> (&'a self, max: Size) -> Result<Layout<'a>> {
        Size(self.text.len() as u16 + 6, 3).limit(max, |term, area| {
            Layers(&|layer: Collect<'a>|{
                if self.pressed {
                    layer(Inset(0));
                } else {
                    layer(Outset(0));
                }
                layer(&|term, area|{
                    let bg = Color::AnsiValue(232);
                    let fg = Color::White;
                    let hi = Color::Yellow;
                    term.queue(ResetColor)?
                        //.queue(SetBackgroundColor(if self.focused { Color::AnsiValue(240) } else { Color::AnsiValue(238) }))?
                        .queue(SetForegroundColor(if self.focused { hi } else { fg }))?
                        .queue(MoveTo(x+3, y+1))?.queue(Print(&self.text))?;
                    Ok(())
                });
            }).layout(max)?.render(term, area)
        })
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
    fn layout <'l> (&'l self, max: Size) -> Result<Layout<'l>> {
        Ok(Layout(&|term, area|{
            let Area(Point(x, y), Size(w, h)) = area;
            if w == 0 || h == 0 { return Ok(()) }
            let bg = Color::AnsiValue(235);
            Filled(bg).render(term, Area(Point(x, y), Size(w, h)))?;
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
        }))
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
    fn layout <'l> (&'l self, max: Size) -> Result<Layout<'l>> {
        Ok(Layout(&|term, area|{
            let Area(Point(x, y), Size(w, h)) = area;
            if w == 0 || h == 0 { return Ok(()) }
            let bg = Color::AnsiValue(235);
            Filled(bg).render(term, Area(Point(x, y), Size(w, h)))?;
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
        }))
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

//#[cfg(test)]
//mod test {
    //use std::str::from_utf8;
    //use crate::{*, layout::*};

    //#[test]
    //fn test_frame () {
        //let frame = Inset;
        //assert_rendered!(frame == "\u{1b}[0m\u{1b}[38;5;232m\u{1b}[6;6H▄▄▄▄▄▄▄▄▄▄\u{1b}[15;6H▀▀▀▀▀▀▀▀▀▀\u{1b}[0m\u{1b}[48;5;232m\u{1b}[7;6H          \u{1b}[8;6H          \u{1b}[9;6H          \u{1b}[10;6H          \u{1b}[11;6H          \u{1b}[12;6H          \u{1b}[13;6H          \u{1b}[14;6H          \u{1b}[48;5;232m\u{1b}[38;5;15m\u{1b}[6;6H \u{1b}[6;7H\u{1b}[1m\u{1b}[4m\u{1b}[0m\u{1b}[6;7H\u{1b}[48;5;232m\u{1b}[38;5;15m ");
    //}
//}
