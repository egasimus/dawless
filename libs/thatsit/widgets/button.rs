use crate::*;

#[derive(Default)]
pub struct Button {
    pub theme:   Theme,
    pub focused: bool,
    pub text:    String,
    pub click:   Option<Box<dyn FnMut() -> ()>>
}

impl Button {
    pub fn new (
        text:  impl Into<String>,
        click: Option<Box<dyn FnMut() -> ()>>
    ) -> Self {
        Self { text: text.into(), click, ..Self::default() }
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
            if let Some(click) = &mut self.click {
                (click)();
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
