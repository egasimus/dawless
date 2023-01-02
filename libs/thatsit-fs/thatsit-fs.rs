use thatsit::*;
use thatsit_focus::*;
use std::{env::current_dir, fs::{metadata, read_dir}};
use crossterm::{
    style::{SetAttribute, Attribute, SetBackgroundColor, SetForegroundColor, Print, Color},
    cursor::MoveTo,
    event::Event
};

#[derive(Debug, Default)]
pub struct FileEntry {
    pub path:    String,
    pub is_dir:  bool,
    pub focused: bool
}

impl FileEntry {
    fn file (path: &str) -> Self {
        FileEntry { path: path.into(), is_dir: false, focused: false }
    }
    fn dir (path: &str) -> Self {
        FileEntry { path: path.into(), is_dir: true, focused: false }
    }
}

impl TUI for FileEntry {
    tui! {
        layout (self, max) {
            Ok(Size(self.path.len() as u16, 1).into())
        }
        render (self, term, area) {
            let Area(Point(x, y), _) = area;
            let fg = Color::White;
            let hi = Color::Yellow;
            let label = format!(" {} {}", if self.is_dir { "📁" } else { "  " }, self.path);
            term.queue(SetAttribute(if self.is_dir { Attribute::Bold } else { Attribute::Reset }))?
                .queue(SetBackgroundColor(Color::AnsiValue(235)))?
                .queue(SetForegroundColor(if self.focused { hi } else { fg }))?
                .queue(MoveTo(x, y))?.queue(Print(label))?;
            Ok(())
        }
    }
}

#[derive(Debug, Default)]
pub struct FileList(pub FocusColumn<FileEntry>);

impl FileList {
    pub fn update (&mut self) -> &mut Self {
        let (entries, _) = list_current_directory();
        self.replace(entries);
        self.select(0);
        self
    }
    pub fn selected (&self) -> &FileEntry {
        self.get().unwrap()
    }
}

impl FocusList<FileEntry> for FileList {
    fn items (&self) -> &Vec<FileEntry> { &self.0.items() }
    fn items_mut (&mut self) -> &mut Vec<FileEntry> { self.0.items_mut() }
    fn state (&self) -> &Focus<usize> { &self.0.state() }
    fn state_mut (&mut self) -> &mut Focus<usize> { self.0.state_mut() }
}

impl TUI for FileList {
    tui! {
        layout (self, max) {
            let mut layout = col(|add|{ for item in self.0.items().iter() { add(item) } });
            layout.min_size = layout.min_size + Size(4, 0);
            Ok(layout)
        }
        handle (self, event) {
            self.0.handle(event)
        }
    }
}

pub fn list_current_directory () -> (Vec<FileEntry>, usize) {
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
    for dir  in dirs.iter()  { entries.push(FileEntry::dir(dir))   }
    for file in files.iter() { entries.push(FileEntry::file(file)) }
    (entries, max_len)
}

