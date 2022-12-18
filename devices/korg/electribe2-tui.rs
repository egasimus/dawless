use crate::electribe2::*;
use thatsit::{*, crossterm::{event::Event, style::Color}};
use thatsit_fs::*;
use laterna;

opt_mod::module_flat!(electribe2_tui_patterns @ "electribe2-tui-patterns.rs");
opt_mod::module_flat!(electribe2_tui_samples @ "electribe2-tui-samples.rs");

pub static THEME: Theme = Theme {
    bg: Color::AnsiValue(232),
    fg: Color::White,
    hi: Color::Yellow
};

#[derive(Debug)]
pub struct Electribe2TUI {
    focused:  bool,
    patterns: Toggle<Label, Electribe2PatternsTUI>,
    samples:  Toggle<Label, Electribe2SamplesTUI>,
    frame:    Frame,
    section:  List<Electribe2TUIFeature>,
}

#[derive(Debug, Default)]
pub enum Electribe2TUIFeature {
    #[default]
    Patterns,
    Samples
}

impl Electribe2TUI {
    pub fn new () -> Self {
        let frame = Frame { title: "Electribe 2".into(), ..Frame::default() };
        let mut section = List::default();
        section.add("Edit pattern bank".into(), Electribe2TUIFeature::Patterns)
               .add("Edit sample bank".into(),  Electribe2TUIFeature::Samples);
        let mut patterns = Toggle::new(
            Label::new("Load pattern bank..."),
            Electribe2PatternsTUI::new()
        );
        let samples = Toggle::new(
            Label::new("Load sample bank..."),
            Electribe2SamplesTUI::new()
        );
        patterns.focus(true);
        Self {
            focused:  false,
            section,
            patterns,
            samples,
            frame
        }
    }
    fn selected (&self) -> &dyn TUI {
        match self.section.get().unwrap() {
            Electribe2TUIFeature::Patterns => &self.patterns,
            Electribe2TUIFeature::Samples  => &self.samples,
        }
    }
    fn selected_mut (&mut self) -> &mut dyn TUI {
        match self.section.get().unwrap() {
            Electribe2TUIFeature::Patterns => &mut self.patterns,
            Electribe2TUIFeature::Samples  => &mut self.samples,
        }
    }
    fn focus_selected (&mut self) {
        self.patterns.focus(false);
        self.samples.focus(false);
        self.selected_mut().focus(true);
        self.focus(false);
    }
}

impl TUI for Electribe2TUI {
    fn layout (&self) -> Layout {
        Layout::Layers(Sizing::Min, vec![
            Layout::Item(Sizing::AUTO, &self.frame),
            Layout::Column(Sizing::Pad(1, &Sizing::Min), vec![
                //Layout::Item(Sizing::Min, &DebugBox { bg: Color::AnsiValue(100) }),
                Layout::Item(Sizing::Min, &self.patterns),
                Layout::Item(Sizing::Min, &self.samples)
            ])
        ])
        //])
    }
    fn focus (&mut self, focus: bool) -> bool {
        self.focused = focus;
        self.frame.focused = focus;
        true
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        if self.selected_mut().handle(&event)? {
            self.focus(false);
            return Ok(true)
        }
        if self.section.handle(event)? {
            self.focus_selected();
            return Ok(true)
        }
        //handle_menu_focus!(event, self, self.feature_mut(), self.focused)
        Ok(false)
    }
}
