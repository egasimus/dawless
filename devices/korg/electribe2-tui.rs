use crate::electribe2::*;
use std::{rc::Rc, cell::RefCell};
use thatsit::{*, crossterm::{
    QueueableCommand,
    event::{Event, KeyEvent, KeyCode, KeyModifiers, KeyEventKind, KeyEventState},
    cursor::MoveTo,
    style::{
        Color, ResetColor, SetBackgroundColor, SetForegroundColor,
        Attribute, SetAttribute,
        Print
    },
}};
use thatsit_fs::*;
use laterna;

pub static THEME: Theme = Theme {
    bg: Color::AnsiValue(232),
    fg: Color::White,
    hi: Color::Yellow
};

#[derive(Debug)]
pub struct Electribe2TUI {
    focused:  bool,
    entered:  bool,
    selector: FocusColumn<Collapsible>,
}

impl Electribe2TUI {
    pub fn new () -> Self {
        let mut selector = FocusColumn::new(vec![
            Self::feature("Edit pattern bank...", Box::new(Electribe2PatternsTUI::new()) as Box<dyn TUI>),
            Self::feature("Edit sample bank... ", Box::new(Electribe2SamplesTUI::new())  as Box<dyn TUI>),
        ]);
        selector.0.items[0].focus(true);
        Self { focused: false, entered: false, selector, }
    }
    fn feature (text: &str, feature: Box<dyn TUI>) -> Collapsible {
        Collapsible(Toggle::new(Button::new(String::from(text), Some(Box::new(||Ok(false)))), feature))
    }
    fn enter (&mut self) {
        self.entered = true;
    }
}

impl TUI for Electribe2TUI {
    fn layout <'a> (&'a self) -> Thunk<'a> {
        (&self.selector).into()
    }
    fn min_size (&self) -> Size {
        self.selector.min_size()
    }
    fn focus (&mut self, focus: bool) -> bool {
        self.focused = focus;
        true
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        Ok(if self.entered {
            self.selector.get_mut().handle(event)? || if event == &key!(Esc) {
                self.entered = false;
                self.selector.get_mut().collapse();
                true
            } else if event == &key!(Enter) {
                self.entered = false;
                self.selector.get_mut().collapse();
                true
            } else {
                false
            }
        } else {
            self.selector.handle(event)?;
            if event == &key!(Enter) {
                self.entered = true;
                true
            } else {
                false
            }
        })
    }
}

#[derive(Debug, Default)]
pub struct Electribe2PatternsTUI {
    pub focused:   bool,
    pub label:     Label,
    pub file_list: FileList,
    pub bank:      Option<Electribe2PatternBank>,
    pub patterns:  Electribe2PatternList,
    pub pattern:   Electribe2PatternTUI,
    pub offset:    usize,
}

impl Electribe2PatternsTUI {
    pub fn new () -> Self {
        let mut new = Self::default();
        new.label.text = " Select pattern bank:".into();
        new.update_listing();
        return new
    }
    pub fn import (&mut self, bank: &std::path::Path) {
        let data = crate::read(bank);
        let bank = Electribe2PatternBank::read(&data);
        self.bank = Some(bank);
        let patterns: Vec<Electribe2PatternTUI> = self.bank.as_ref().unwrap().patterns.iter()
            .map(|pattern|Electribe2PatternTUI(pattern.clone()))
            .collect();
        self.patterns.0.replace(patterns);
    }
    fn update_listing (&mut self) {
        let (entries, _) = list_current_directory();
        self.file_list.replace(entries);
    }
}

impl TUI for Electribe2PatternsTUI {
    fn layout <'a> (&'a self) -> Thunk<'a> {
        let Self { focused, offset, bank, .. } = self;
        Inset(1).around(if let Some(bank) = &bank {
            row(|add|{ add(&self.patterns); add(&self.pattern); })
        } else {
            col(|add|{ add(&self.label); add(&self.file_list); })
        })
    }
    fn min_size (&self) -> Size {
        self.layout().min_size
    }
    fn focus (&mut self, focus: bool) -> bool {
        self.focused = focus;
        true
    }
    fn focused (&self) -> bool {
        self.focused
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        if let Some(bank) = &self.bank {
            if self.patterns.0.handle(event)? {
                self.offset = handle_scroll(
                    self.patterns.0.0.items.len(), self.patterns.0.0.index, 36, self.offset
                );
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(if_key!(event => KeyCode::Enter => {
                let (path, is_dir) = &self.file_list.0.items.get(self.file_list.0.index).unwrap().1;
                if *is_dir {
                    std::env::set_current_dir(path)?;
                    self.update_listing();
                } else {
                    let path = std::path::PathBuf::from(path);
                    self.import(&path);
                }
                true
            }) || self.file_list.handle(event)?)
        }
    }
}

#[derive(Debug, Default)]
pub struct Electribe2PatternList(FocusColumn<Electribe2PatternTUI>);

impl TUI for Electribe2PatternList {
    fn layout <'a> (&'a self) -> Thunk<'a> {
        self.0.layout()
    }
    fn min_size (&self) -> Size {
        Size(24, 10)
    }
    fn max_size (&self) -> Size {
        Size(24, Unit::MAX)
    }
    //fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        //return self.0.render(term, area);
        //return Layout::Item(
            //Sizing::Range(self.min_size(), self.max_size()), &self.0
        //).render(term, area);
        //let Area(Point(x, y), Size(w, h)) = area;
        //let Self { offset, .. } = *self;
        //let Theme { bg, fg, hi } = THEME;
        //term.queue(SetBackgroundColor(bg))?
            //.queue(SetForegroundColor(fg))?
            //.queue(SetAttribute(Attribute::Bold))?
            //.queue(MoveTo(x, y))?
            //.queue(Print(format!("{:>3}  {:<16}  {:<5} {:>3}  {:>3}  {:>3}  {:>3}",
                //"#", "Name", "BPM", "Length", "Beats", "Key", "Scale"
            //)))?
            //.queue(SetAttribute(Attribute::Reset))?
            //.queue(SetBackgroundColor(bg))?;
        //let height = 36;
        //for index in 0..0+height {
            //let index_offset = index + self.offset;
            //let focused = self.selected == index_offset;
            //let text = if let Some(pattern) = self.patterns.get(index_offset as usize) {
                //format!("{:>3}  {:<16} {:>5.1}   {:>3}    {:>3}    {:>3}   {:>3}",
                    //index + self.offset + 1,
                    //pattern.name.trim(),
                    //pattern.bpm,
                    //pattern.length,
                    //pattern.beats,
                    //pattern.key,
                    //pattern.scale,
                //)
            //} else {
                //"".into()
            //};
            ////Label { theme: THEME, focused, text }
                ////.render(term, &Space::new(x, y + 2 + index as u16, 10, 1))?;
        //}
        ////Scrollbar { theme: THEME, offset, length: self.patterns.len() }
            ////.render(term, &Space::new(x + 55, y + 2, 0, height as u16))?;
        //Ok(())
    //}
}

#[derive(Debug, Default)]
pub struct Electribe2PatternTUI(Electribe2Pattern);

impl TUI for Electribe2PatternTUI {
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        return Ok(())
        //return Layout::Item(Sizing::Min, &DebugBox { bg: Color::AnsiValue(100) })
            //.render(term, area);

        //let Area(Point(x, y), Size(w, h)) = area;
        //let Theme { bg, fg, hi } = THEME;
        ////let  = *space;
        //let title = String::from("Pattern details");
        ////Inset { theme: THEME, focused: true, title }
            ////.render(term, &Space(Point(x, y), Point(46, 30)))?;
        //term.queue(SetForegroundColor(fg))?
            //.queue(MoveTo(x +  1, y + 2))?.queue(Print(&self.pattern.name))?
            //.queue(MoveTo(x + 21, y + 2))?.queue(Print(&self.pattern.level))?
            //.queue(MoveTo(x +  1, y + 3))?.queue(Print(&self.pattern.bpm))?
            //.queue(MoveTo(x + 21, y + 3))?.queue(Print(&self.pattern.swing))?
            //.queue(MoveTo(x +  1, y + 4))?.queue(Print(&self.pattern.length))?
            //.queue(MoveTo(x + 21, y + 4))?.queue(Print(&self.pattern.beats))?
            //.queue(MoveTo(x +  1, y + 5))?.queue(Print(&self.pattern.key))?
            //.queue(MoveTo(x + 21, y + 5))?.queue(Print(&self.pattern.scale))?
            //.queue(MoveTo(x +  1, y + 6))?.queue(Print(&self.pattern.chord_set))?
            //.queue(MoveTo(x + 21, y + 6))?.queue(Print(&self.pattern.gate_arp))?
            //.queue(MoveTo(x +  1, y + 7))?.queue(Print(&self.pattern.mfx_type))?
            //.queue(MoveTo(x +  1, y + 8))?.queue(Print(&self.pattern.alt_13_14))?
            //.queue(MoveTo(x + 21, y + 8))?.queue(Print(&self.pattern.alt_15_16))?
            //.queue(MoveTo(x + 1, y + 10))?
            //.queue(SetAttribute(Attribute::Bold))?
            //.queue(Print("Part  Snd  Pit  Fil  Mod  IFX  Vol  Pan  MFX"))?
            //.queue(SetAttribute(Attribute::Reset))?
            //.queue(SetBackgroundColor(bg))?
            //.queue(SetForegroundColor(fg))?;
        //for index in 0..17 {
            //term.queue(MoveTo(x + 1, y + 12 + index))?
                //.queue(if let Some(part) = self.pattern.parts.get(index as usize) {
                    //Print(format!("{:>4}  {:>3}  {:>3}  {:>3}  {:>3}  {:>3}  {:>3}  {:>3}  {:>3}",
                        //index + 1,
                        //part.sample,
                        //part.pitch,
                        //part.filter_type,
                        //part.modulation_type,
                        //part.ifx_on,
                        //part.level,
                        //part.pan,
                        //part.mfx_on,
                    //))
                //} else {
                    //Print("".into())
                //})?;
        //}
        //Ok(())
    }
}

#[derive(Debug, Default)]
pub struct Electribe2SamplesTUI {
    pub focused: bool,
    pub file_list: FileList,
    pub bank: Option<Electribe2SampleBank>,
    pub sample_list: List<String>,
    pub sample: Spacer
}

impl TUI for Electribe2SamplesTUI {
    fn layout <'a> (&'a self) -> Thunk<'a> {
        let Self { focused, .. } = *self;
        Inset(1).around(if let Some(bank) = &self.bank {
            col(|add| {
                add(&self.sample_list);
                add(&self.sample);
            })
        } else {
            col(|add|{
                add(&self.file_list);
            })
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
        self.file_list.replace(entries);
    }
}
