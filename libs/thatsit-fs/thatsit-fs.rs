use thatsit::*;
use std::{
    env::current_dir,
    fs::{File, metadata, read_dir},
    io::{Read},
    path::{Path}
};
use crossterm::{
    QueueableCommand,
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
    impl_focus!(focused);
    fn min_size (&self) -> Size { Size(self.path.len() as u16, 1) }
    fn render (&self, term: &mut dyn Write, Area(Point(x, y), _): Area) -> Result<()> {
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

#[derive(Debug, Default)]
pub struct FileList(pub FocusColumn<FileEntry>);

impl FileList {
    pub fn select (&mut self, index: usize) -> &mut Self { self.0.0.focus(index); self }
    pub fn index (&self) -> usize { self.0.0.index }
    pub fn replace (&mut self, items: Vec<FileEntry>) -> &mut Self {
        self.0.replace(items);
        self
    }
    pub fn update (&mut self) -> &mut Self {
        let (entries, _) = list_current_directory();
        self.replace(entries);
        self.select(0);
        self
    }
    pub fn selected (&self) -> &FileEntry {
        self.0.0.items.get(self.index()).unwrap()
    }
}

impl TUI for FileList {
    fn layout <'a> (&'a self) -> Thunk<'a> {
        col(|add|{ for item in self.0.0.items.iter() { add(item) } })
    }
    fn handle (&mut self, event: &Event) -> Result<bool> { self.0.handle(event) }
    fn min_size (&self) -> Size { self.0.min_size() + Size(4, 0) }
    fn max_size (&self) -> Size { self.min_size() }
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

