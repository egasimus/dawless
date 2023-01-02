use crate::*;

/// The empty widget
#[derive(Debug, Default, Copy, Clone)]
pub struct Spacer(pub Size);

/// A global instance of the 0x0 empty widget
pub const BLANK: &'static Spacer = &Spacer(Size::MIN);

/// A global instance of the 1x1 empty widget
pub const SPACE: &'static Spacer = &Spacer(Size(1, 1));

impl TUI for Spacer {
    tui! { layout (self, max) { self.0.constrain(max, Wrapper::one(self)) } }
}

/// A debug widget that displays its size and position on a colored background
pub struct DebugBox { pub bg: Color }

impl Default for DebugBox {
    fn default () -> Self {
        Self { bg: Color::AnsiValue(123) }
    }
}

impl TUI for DebugBox {
    tui! {
        layout (self, max) {
            Size(16, 3).constrain(max, Wrapper::one(self))
        }
        render (self, term, area) {
            let Area(Point(x, y), Size(w, h)) = area;
            let background = " ".repeat(w as usize);
            term.queue(SetBackgroundColor(self.bg))?
                .queue(SetForegroundColor(Color::AnsiValue(234)))?;
            for row in y..y+h { term.queue(MoveTo(x, row))?.queue(Print(&background))?; }
            let text = format!("{w}x{h}+{x}+{y}");
            term.queue(MoveTo(x, y))?.queue(Print(&text))?;
            Ok(())
        }
    }
}

impl TUI for String {
    tui! {
        layout (self, max) {
            Size(self.len() as u16, 1).constrain(max, Wrapper::one(self))
        }
        render (self, term, area) {
            let Area(Point(x, y), Size(w, h)) = area;
            term.queue(MoveTo(x, y))?.queue(Print(&self))?;
            Ok(())
        }
    }
}

impl TUI for &str {
    tui! {
        layout (self, max) {
            Size(self.len() as u16, 1).constrain(max, Wrapper::one(self))
        }
        render (self, term, area) {
            let Area(Point(x, y), Size(w, h)) = area;
            term.queue(MoveTo(x, y))?.queue(Print(&self))?;
            Ok(())
        }
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
    tui! {
        layout (self, max) {
            Size(self.0.len() as u16, 1).constrain(max, Wrapper::Empty)
        }
        render (self, term, area) {
            let Area(Point(x, y), Size(w, h)) = area;
            term.queue(MoveTo(x, y))?.queue(Print(&self.0))?;
            Ok(())
        }
    }
}

/// A background rectangle of a fixed color
#[derive(Debug)]
pub struct Filled(pub Color);

impl TUI for Filled {
    tui! {
        render (self, term, area) {
            let Area(Point(x, y), Size(w, h)) = area;
            let background  = " ".repeat(w as usize);
            term.queue(ResetColor)?.queue(SetBackgroundColor(self.0))?;
            for y in y..y+h { term.queue(MoveTo(x, y))?.queue(Print(&background))?; }
            Ok(())
        }
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
    tui! {
        layout (self, max) {
            self.current().layout(max)
        }
        handle (self, event) {
            self.current_mut().handle(event)
        }
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
        write!(f, "Button[{}; {:?}]", self.text, "")//, self.layout(Size::MAX))
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
    tui! {
        layout (self, max) {
            Size(self.text.len() as u16 + 6, 3).constrain(max, Wrapper::Empty)
        }
        render (self, term, area) {
            let border = (if self.pressed { &Inset(0) as &dyn TUI } else { &Outset(0) as &dyn TUI })
                .render(term, area)?;
            //let bg = Color::AnsiValue(232);
            let fg = Color::White;
            let hi = Color::Yellow;
            let Area(Point(x, y), _) = area;
            term.queue(ResetColor)?
                //.queue(SetBackgroundColor(if self.focused { Color::AnsiValue(240) } else { Color::AnsiValue(238) }))?
                .queue(SetForegroundColor(if self.focused { hi } else { fg }))?
                .queue(MoveTo(x+3, y+1))?.queue(Print(&self.text))?;
            Ok(())
        }
        handle (self, event) {
            Ok(if_key!(event => Enter => {
                if let Some(action) = &mut self.action {
                    (action)()?
                } else {
                    false
                }
            }))
        }
    }
}

#[derive(Clone, Copy)]
pub struct Columns<'l, const N: usize>(pub [&'l dyn TUI; N]);

impl<'l, const N: usize> TUI for Columns<'l, N> {
    tui! {
        render (self, term, area) {
            let mut y = area.0.y();
            let max_y = area.0.y() + area.1.height();
            for item in self.0.iter() {
                let size = Size::MIN;//item.min_size();
                let next_y = y + size.height();
                if next_y > max_y {
                    let msg = format!("need {} more rows", next_y - max_y);
                    return Err(Error::new(ErrorKind::Other, msg))
                }
                item.render(term, Area(Point(area.0.x(), y), size))?;
                y = y + size.height();
            }
            Ok(())
        }
    }
}

pub struct Rows<'l, const N: usize>(pub [&'l dyn TUI; N]);

impl<'l, const N: usize> TUI for Rows<'l, N> {
    tui! {
        render (self, term, area) {
            let mut x = area.0.x();
            for item in self.0.iter() {
                let size = Size::MIN;//item.min_size();
                let area = Area(Point(x, area.0.y()), size);
                item.render(term, area)?;
                x = x + size.width();
            }
            Ok(())
        }
    }
}

pub struct Layers<'a, const N: usize>(pub [&'a dyn TUI; N]);

impl<'a, const N: usize> TUI for Layers<'a, N> {
    tui! {
        render (self, term, area) {
            for item in self.0.iter() { item.render(term, area)?; }
            Ok(())
        }
    }
}

/// An inset border
#[derive(Default, Debug, Copy, Clone)]
pub struct Inset(
    /// The amount of padding between the border and the content
    pub Unit
);

impl Inset {
    fn render (layers: &Self, term: &mut dyn Write, area: Area) -> Result<()> {
        let Area(Point(x, y), Size(w, h)) = area;
        if w == 0 || h == 0 { return Ok(()) }
        let bg = Color::AnsiValue(235);
        let fill = Filled(bg);
        fill.render(term, Area(Point(x, y), Size(w, h)))?;
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

impl<'a> TUI for Inset {
    tui! {
        render (self, term, area) {
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
        }
    }
}

/// An outset border
#[derive(Default, Debug, Copy, Clone)]
pub struct Outset(
    /// The amount of padding between the border and the content
    pub Unit
);

impl<'a> TUI for Outset {
    tui! {
        render (self, term, area) {
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
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Centered;

impl<'a> TUI for Centered {}

//pub trait Border {
    //fn around <'a> (&'a self, thunk: Wrapper<'a>) -> Wrapper<'a>;
//}

//impl Border for Inset {
    //fn around <'a> (&'a self, mut thunk: Wrapper<'a>) -> Wrapper<'a> {
        //let padding   = Size(self.0, self.0);
        //let min_size  = thunk.min_size + padding + padding;
        //if self.0 > 0 { thunk = pad(padding, thunk.into()) }
        //let items = vec![self.into(), thunk.into()];
        //let render_fn = render_stack;
        //Wrapper { min_size, items, render_fn }
    //}
//}

//impl Border for Outset {
    //fn around <'a> (&'a self, mut thunk: Wrapper<'a>) -> Wrapper<'a> {
        //let padding = Size(self.0, self.0);
        //let min_size = thunk.min_size + padding + padding;
        //if self.0 > 0 { thunk = pad(padding, thunk.into()) }
        //let items = vec![self.into(), thunk.into()];
        //let render_fn = render_stack;
        //Wrapper { min_size, items, render_fn }
    //}
//}

//impl Border for Centered {
    //fn around <'a> (&'a self, thunk: Wrapper<'a>) -> Wrapper<'a> {
        //let min_size  = thunk.min_size;
        //let items     = vec![thunk.into()];
        //let render_fn = render_centered;
        //Wrapper { min_size, items, render_fn }
    //}
//}

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
