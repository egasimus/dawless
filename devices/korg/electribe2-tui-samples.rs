use crate::electribe2::*;
use thatsit::{*, crossterm::style::Color};
use thatsit_fs::*;

#[derive(Debug, Default)]
pub struct Electribe2SamplesTUI {
    pub focused: bool,
    pub file_list: FileList,
    pub bank: Option<Electribe2SampleBank>,
    pub sample_list: List<String>,
    pub sample: Blank
}

impl TUI for Electribe2SamplesTUI {
    fn layout <'a> (&'a self) -> Thunk<'a> {
        let Self { focused, .. } = *self;
        Inset(0).around(if let Some(bank) = &self.bank {
            col(|add| {
                add(&self.sample_list);
                add(&self.sample);
            })
        } else {
            (&self.file_list).into()
        })
    }
    fn min_size (&self) -> Size {
        self.layout().min_size
    }
}

impl Electribe2SamplesTUI {
    pub fn new () -> Self {
        let mut new = Self::default();
        new.update_listing();
        return new
    }
    fn update_listing (&mut self) {
        let (entries, _) = list_current_directory();
        self.file_list.list.replace(entries);
    }
}
