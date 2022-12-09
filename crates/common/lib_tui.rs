use std::io::{Result, Write};

use crossterm::{
    QueueableCommand,
    style::{
        Print,
        Color, ResetColor, SetForegroundColor, SetBackgroundColor,
        Attribute, SetAttribute
    },
    cursor::MoveTo,
    event::{Event, KeyEvent, KeyCode}
};

#[macro_export] macro_rules! tui {
    ($($body:item)*) => {
        #[cfg(feature = "tui")] pub use tui::*;
        #[cfg(feature = "tui")] mod tui {
            use super::*;
            use std::fs::{File, metadata};
            use std::io::{Read, Write};
            use std::path::{Path, PathBuf};
            use dawless_common::read;
            $($body)*
        }
    }
}

pub fn render_frame (
    term: &mut dyn Write, col1: u16, row1: u16, cols: u16, rows: u16,
    bg: Color, title: Option<(Color, Color, &str)>
) -> Result<()> {

    term.queue(ResetColor)?
        .queue(SetForegroundColor(bg))?
        .queue(MoveTo(col1, row1))?
        .queue(Print("▄".repeat(cols as usize)))?
        .queue(ResetColor)?
        .queue(SetBackgroundColor(bg))?;

    let background = " ".repeat(cols as usize);
    for row in row1+1..row1+rows-1 {
        term.queue(MoveTo(col1, row))?.queue(Print(&background))?;
    }

    if let Some((bg, fg, text)) = title {
        term.queue(SetBackgroundColor(bg))?
            .queue(SetForegroundColor(fg))?
            .queue(MoveTo(col1, row1))?
            .queue(Print(" "))?
            .queue(MoveTo(col1+1, row1))?
            .queue(SetAttribute(Attribute::Bold))?
            .queue(SetAttribute(Attribute::Underlined))?
            .queue(Print(text))?
            .queue(SetAttribute(Attribute::Reset))?
            .queue(MoveTo(col1+1+text.len() as u16, row1))?
            .queue(SetBackgroundColor(bg))?
            .queue(SetForegroundColor(fg))?
            .queue(Print(" "))?;
    }

    Ok(())

}

pub trait TUI: Sync {
    fn render (&self, term: &mut dyn Write, _col1: u16, _row1: u16, _cols: u16, _rows: u16)
        -> Result<()>;
    fn handle (&mut self, _event: &Event)
        -> Result<bool> { Ok(false) }
    fn focus (&mut self, _focus: bool)
        -> bool { false }
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
    pub fn len (&self) -> usize {
        self.items.len()
    }
}

impl <T: Sync> TUI for Menu <T> {
    fn render (&self, term: &mut dyn Write, col1: u16, row1: u16, cols: u16, _rows: u16) -> Result<()> {
        let bg = Color::AnsiValue(232);
        let fg = Color::White;
        let hi = Color::Yellow;
        for (index, item) in self.items.iter().enumerate() {
            term.queue(SetBackgroundColor(bg))?
                .queue(SetForegroundColor(if index == self.index { hi } else { fg }))?
                .queue(MoveTo(col1, row1 + (index as u16)))?
                .queue(Print(format!(" {:<0width$} ▶ ", item.0, width = cols as usize)))?;
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

pub fn render_directory_listing (
    term: &mut dyn Write, col1: u16, row1: u16, pad: usize,
    items: &Vec<(String, (String, bool))>,
    selected: usize,
) -> Result<()> {
    let bg = Color::AnsiValue(232);
    let fg = Color::White;
    let hi = Color::Yellow;
    for (index, (_, (path, is_dir))) in items.iter().enumerate() {
        term.queue(SetAttribute(if *is_dir { Attribute::Bold } else { Attribute::Reset }))?
            .queue(SetBackgroundColor(bg))?
            .queue(SetForegroundColor(if selected == index { hi } else { fg }))?
            .queue(MoveTo(col1, row1 + index as u16))?
            .queue(Print(format!("{} {:<0width$}",
                if *is_dir { "■" } else { " " },
                path,
                width = pad as usize
            )))?;
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

pub fn render_scrollbar (
    term: &mut dyn Write, col1: u16, row1: u16,
    length: usize, offset: usize, height: usize,
) -> Result<()> {
    let fg = Color::White;
    let hi = Color::Yellow;
    for index in 0..height {
        let scroll_offset = (offset * height) / length;
        let scroll_index  = (index  * height) / length;
        term.queue(SetForegroundColor(if scroll_offset == scroll_index { hi } else { fg }))?
            .queue(MoveTo(col1, row1 + index as u16))?
            .queue(Print("▒"))?;
    }
    Ok(())
}
