use crate::electribe2::*;
use crate::*;
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
use laterna;

#[derive(Debug, Default)]
pub struct Electribe2UI(TabsLeft<Box<dyn Widget>>);

impl Electribe2UI {
    pub fn new () -> Self {
        let mut selector = TabsLeft::<Box<dyn Render>>::default();
        selector.add("Edit patterns".into(), Box::new(Electribe2PatternsUI::new()));
        selector.add("Edit samples".into(),  Box::new(Electribe2SamplesUI::new()));
        selector.pages.select_next();
        Self(selector)
    }
}

impl Render for Electribe2UI {
    impl_render!(self, out, area => self.0.render(out, area));
}

impl Handle for Electribe2UI {
    impl_handle!(self, event => self.0.handle(event));
}

/// UI for editing a Korg Electribe 2 pattern bank
#[derive(Debug)]
pub struct Electribe2PatternsUI<'a> {
    pub label:     String,
    /// File explorer for selecting a pattern bank
    pub file_list: FileList<'a>,
    /// The currently selected pattern bank
    pub bank:      Option<Electribe2PatternBank>,
    /// Selector for editing an individual pattern
    pub patterns:  TabsLeft<Electribe2PatternUI>,
    /// FIXME: Scroll offset. Need to implement generic scrollable
    pub offset:    usize,
}

impl<'a> Electribe2PatternsUI<'a> {
    const SELECT_PATTERN_BANK: &'static str = " Select pattern bank: ";
    pub fn new () -> Self {
        let mut file_list = FileList::default();
        file_list.update();
        Self {
            label: Self::SELECT_PATTERN_BANK.into(),
            bank: None,
            patterns: TabsLeft::default(),
            offset: 0,
            file_list,
        }
    }
    pub fn update (&mut self) { self.file_list.update(); }
    pub fn import (&mut self, bank: &std::path::Path) {
        let data = crate::read(bank);
        let bank = Electribe2PatternBank::read(&data);
        self.bank = Some(bank);
        self.patterns.pages.replace(
            self.bank.as_ref().unwrap().patterns.iter().enumerate()
            .map(|(index,pattern)|(format!(
                "{:>3}  {:<16} {:>5.1}   {:>3}    {:>3}    {:>3}   {:>3}",
                index + self.offset + 1,
                pattern.name.trim(),
                pattern.bpm,
                pattern.length,
                pattern.beats,
                pattern.key,
                pattern.scale,
            ), Electribe2PatternUI::new(pattern)))
            .collect::<Vec<_>>());
    }
}

impl<'a> Render for Electribe2PatternsUI<'a> {
    impl_render!(self, out, area => {
        let Self { offset, bank, .. } = self;
        if let Some(bank) = &bank {
            Border(InsetTall, self.patterns).render(out, area)
        } else {
            Border(InsetTall, Stacked::y(|row|{
                row(&self.label);
                row(());
                row(&self.file_list);
            })).render(out, area)
        }
    });
}

impl<'a> Handle for Electribe2PatternsUI<'a> {
    impl_handle!(self, event => {
        Ok(if let Some(bank) = &self.bank {
            if *event == key!(Ctrl-Up) {
                if let Some(index) = self.patterns.pages.selected() {
                    if index > 0 {
                        //self.patterns.tabs.items.items.swap(index, index-1);
                        self.patterns.pages.items_mut().swap(index, index-1);
                    }
                }
                true
            } else if *event == key!(Ctrl-Down) {
                if let Some(index) = self.patterns.pages.selected() {
                    if index < self.patterns.len() - 1 {
                        //self.patterns.tabs.items.items.swap(index, index+1);
                        self.patterns.pages.items_mut().swap(index, index+1);
                    }
                }
                true
            } else if self.patterns.handle(event)? {
                //let len     = self.patterns.len();
                //let index   = self.patterns.index();
                //self.offset = handle_scroll(len, index, 36, self.offset);
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
    });
}

#[derive(Debug, Default)]
pub struct Electribe2PatternList<'a>(FocusStack<'a>);

impl<'a> Electribe2PatternList<'a> {
    pub fn len (&self) -> usize { self.0.len() }
    //pub fn index (&self) -> usize { self.0.index() }
}

impl<'a> Render for Electribe2PatternList<'a> {
    impl_render!(self, out, area => self.0.render(out, area));
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
pub struct Electribe2PatternUI {
    pattern: Electribe2Pattern,
    name:    String,
    level:   String,
    bpm:     String,
    length:  String,
    beats:   String,
}

impl Electribe2PatternUI {
    fn new (pattern: &Electribe2Pattern) -> Self {
        Self {
            pattern: pattern.clone(),
            name:    format!("Pattern name: {}", pattern.name),
            level:   format!("  Level: {}",      pattern.level),
            bpm:     format!("BPM: {}",          pattern.bpm),
            length:  format!("  Length: {}",     pattern.length),
            beats:   format!("  Beats: {}",      pattern.beats),
        }
    }
}

impl Render for Electribe2PatternUI {
    impl_render!(self, out, area => {
        Border(InsetTall, Stacked::x(|column|{
            column(Stacked::y(|row|{row(());row(&self.name);row(&self.level);}));
            column(Stacked::y(|row|{row(());row(&self.bpm);row(&self.length);row(&self.beats)}));
        })).render(out, area)
    });
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
pub struct Electribe2SamplesUI<'a> {
    pub focused: bool,
    pub file_list: FileList<'a>,
    pub bank: Option<Electribe2SampleBank>,
    pub sample_list: (),//List<String>,
    pub sample: ()
}

impl<'a> Render for Electribe2SamplesUI<'a> {
    impl_render!(self, out, area => {
        let Self { focused, .. } = *self;
        Border(InsetTall, Stacked::y(|row|{
            if self.bank.is_some() {
                row(&self.sample_list);
                row(&self.sample);
            } else {
                row(&self.file_list);
            }
        })).render(out, area)
    });
}

impl<'a> Electribe2SamplesUI<'a> {
    pub fn new () -> Self { let mut new = Self::default(); new.update(); return new }
    fn update (&mut self) {
        let (entries, _) = list_current_directory();
        self.file_list.replace(entries);
    }
}
