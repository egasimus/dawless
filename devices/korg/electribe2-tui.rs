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

#[derive(Debug, Default)]
pub struct Electribe2TUI(TabbedVertical<Box<dyn TUI>>);

impl Electribe2TUI {
    pub fn new () -> Self {
        let mut selector = TabbedVertical::<Box<dyn TUI>>::default();
        selector.add("Load pattern bank...", Box::new(Electribe2PatternsTUI::new()));
        selector.add("Load sample bank...",  Box::new(Electribe2SamplesTUI::new()));
        selector.tabs.items.items[0].focus(true);
        Self(selector)
    }
}

impl TUI for Electribe2TUI {
    fn focused (&self) -> bool { self.0.focused() }
    fn focus (&mut self, focus: bool) -> bool { self.0.focus(focus) }
    fn layout <'a> (&'a self, max: Size) -> Result<Thunk<'a>> { self.0.layout(max) }
    fn handle (&mut self, event: &Event) -> Result<bool> { self.0.handle(event) }
}

/// UI for editing a Korg Electribe 2 pattern bank
#[derive(Debug)]
pub struct Electribe2PatternsTUI {
    pub label:     Foreground<Text>,
    /// File explorer for selecting a pattern bank
    pub file_list: FileList,
    /// The currently selected pattern bank
    pub bank:      Option<Electribe2PatternBank>,
    /// Selector for editing an individual pattern
    pub patterns:  TabbedVertical<Electribe2PatternTUI>,
    /// FIXME: Scroll offset. Need to implement generic scrollable
    pub offset:    usize,
}

impl Electribe2PatternsTUI {
    const SELECT_PATTERN_BANK: &'static str = " Select pattern bank:";
    pub fn new () -> Self {
        let mut file_list = FileList::default();
        file_list.update();
        Self {
            label: Text(Self::SELECT_PATTERN_BANK.into()).fg(Color::White),
            bank: None,
            patterns: TabbedVertical::default(),
            offset: 0,
            file_list,
        }
    }
    pub fn update (&mut self) { self.file_list.update(); }
    pub fn import (&mut self, bank: &std::path::Path) {
        let data = crate::read(bank);
        let bank = Electribe2PatternBank::read(&data);
        self.bank = Some(bank);
        self.patterns.tabs.replace(self.bank.as_ref().unwrap().patterns.iter().enumerate()
            .map(|(index, pattern)|Button::new(format!(
                "{:>3}  {:<16} {:>5.1}   {:>3}    {:>3}    {:>3}   {:>3}",
                index + self.offset + 1,
                pattern.name.trim(),
                pattern.bpm,
                pattern.length,
                pattern.beats,
                pattern.key,
                pattern.scale,
            ), None))
            .collect::<Vec<_>>());
        self.patterns.pages.replace(self.bank.as_ref().unwrap().patterns.iter()
            .map(|pattern|Electribe2PatternTUI::new(pattern))
            .collect::<Vec<_>>());
    }
}

impl TUI for Electribe2PatternsTUI {
    fn layout <'a> (&'a self, max: Size) -> Result<Thunk<'a>> {
        let Self { offset, bank, .. } = self;
        Ok(Inset(0).around(if let Some(bank) = &bank {
            (&self.patterns).into()
        } else {
            pad(Size(1, 1), col(|add|{ add(&self.label); add(SPACE); add(&self.file_list); }))
        }))
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        Ok(if let Some(bank) = &self.bank {
            if self.patterns.handle(event)? {
                let len     = self.patterns.len();
                let index   = self.patterns.index();
                self.offset = handle_scroll(len, index, 36, self.offset);
                true
            } else {
                false
            }
        } else {
            self.file_list.handle(event)? || if_key!(event => Enter => {
                let FileEntry { path, is_dir, .. } = self.file_list.selected();
                if *is_dir {
                    std::env::set_current_dir(path)?;
                    self.update();
                } else {
                    self.import(&std::path::PathBuf::from(path));
                }
                true
            })
        })
    }
}

#[derive(Debug, Default)]
pub struct Electribe2PatternList(FocusColumn<Electribe2PatternTUI>);

impl Electribe2PatternList {
    pub fn len (&self) -> usize { self.0.len() }
    pub fn index (&self) -> usize { self.0.index() }
}

impl TUI for Electribe2PatternList {
    fn layout <'a> (&'a self, max: Size) -> Result<Thunk<'a>> {
        self.0.layout(max)
    }
    //fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        //return self.0.render(term, area);
        //return Layout::Item(
            //Sizing::Range(self.min_size(), self.max()), &self.0
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
pub struct Electribe2PatternTUI {
    pattern: Electribe2Pattern,
    name:    Text,
    level:   Text,
    bpm:     Text,
    length:  Text,
    beats:   Text,
}

impl Electribe2PatternTUI {
    fn new (pattern: &Electribe2Pattern) -> Self {
        Self {
            pattern: pattern.clone(),
            name:    Text(format!("Pattern name: {}", pattern.name)),
            level:   Text(format!("  Level: {}",      pattern.level)),
            bpm:     Text(format!("BPM: {}",          pattern.bpm)),
            length:  Text(format!("  Length: {}",     pattern.length)),
            beats:   Text(format!("  Beats: {}",      pattern.beats)),
        }
    }
}

impl TUI for Electribe2PatternTUI {
    fn layout <'a> (&'a self, max: Size) -> Result<Thunk<'a>> {
        Ok(Inset(2).around(col(|add|{
            add(row(|add|{add(SPACE);add(&self.name);add(&self.level);}));
            add(row(|add|{add(SPACE);add(&self.bpm);add(&self.length);add(&self.beats)}));
        })))
    }
    //fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        //return Ok(())
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
    //}
}

#[derive(Debug, Default)]
pub struct Electribe2SamplesTUI {
    pub focused: bool,
    pub file_list: FileList,
    pub bank: Option<Electribe2SampleBank>,
    pub sample_list: Spacer,//List<String>,
    pub sample: Spacer
}

impl TUI for Electribe2SamplesTUI {
    fn layout <'a> (&'a self, max: Size) -> Result<Thunk<'a>> {
        let Self { focused, .. } = *self;
        Ok(Inset(1).around(if self.bank.is_some() {
            col(|add| { add(&self.sample_list); add(&self.sample); })
        } else {
            col(|add| { add(&self.file_list); })
        }))
    }
}

impl Electribe2SamplesTUI {
    pub fn new () -> Self { let mut new = Self::default(); new.update(); return new }
    fn update (&mut self) {
        let (entries, _) = list_current_directory();
        self.file_list.replace(entries);
    }
}
