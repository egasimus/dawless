use thatsit::*;
use std::{
    env::current_dir,
    fs::{File, metadata, read_dir},
    io::{Read},
    path::{Path}
};
use crossterm::{
    QueueableCommand,
    style::{SetAttribute, Attribute, SetBackgroundColor, SetForegroundColor, Print},
    cursor::MoveTo,
    event::Event
};

pub type FileListItem = (String, bool);

#[derive(Debug, Default)]
pub struct FileList {
    pub list: List<FileListItem>,
    pub scrollbar: Scrollbar
}

impl TUI for FileList {
    fn handle (&mut self, event: &Event) -> Result<bool> {
        self.list.handle(event)
    }
    fn min_size (&self) -> Size {
        self.list.min_size()
    }
    fn max_size (&self) -> Size {
        self.list.max_size()
    }
    fn render (&self, term: &mut dyn Write, Area(Point(x, y), Size(w, ..)): Area) -> Result<()> {
        let Theme { bg, fg, hi } = self.list.theme;
        for (index, (_, (path, is_dir))) in self.list.items.iter().enumerate() {
            term.queue(SetAttribute(if *is_dir { Attribute::Bold } else { Attribute::Reset }))?
                .queue(SetForegroundColor(if self.list.index == index { hi } else { fg }))?
                .queue(MoveTo(x, y + index as u16))?
                .queue(Print(format!("{} {:<0width$}",
                    if *is_dir { "■" } else { " " },
                    path,
                    width = w as usize
                )))?;
        }
        Ok(())
    }
}

pub fn list_current_directory () -> (Vec<(String, (String, bool))>, usize) {
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

