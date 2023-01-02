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
pub struct FileList(pub Vec<FileEntry>, pub FocusState<usize>);

impl FileList {
    pub fn update (&mut self) -> &mut Self {
        self.replace(list_current_directory().0);
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

impl Focus<FileEntry> for FileList {
    fn items     (&self)     -> &Vec<FileEntry>        { &self.0     }
    fn items_mut (&mut self) -> &mut Vec<FileEntry>    { &mut self.0 }
    fn state     (&self)     -> &FocusState<usize>     { &self.1     }
    fn state_mut (&mut self) -> &mut FocusState<usize> { &mut self.1 }
}

impl Widget for FileList {
    impl_render!(self, out, area => {
        Stacked::y(|row|{
            let focused = self.state().1;
            for (index, item) in self.items().iter().enumerate() {
                let label = format!(" {} {}", if item.is_dir { "📁" } else { "  " }, item.path);
                let color = if focused == Some(index) { Color::Yellow } else { Color::White };
                let bold  = item.is_dir;
                let style = Box::new(move|s: String|{
                    let s = s.with(color);
                    let s = if bold { s.bold() } else { s };
                    s
                });
                row(StyledBoxed(style, label));
            }
        }).render(out, area)
    });
    impl_handle!(self, event => Ok(match_key!((event) {
        KeyCode::Up    => { self.select_prev() },
        KeyCode::Down  => { self.select_next() }
    })));
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
