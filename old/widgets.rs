use crate::*;

/// The empty widget
#[derive(Debug, Default, Copy, Clone)]
pub struct Spacer(pub Size);

/// A global instance of the 0x0 empty widget
pub const BLANK: &'static Spacer = &Spacer(Size::MIN);

/// A global instance of the 1x1 empty widget
pub const SPACE: &'static Spacer = &Spacer(Size(1, 1));

impl TUI for Spacer {
    tui! { 'a layout (self, max) { self.0.constrain(max, Layout::One(self)) } }
}

/// A debug widget that displays its size and position on a colored background
pub struct DebugBox { pub bg: Color }

impl Default for DebugBox {
    fn default () -> Self {
        Self { bg: Color::AnsiValue(123) }
    }
}

tui! {
    <'a> DebugBox {
        <'b> layout (self, max) {
            Size(16, 3).constrain(max, Layout::CustomBox(Box::new(move |term, area|{
                let Area(Point(x, y), Size(w, h)) = area;
                let background = " ".repeat(w as usize);
                term.queue(SetBackgroundColor(self.bg))?
                    .queue(SetForegroundColor(Color::AnsiValue(234)))?;
                for row in y..y+h { term.queue(MoveTo(x, row))?.queue(Print(&background))?; }
                let text = format!("{w}x{h}+{x}+{y}");
                term.queue(MoveTo(x, y))?.queue(Print(&text))?;
                Ok(())
            })))
        }
    }
}

tui! {
    <'a> String {
        <'b> layout (self, max) {
            Size(self.len() as u16, 1).constrain(max, Layout::CustomBox(Box::new(move |term, area|{
                let Area(Point(x, y), Size(w, h)) = area;
                term.queue(MoveTo(x, y))?.queue(Print(&self))?;
                Ok(())
            })))
        }
    }
}

tui! {
    <'a> &str {
        <'b> layout (self, max) {
            Size(self.len() as u16, 1).constrain(max, Layout::CustomBox(Box::new(move |term, area|{
                let Area(Point(x, y), Size(w, h)) = area;
                term.queue(MoveTo(x, y))?.queue(Print(&self))?;
                Ok(())
            })))
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

tui! {
    <'a> Text<'a> {
        <'b> layout (self, max) {
            Size(self.0.len() as u16, 1).constrain(max, Layout::CustomBox(Box::new(move |term,area|{
                let Area(Point(x, y), Size(w, h)) = area;
                term.queue(MoveTo(x, y))?.queue(Print(&self.0))?;
                Ok(())
            })))
        }
    }
}

/// A background rectangle of a fixed color
#[derive(Debug)]
pub struct Filled(pub Color);

tui! {
    <'a> Filled {
        <'b> layout (self, max) {
            Ok(Layout::CustomBox(Box::new(move |term,area|{
                let Area(Point(x, y), Size(w, h)) = area;
                let background  = " ".repeat(w as usize);
                term.queue(ResetColor)?.queue(SetBackgroundColor(self.0))?;
                for y in y..y+h { term.queue(MoveTo(x, y))?.queue(Print(&background))?; }
                Ok(())
            })))
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
        'a
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
        'a
        layout (self, max) {
            let layout = Layout::CustomBox(Box::new(|term, area| {
                //let border = (if self.pressed {
                    //&Inset(0) as &dyn TUI
                //} else {
                    //&Outset(0) as &dyn TUI
                //})
                    //.layout(max)?
                    //.render(term, area)?;
                //let bg = Color::AnsiValue(232);
                let fg = Color::White;
                let hi = Color::Yellow;
                let Area(Point(x, y), _) = area;
                term.queue(ResetColor)?
                    //.queue(SetBackgroundColor(if self.focused { Color::AnsiValue(240) } else { Color::AnsiValue(238) }))?
                    .queue(SetForegroundColor(if self.focused { hi } else { fg }))?
                    .queue(MoveTo(x+3, y+1))?.queue(Print(&self.text))?;
                Ok(())
            }));
            Size(self.text.len() as u16 + 6, 3).constrain(max, layout)
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
pub struct Columns<'a, const N: usize>(pub [&'a dyn TUI; N]);

impl<'a, const N: usize> Columns<'a, N> {
    fn render (items: &Vec<Layout<'a>>, term: &mut dyn Write, area: Area)->Result<()> {
        let mut y = area.0.y();
        let max_y = area.0.y() + area.1.height();
        for item in items.iter() {
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

impl<'a, const N: usize> TUI for Columns<'a, N> {
    tui! { 'b
        layout (self, max) {
            let items = self.0.iter().map(|item|Layout::One(item)).collect();
            Ok(Layout::Many(max, &Self::render, items))
        }
    }
}

pub struct Rows<'a, const N: usize>(pub [&'a dyn TUI; N]);

impl<'a, const N: usize> Rows<'a, N> {
    fn render (items: &Vec<Layout<'a>>, term: &mut dyn Write, area: Area)->Result<()> {
        let mut x = area.0.x();
        for item in items.iter() {
            let size = Size::MIN;//item.min_size();
            let area = Area(Point(x, area.0.y()), size);
            item.render(term, area)?;
            x = x + size.width();
        }
        Ok(())
    }
}

impl<'a, const N: usize> TUI for Rows<'a, N> {
    tui! {
        'b
        layout (self, max) {
            let items = self.0.iter().map(|item|Layout::One(item)).collect();
            Ok(Layout::Many(max, &Self::render, items))
        }
    }
}

pub struct Layers<'a, const N: usize>(pub [&'a dyn TUI; N]);

impl<'a, const N: usize> Layers<'a, N> {
    fn render (items: &Vec<Layout<'a>>, term: &mut dyn Write, area: Area)->Result<()> {
        for item in items.iter() { item.render(term, area)?; }
        Ok(())
    }
}

impl<'a, const N: usize> TUI for Layers<'a, N> {
    tui! {
        'b
        layout (self, max) {
            Ok(Layout::Many(
                max,
                &Self::render,
                self.0.iter().map(|item|Layout::One(item)).collect()
            ))
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
    fn render <'a> (term: &mut dyn Write, area: Area)->Result<()> {
        let Area(Point(x, y), Size(w, h)) = area;
        if w == 0 || h == 0 { return Ok(()) }
        let bg = Color::AnsiValue(235);
        //Filled(bg).layout(max)?.render(term, Area(Point(x, y), Size(w, h)))?;
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

impl TUI for Inset {
    tui! {
        'a
        layout (self, max) {
            Ok(Layout::CustomFn(Self::render))
        }
    }
}

/// An outset border
#[derive(Default, Debug, Copy, Clone)]
pub struct Outset(
    /// The amount of padding between the border and the content
    pub Unit
);

impl Outset {
    fn render (term: &mut dyn Write, area: Area) -> Result<()> {
        let Area(Point(x, y), Size(w, h)) = area;
        if w == 0 || h == 0 { return Ok(()) }
        let bg = Color::AnsiValue(235);
        //Filled(bg).layout(max)?.render(term, Area(Point(x, y), Size(w, h)))?;
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

impl TUI for Outset {
    tui! {
        'a
        layout (self, max) {
            Ok(Layout::CustomFn(Outset::render))
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Centered;

impl TUI for Centered {}
