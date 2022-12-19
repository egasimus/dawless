use crate::electribe2::*;
use thatsit::{*, crossterm::{event::{Event, KeyEvent, KeyCode}, style::Color}};
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
    entered:  bool,
    frame:    Frame,
    selector: List<Electribe2TUIFeature>,
}

impl Electribe2TUI {
    pub fn new () -> Self {
        let frame = Frame { title: "Electribe 2".into(), ..Frame::default() };
        let mut selector = List::default();
        selector
            .add(
                "Edit pattern bank".into(),
                Electribe2TUIFeature::Patterns(Toggle::new(
                    Label::new("Load pattern bank..."),
                    Electribe2PatternsTUI::new()
                ))
            )
            .add(
                "Edit sample bank".into(),
                Electribe2TUIFeature::Samples(Toggle::new(
                    Label::new("Load sample bank..."),
                    Electribe2SamplesTUI::new()
                ))
            );
        Self {
            focused: false,
            entered: false,
            frame,
            selector,
        }
    }
    fn enter (&mut self) {
        self.entered = true;
        let selected = self.selector.get_mut().unwrap();
        selected.toggle();
        //panic!("{:?}", selected.state());
        //panic!("{:#?}",self.selector.get_mut().unwrap());//.toggle();
        //self.selected_mut().focus(true);
    }
    fn handle_child (&mut self, event: &Event) -> Result<bool> {
        Ok(false)
    }
}

impl TUI for Electribe2TUI {
    fn layout (&self) -> Layout {
        Layout::Layers(Sizing::Min, vec![
            Layout::Item(Sizing::AUTO, &self.frame),
            Layout::Item(Sizing::Pad(1, &Sizing::Min), &self.selector),
        ])
        //])
    }
    fn focus (&mut self, focus: bool) -> bool {
        self.focused = focus;
        self.frame.focused = self.focused;// || self.patterns.focused() || self.samples.focused();
        true
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        Ok(
            self.handle_child(event)? ||
            is_key!(event => KeyCode::Enter => {self.enter();true}) ||
            self.selector.handle(event)?//.map(|_|{self.focus_selected();true})?
        )
    }
}

#[derive(Debug, Default)]
pub enum Electribe2TUIFeature {
    #[default]
    None,
    Patterns(Toggle<Label, Electribe2PatternsTUI>),
    Samples(Toggle<Label, Electribe2SamplesTUI>)
}

impl Electribe2TUIFeature {
    fn selected <T> (&self) -> Option<&dyn TUI> {
        match self {
            Self::None => None,
            Self::Patterns(toggle) => Some(&toggle.open),
            Self::Samples(toggle)  => Some(&toggle.open),
        }
    }
    fn selected_mut (&mut self) -> Option<&mut dyn TUI> {
        match self {
            Self::None => None,
            Self::Patterns(toggle) => Some(toggle.open_mut()),
            Self::Samples(toggle)  => Some(toggle.open_mut()),
        }
    }
    fn toggle (&mut self) {
        match self {
            Self::None => {},
            Self::Patterns(toggle) => toggle.toggle(),
            Self::Samples(toggle)  => toggle.toggle(),
        }
    }
    fn state (&mut self) -> Option<bool> {
        match self {
            Self::None => None,
            Self::Patterns(toggle) => Some(toggle.get()),
            Self::Samples(toggle)  => Some(toggle.get()),
        }
    }
}
