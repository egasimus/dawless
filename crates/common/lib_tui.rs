use std::io::{Result, Write};

use crossterm::{
    queue,
    style::{
        Print,
        Color, ResetColor, SetForegroundColor, SetBackgroundColor,
        Attribute, SetAttribute
    },
    cursor::MoveTo,
    event::{Event, KeyEvent, KeyCode}
};

pub fn draw_box <W: Write> (
    out:   &mut W,
    col1:  u16,
    row1:  u16,
    cols:  u16,
    rows:  u16,
    bg:    Color,
    title: Option<(Color, Color, &str)>
) -> Result<()> {
    queue!(out,
        ResetColor,
        SetForegroundColor(bg),
        MoveTo(col1, row1),
        Print("▄".repeat(cols as usize)),
        ResetColor, SetBackgroundColor(bg)
    )?;
    let background = " ".repeat(cols as usize);
    for row in row1+1..row1+rows-1 {
        queue!(out, MoveTo(col1, row), Print(&background))?;
    }
    if let Some((bg, fg, text)) = title {
        queue!(out,
            SetBackgroundColor(bg),
            SetForegroundColor(fg),
            MoveTo(col1, row1),
            Print(" "),
            MoveTo(col1+1, row1),
            SetAttribute(Attribute::Bold),
            SetAttribute(Attribute::Underlined),
            Print(text),
            SetAttribute(Attribute::Reset),
            MoveTo(col1+1+text.len() as u16, row1),
            SetBackgroundColor(bg),
            SetForegroundColor(fg),
            Print(" "),
        )?;
    }

    Ok(())
}

pub trait TUI: Sync {
    fn render (&self, _col1: u16, _row1: u16, _cols: u16, _rows: u16) -> Result<()>;
    fn handle (&mut self, _event: &Event) -> Result<bool> {
        Ok(false)
    }
    fn focus (&mut self, _focus: bool) -> bool {
        false
    }
}

pub struct Menu <'a, T> {
    pub index: usize,
    pub items: Vec<(&'a str, T)>
}

impl <'a, T> Menu <'a, T> {
    pub fn new (items: Vec<(&'a str, T)>) -> Self {
        Self {
            index: 0,
            items
        }
    }
    pub fn get (&self) -> Option<&T> {
        self.items.get(self.index).map(|x| &x.1)
    }
    pub fn get_mut (&mut self) -> Option<&mut T> {
        self.items.get_mut(self.index).map(|x| &mut x.1)
    }
}

impl <'a, T: Sync> TUI for Menu <'a, T> {
    fn render (&self, col1: u16, row1: u16, cols: u16, _rows: u16) -> Result<()> {
        let mut out = std::io::stdout();
        let bg = Color::AnsiValue(232);
        let fg = Color::White;
        let hi = Color::Yellow;
        for (index, item) in self.items.iter().enumerate() {
            queue!(out,
                SetBackgroundColor(bg),
                SetForegroundColor(if index == self.index { hi } else { fg }),
                MoveTo(col1, row1 + (index as u16)),
                Print(format!(" {:<0width$} ▶ ", item.0, width = cols as usize))
            )?;
        }
        Ok(())
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        Ok(
            handle_menu_list(event, self.items.len(), &mut self.index)?
        )
    }
}

pub fn handle_menu_list (event: &Event, length: usize, index: &mut usize) -> Result<bool> {
    Ok(match event {
        Event::Key(KeyEvent { code: KeyCode::Up, .. }) => {
            *index = if *index == 0 {
                length - 1
            } else {
                *index - 1
            };
            true
        },
        Event::Key(KeyEvent { code: KeyCode::Down, .. }) => {
            *index = if *index >= length - 1 {
                0
            } else {
                *index + 1
            };
            true
        },
        _ => false
    })
}

#[macro_export] macro_rules! handle_menu_focus {
    ($event:expr, $parent:expr, $child:expr, $focused:expr) => {
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
