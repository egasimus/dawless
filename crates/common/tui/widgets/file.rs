use super::{*, super::{*, layout::*}};

pub type FileListItem = (String, bool);

pub struct FileList<'a> (pub &'a List<FileListItem>);

impl<'a> TUI for FileList<'a> {

    fn size (&self) -> Size {
        Size { max_w: Some(self.0.width()), min_h: Some(3), ..Size::default() }
    }

    fn render (&self, term: &mut dyn Write, space: &Space) -> Result<()> {
        let Theme { bg, fg, hi } = self.0.theme;
        let Space(Point(x, y), Point(w, ..)) = *space;
        for (index, (_, (path, is_dir))) in self.0.items.iter().enumerate() {
            term.queue(SetAttribute(if *is_dir { Attribute::Bold } else { Attribute::Reset }))?
                .queue(SetBackgroundColor(bg))?
                .queue(SetForegroundColor(if self.0.index == index { hi } else { fg }))?
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
