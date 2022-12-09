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

pub fn render_frame <W: Write> (
    out: &mut W, col1: u16, row1: u16, cols: u16, rows: u16,
    bg: Color, title: Option<(Color, Color, &str)>
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

pub struct Menu <T> {
    pub index: usize,
    pub items: Vec<(String, T)>
}

impl <T> Menu <T> {
    pub fn new (items: Vec<(String, T)>) -> Self {
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

impl <T: Sync> TUI for Menu <T> {
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
        handle_menu_selection(event, self.items.len(), &mut self.index)
    }
}

pub fn handle_menu_selection (event: &Event, length: usize, index: &mut usize) -> Result<bool> {
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

pub fn render_directory_listing <W: Write> (
    out: &mut W, col1: u16, row1: u16, pad: usize,
    items: &Vec<(String, (String, bool))>,
    selected: usize,
) -> Result<()> {
    let bg = Color::AnsiValue(232);
    let fg = Color::White;
    let hi = Color::Yellow;
    for (index, (_, (path, is_dir))) in items.iter().enumerate() {
        queue!(out,
            SetAttribute(if *is_dir { Attribute::Bold } else { Attribute::Reset }),
            SetBackgroundColor(bg),
            SetForegroundColor(if selected == index { hi } else { fg }),
            MoveTo(col1, row1 + index as u16),
            Print(format!("{} {:<0width$}",
                if *is_dir { "■" } else { " " },
                path,
                width = pad as usize
            )),
        )?;
    }
    Ok(())
}

pub fn list_current_directory () -> (Vec<(String, (String, bool))>, usize) {
    use std::env::current_dir;
    use std::fs::{read_dir, metadata};
    let cwd = current_dir().unwrap();
    let mut dirs: Vec<String> = vec!["..".into()];
    let mut files: Vec<String> = vec![];
    let mut max_len: usize = 32;
    for file in read_dir(cwd).unwrap() {
        let file = file.unwrap();
        let name: String = file.path().file_name().unwrap().to_str().unwrap().into();
        max_len = usize::max(max_len, name.len());
        if metadata(file.path()).unwrap().is_dir() {
            dirs.push(name)
        } else {
            files.push(name)
        }
    }
    dirs.sort();
    files.sort();

    let mut entries = vec![];
    for dir in dirs.iter() {
        entries.push((dir.clone(), (dir.clone(), true)))
    }
    for file in files.iter() {
        entries.push((file.clone(), (file.clone(), false)))
    }
    (entries, max_len)
}

pub fn render_scrollbar <W: Write> (
    out: &mut W, col1: u16, row1: u16,
    length: usize, offset: usize, height: usize,
) -> Result<()> {
    let fg = Color::White;
    let hi = Color::Yellow;
    for index in 0..height {
        let scroll_offset = (offset * height) / length;
        let scroll_index  = (index  * height) / length;
        queue!(out,
            SetForegroundColor(if scroll_offset == scroll_index { hi } else { fg }),
            MoveTo(col1, row1 + index as u16),
            Print("▒"),
        )?;
    }
    Ok(())
}
