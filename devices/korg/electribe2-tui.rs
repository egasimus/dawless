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
    selector: Accordion<Box<dyn TUI>>,
}

impl Electribe2TUI {
    pub fn new () -> Self {
        let frame = Frame { title: "Electribe 2".into(), ..Frame::default() };
        let mut selector = Accordion::default();
        selector
            .add(
                "Edit pattern bank".into(),
                Box::new(Electribe2PatternsTUI::new()) as Box<dyn TUI>
            )
            .add(
                "Edit sample bank".into(),
                Box::new(Electribe2SamplesTUI::new())
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
        //let selected = self.selector.get_mut().unwrap();
        //selected.toggle();
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
            self.selector.handle(event)?//.map(|_|{self.focus_selected();true})?
        )
    }
}
