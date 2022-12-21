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

impl<'a> TUI<'a> for Electribe2SamplesTUI {
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
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
        }.render(term, area)
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
