use crate::electribe2::*;
use thatsit::{*, crossterm::style::Color};
use thatsit_fs::*;

#[derive(Debug, Default)]
pub struct Electribe2SamplesTUI {
    pub focused: bool,
    pub frame: Frame,
    pub file_list: FileList,
    pub bank: Option<Electribe2SampleBank>,
    pub sample_list: List<String>,
    pub sample: Blank
}

impl TUI for Electribe2SamplesTUI {
    fn layout <'a> (&'a self) -> Thunk<'a> {
        let Self { focused, .. } = *self;
        if let Some(bank) = &self.bank {
            stack(|add| {
                add(&self.frame);
                col(|add| {
                    add(&self.sample_list);
                    add(&self.sample);
                });
            })
        } else {
            stack(|add| {
                add(&self.frame);
                add(&self.file_list);
            })
        }
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
