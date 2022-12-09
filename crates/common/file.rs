use std::env::current_dir;
use std::fs::{File, metadata, read_dir};
use std::io::{Read};
use std::path::{Path};

pub fn read (filename: &Path) -> Vec<u8> {
    let mut f      = File::open(&filename).expect("file not found");
    let metadata   = metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    buffer
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
