use crate::electribe2::*;
use thatsit::*;
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
    fn layout (&self) -> Layout {
        let Self { focused, .. } = *self;
        if let Some(bank) = &self.bank {
            Layout::Layers(Sizing::AUTO, vec![
                Layout::Item(Sizing::AUTO, &self.frame),
                Layout::Column(Sizing::AUTO, vec![
                    Layout::Item(Sizing::AUTO, &self.sample_list),
                    Layout::Item(Sizing::AUTO, &self.sample),
                ])
            ])
        } else {
            Layout::Layers(Sizing::AUTO, vec![
                Layout::Item(Sizing::AUTO, &self.frame),
                Layout::Item(Sizing::AUTO, &self.file_list),
            ])
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
        self.file_list.0.replace(entries);
    }
}
