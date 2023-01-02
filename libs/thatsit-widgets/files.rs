use thatsit::*;
use std::{env::current_dir, fs::{metadata, read_dir}};
use crossterm::{
    style::{SetAttribute, Attribute, SetBackgroundColor, SetForegroundColor, Print, Color},
    cursor::MoveTo,
};

/// Quick and dirty way to get directories then files
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

/// A listing of a directory.
/// FIXME deduplicate (currently FocusStack always erases the type)
#[derive(Debug, Default)]
pub struct FileList<'a>(pub Vec<FileEntry>, pub FocusStack<'a>);

impl<'a> FileList<'a> {
    pub fn update (&mut self) -> &mut Self {
        self.0 = list_current_directory().0;
        let entries = self.0.clone();
        self.replace(entries.into_iter().map(|entry|Layout::Box(Box::new(entry))).collect());
        self.select(0);
        self
    }
    pub fn selected (&self) -> Option<&FileEntry> {
        match Focus::selected(self) {
            Some(index) => Some(&self.0[index]),
            None => None
        }
    }
}

impl<'a> Focus<Layout<'a>> for FileList<'a> {
    fn items     (&self)     -> &Vec<Layout<'a>>       { self.1.items()     }
    fn items_mut (&mut self) -> &mut Vec<Layout<'a>>   { self.1.items_mut() }
    fn state     (&self)     -> &FocusState<usize>     { &self.1.state()    }
    fn state_mut (&mut self) -> &mut FocusState<usize> { self.1.state_mut() }
}

impl<'a> Widget for FileList<'a> {
    impl_render!(self, out, area => {
        Stacked::y(|row|{ for item in self.1.items().iter() { row(item) } }).render(out, area)
    });
}

#[derive(Debug, Default, Clone)]
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

impl Widget for FileEntry {
    impl_render!(self, out, area => {
        let Area(x, y, ..) = area;
        let fg = Color::White;
        let hi = Color::Yellow;
        let label = format!(" {} {}", if self.is_dir { "📁" } else { "  " }, self.path);
        out
            .queue(SetAttribute(if self.is_dir { Attribute::Bold } else { Attribute::Reset }))?
            .queue(SetBackgroundColor(Color::AnsiValue(235)))?
            .queue(SetForegroundColor(if self.focused { hi } else { fg }))?
            .queue(MoveTo(x, y))?.queue(Print(&label))?;
        Ok((label.len() as Unit, 1))
    });
}
