use std::path::PathBuf;
use std::env::{current_dir, set_current_dir};
use std::fs::{read_dir, metadata};
use std::io::{Result, Write};
use pathdiff::diff_paths;

use crossterm::{
    queue,
    style::{
        Print,
        Color, ResetColor, SetForegroundColor, SetBackgroundColor,
        Attribute, SetAttribute
    },
    cursor::MoveTo,
    event::Event
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
    queue!(out, ResetColor, SetForegroundColor(bg))?;

    let border = "▄".repeat(cols as usize);
    queue!(out, MoveTo(col1, row1), Print(&border))?;

    //let border = "▀".repeat(cols as usize);
    //queue!(out, MoveTo(col1, row1 + rows - 1), Print(&border))?;

    let background = " ".repeat(cols as usize);
    queue!(out, ResetColor, SetBackgroundColor(bg))?;
    for row in row1+1..row1+rows-1 {
        queue!(out, MoveTo(col1, row), Print(&background))?;
    }

    if let Some((bg, fg, text)) = title {
        queue!(out,
            SetBackgroundColor(bg),
            SetForegroundColor(fg),
            MoveTo(col1, row1),
            SetAttribute(Attribute::Bold),
            Print(format!(" {text} ")),
            SetAttribute(Attribute::Reset)
        )?;
    }

    Ok(())
}

pub trait TUI: Sync {
    fn render (&self, _col1: u16, _row1: u16, _cols: u16, _rows: u16) -> Result<()>;
    fn update (&mut self, _event: Event) -> Result<()> {
        Ok(())
    }
}

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
