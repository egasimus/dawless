use crate::electribe2::*;
use thatsit::{*, crossterm::{event::{Event, KeyEvent, KeyCode}, style::Color}};
use thatsit_fs::*;
use std::{rc::Rc, cell::RefCell};
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
    selector: FocusColumn<Collapsible>,
}

impl Electribe2TUI {
    pub fn new () -> Self {
        let frame = Frame { title: "Electribe 2".into(), ..Frame::default() };
        let mut selector = FocusColumn::default();
        let mut tui = Self { focused: false, entered: false, frame, selector, };
        tui.add_button("Edit pattern bank...", Box::new(Electribe2PatternsTUI::new()) as Box<dyn TUI>);
        tui.add_button("Edit sample bank... ", Box::new(Electribe2SamplesTUI::new()) as Box<dyn TUI>);
        return tui
    }
    fn add_button (&mut self, text: &str, feature: Box<dyn TUI>) {
        self.selector.items.push(
            Collapsible(Toggle::new(Button::new(String::from(text), None), feature))
        );
    }
    fn enter (&mut self) {
        self.entered = true;
        //let selected = self.selector.get_mut().unwrap();
        //selected.toggle();
        //panic!("{:?}", selected.state());
        //panic!("{:#?}",self.selector.get_mut().unwrap());//.toggle();
        //self.selected_mut().focus(true);
    }
}

impl TUI for Electribe2TUI {
    fn min_size (&self) -> Size {
        self.selector.min_size()
    }
    fn layout <'b> (&'b self) -> Thunk<'b> {
        stack(|add| {
            add(&self.frame);
            add(&self.selector);//Layout::Item(Sizing::Pad(1, &Sizing::Min), &self.selector),
        })
    }
    fn focus (&mut self, focus: bool) -> bool {
        self.focused = focus;
        self.frame.focused = self.focused;// || self.patterns.focused() || self.samples.focused();
        true
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        self.selector.handle(event)
    }
}
