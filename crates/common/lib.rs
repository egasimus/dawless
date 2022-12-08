use std::path::PathBuf;
use std::env::{current_dir, set_current_dir};
use std::fs::{read_dir, metadata};
use pathdiff::diff_paths;

mod lib_tui;
pub use lib_tui::*;

#[macro_export] macro_rules! module {
    ($name:ident) => {
        mod $name;
        pub use $name::*;
    };
}

#[macro_export] macro_rules! cli {
    ($($body:item)*) => {
        #[cfg(feature = "cli")] pub use cli::*;
        #[cfg(feature = "cli")] mod cli {
            use super::*;
            use std::fs::{File, metadata};
            use std::io::{Read, Write};
            use std::path::{Path, PathBuf};
            pub fn read (filename: &Path) -> Vec<u8> {
                let mut f      = File::open(&filename).expect("file not found");
                let metadata   = metadata(&filename).expect("unable to read metadata");
                let mut buffer = vec![0; metadata.len() as usize];
                f.read(&mut buffer).expect("buffer overflow");
                buffer
            }
            $($body)*
        }
    }
}

#[macro_export] macro_rules! tui {
    ($($body:item)*) => {
        #[cfg(feature = "tui")] pub use tui::*;
        #[cfg(feature = "tui")] mod tui {
            use super::*;
            use std::fs::{File, metadata};
            use std::io::{Read, Write};
            use std::path::{Path, PathBuf};
            pub fn read (filename: &Path) -> Vec<u8> {
                let mut f      = File::open(&filename).expect("file not found");
                let metadata   = metadata(&filename).expect("unable to read metadata");
                let mut buffer = vec![0; metadata.len() as usize];
                f.read(&mut buffer).expect("buffer overflow");
                buffer
            }
            $($body)*
        }
    }
}

//pub trait FileView: cursive::View {
    //fn set_file (&mut self, name: String);
//}

//pub fn pick_file <'a, V: FileView> (siv: &mut Cursive, name: &str) {
    //let cwd    = current_dir().unwrap();
    //let parent = cwd.parent().unwrap().to_path_buf();
    //let mut files: Vec<(String, bool)> = vec![];
    //for file in read_dir(cwd).unwrap() {
        //let file = file.unwrap();
        //files.push((
            //file.path().display().to_string(),
            //metadata(file.path()).unwrap().is_dir()
        //));
    //}
    //files.sort();
    //let name2  = String::from(name);
    //let name3  = String::from(name);
    //let files2 = files.clone();
    //let files3 = files.clone();
    //let mut tree = TreeView::new()
        //.on_collapse(move |siv, i, _, _| {
            //if i == 0 {
                //set_current_dir(&parent).unwrap();
                //siv.pop_layer();
                //pick_file::<V>(siv, &name2.clone());
            //} else if files2[i].1 {
                //set_current_dir(&files2[i-1].0).unwrap();
                //siv.pop_layer();
                //pick_file::<V>(siv, &name2.clone());
            //}
        //})
        //.on_submit(move |siv, i| {
            //if !files3[i].1 {
                //siv.pop_layer();
                //siv.call_on_name(&name3, |v: &mut V| {
                    //v.set_file(files3[i].0.clone())
                //});
            //}
        //});
    //let cwd = current_dir().unwrap();
    //tree.insert_item(
        //cwd.display().to_string(), Placement::LastChild, 0
    //);
    //for (file, is_dir) in files.iter() {
        //let name = diff_paths(&file, &cwd).unwrap();
        //if *is_dir {
            //tree.insert_container_item(name.display().to_string(), Placement::LastChild, 0);
        //} else {
            //tree.insert_item(name.display().to_string(), Placement::LastChild, 0);
        //}
    //}
    //let dialog = Dialog::around(tree)
        //.title("Pick a file");
    //siv.add_layer(dialog);
//}
