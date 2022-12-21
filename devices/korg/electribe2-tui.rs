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
pub struct Electribe2TUI<'a> {
    focused:  bool,
    entered:  bool,
    frame:    Frame,
    selector: Accordion<'a, Box<dyn TUI<'a>>>,
}

impl<'a> Electribe2TUI<'a> {
    pub fn new () -> Self {
        let frame = Frame { title: "Electribe 2".into(), ..Frame::default() };
        let mut selector = Accordion::default();
        selector
            .add(
                "Edit pattern bank".into(),
                Box::new(Electribe2PatternsTUI::new()) as Box<dyn TUI>
            );
            //.add(
                //"Edit sample bank".into(),
                //Box::new(Electribe2SamplesTUI::new())
            //);
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
}

impl<'a> TUI<'a> for Electribe2TUI<'a> {
    fn render (&'a self, term: &mut dyn Write, area: Area) -> Result<()> {
        stack(|add| {
            add(&self.frame);
            add(&self.selector);//Layout::Item(Sizing::Pad(1, &Sizing::Min), &self.selector),
        });//.render(term, area)
        Ok(())
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
