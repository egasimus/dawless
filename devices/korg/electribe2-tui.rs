use crate::electribe2::*;
use thatsit::*;
use thatsit_fs::*;
use laterna;
use crossterm::{
    QueueableCommand,
    event::{Event, KeyEvent, KeyCode},
    cursor::MoveTo,
    style::{
        Color, ResetColor, SetBackgroundColor, SetForegroundColor,
        Attribute, SetAttribute,
        Print
    },
};

static THEME: Theme = Theme {
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
#[derive(Debug, Default)]
pub struct Electribe2PatternsTUI {
    pub focused:  bool,
    pub bank:     Option<Electribe2PatternBank>,
    pub entries:  List<(String, bool)>,
    pub patterns: List<String>,
    pub offset:   usize,
    pub max_len:  u16
}
#[derive(Debug, Default)]
pub struct Electribe2SamplesTUI {
    pub bank: Option<Electribe2SampleBank>,
}
#[derive(Debug)]
struct PatternList<'a> {
    pub patterns: &'a Vec<Electribe2Pattern>,
    pub selected: usize,
    pub offset:   usize
}
#[derive(Debug)]
struct Pattern <'a> {
    pattern: &'a Electribe2Pattern
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
            Electribe2SamplesTUI::default()
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
        Layout::Layers(Sizing::AUTO, vec![
            Layout::Item(Sizing::AUTO, &self.frame),
            Layout::Column(Sizing::AUTO, vec![
                Layout::Item(Sizing::AUTO, &self.patterns),
                Layout::Item(Sizing::AUTO, &self.samples)
            ])
        ])
    }
    fn focus (&mut self, focus: bool) -> bool {
        self.focused = focus;
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

impl Electribe2PatternsTUI {
    pub fn new () -> Self {
        let mut new = Self::default();
        new.update_listing();
        return new
    }
    pub fn import (&mut self, bank: &std::path::Path) {
        let data = crate::read(bank);
        let bank = Electribe2PatternBank::read(&data);
        self.bank = Some(bank);
        let patterns: Vec<(String, String)> = self.bank.as_ref().unwrap().patterns.iter()
            .map(|pattern|(pattern.name.clone(), pattern.name.clone()))
            .collect();
        self.patterns.replace(patterns);
    }
    fn update_listing (&mut self) {
        let (entries, max_len) = list_current_directory();
        self.entries.replace(entries);
        self.max_len = u16::max(max_len as u16, 20);
    }
}

impl TUI for Electribe2PatternsTUI {
    fn layout (&self) -> Layout {
        self.entries.layout()
    }
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        let Self { focused, offset, .. } = *self;
        let Area(Point(x, y), Size(w, _)) = area;
        if let Some(bank) = &self.bank {
            Frame { theme: THEME, focused, title: "Patterns:".into() }
                .render(term, area)?; //&Space::new(x, y, 58, 42))?;

            let patterns = &bank.patterns;
            let selected = self.patterns.index;
            PatternList { patterns, selected, offset }
                .render(term, area)?; //&Space::new(x + 1, y + 2, 50, 0))?;

            let pattern = bank.patterns.get(self.patterns.index).unwrap() ;
            Pattern { pattern }
                .render(term, area)?; //&Space::new(x + 59, y, 0, 0))?;
        } else {
            //let space = Space::new(x, y, 4 + self.max_len, 4 + self.entries.items.len() as u16);
            let title = "Select ALLPAT file (Esc to exit)".into();
            Frame { theme: THEME, focused, title }
                .render(term, area)?;
            FileList(&self.entries)
                .render(term, area)?;
        }
        Ok(())
    }
    fn focus (&mut self, focus: bool) -> bool {
        self.focused = focus;
        true
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        if let Some(bank) = &self.bank {
            if self.patterns.handle(event)? {
                self.offset = handle_scroll(
                    self.patterns.items.len(), self.patterns.index, 36, self.offset
                );
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            if let Event::Key(KeyEvent { code: KeyCode::Enter, .. }) = event {
                let (path, is_dir) = &self.entries.items.get(self.entries.index).unwrap().1;
                if *is_dir {
                    std::env::set_current_dir(path)?;
                    self.update_listing();
                } else {
                    let path = std::path::PathBuf::from(path);
                    self.import(&path);
                }
                Ok(true)
            } else {
                self.entries.handle(event)
            }
        }
    }
}


impl<'a> TUI for PatternList<'a> {
    fn render (&self, term: &mut dyn Write, Area(Point(x, y), Size(w, h)): Area) -> Result<()> {
        let Self { offset, .. } = *self;
        let Theme { bg, fg, hi } = THEME;
        term.queue(SetBackgroundColor(bg))?
            .queue(SetForegroundColor(fg))?
            .queue(SetAttribute(Attribute::Bold))?
            .queue(MoveTo(x, y))?
            .queue(Print(format!("{:>3}  {:<16}  {:<5} {:>3}  {:>3}  {:>3}  {:>3}",
                "#", "Name", "BPM", "Length", "Beats", "Key", "Scale"
            )))?
            .queue(SetAttribute(Attribute::Reset))?
            .queue(SetBackgroundColor(bg))?;
        let height = 36;
        for index in 0..0+height {
            let index_offset = index + self.offset;
            let focused = self.selected == index_offset;
            let text = if let Some(pattern) = self.patterns.get(index_offset as usize) {
                format!("{:>3}  {:<16} {:>5.1}   {:>3}    {:>3}    {:>3}   {:>3}",
                    index + self.offset + 1,
                    pattern.name.trim(),
                    pattern.bpm,
                    pattern.length,
                    pattern.beats,
                    pattern.key,
                    pattern.scale,
                )
            } else {
                "".into()
            };
            //Label { theme: THEME, focused, text }
                //.render(term, &Space::new(x, y + 2 + index as u16, 10, 1))?;
        }
        //Scrollbar { theme: THEME, offset, length: self.patterns.len() }
            //.render(term, &Space::new(x + 55, y + 2, 0, height as u16))?;
        Ok(())
    }
}

impl <'a> TUI for Pattern <'a> {
    fn render (&self, term: &mut dyn Write, Area(Point(x, y), _): Area) -> Result<()> {
        let Theme { bg, fg, hi } = THEME;
        //let  = *space;
        let title = String::from("Pattern details");
        //Frame { theme: THEME, focused: true, title }
            //.render(term, &Space(Point(x, y), Point(46, 30)))?;
        term.queue(SetForegroundColor(fg))?
            .queue(MoveTo(x +  1, y + 2))?.queue(Print(&self.pattern.name))?
            .queue(MoveTo(x + 21, y + 2))?.queue(Print(&self.pattern.level))?
            .queue(MoveTo(x +  1, y + 3))?.queue(Print(&self.pattern.bpm))?
            .queue(MoveTo(x + 21, y + 3))?.queue(Print(&self.pattern.swing))?
            .queue(MoveTo(x +  1, y + 4))?.queue(Print(&self.pattern.length))?
            .queue(MoveTo(x + 21, y + 4))?.queue(Print(&self.pattern.beats))?
            .queue(MoveTo(x +  1, y + 5))?.queue(Print(&self.pattern.key))?
            .queue(MoveTo(x + 21, y + 5))?.queue(Print(&self.pattern.scale))?
            .queue(MoveTo(x +  1, y + 6))?.queue(Print(&self.pattern.chord_set))?
            .queue(MoveTo(x + 21, y + 6))?.queue(Print(&self.pattern.gate_arp))?
            .queue(MoveTo(x +  1, y + 7))?.queue(Print(&self.pattern.mfx_type))?
            .queue(MoveTo(x +  1, y + 8))?.queue(Print(&self.pattern.alt_13_14))?
            .queue(MoveTo(x + 21, y + 8))?.queue(Print(&self.pattern.alt_15_16))?
            .queue(MoveTo(x + 1, y + 10))?
            .queue(SetAttribute(Attribute::Bold))?
            .queue(Print("Part  Snd  Pit  Fil  Mod  IFX  Vol  Pan  MFX"))?
            .queue(SetAttribute(Attribute::Reset))?
            .queue(SetBackgroundColor(bg))?
            .queue(SetForegroundColor(fg))?;
        for index in 0..17 {
            term.queue(MoveTo(x + 1, y + 12 + index))?
                .queue(if let Some(part) = self.pattern.parts.get(index as usize) {
                    Print(format!("{:>4}  {:>3}  {:>3}  {:>3}  {:>3}  {:>3}  {:>3}  {:>3}  {:>3}",
                        index + 1,
                        part.sample,
                        part.pitch,
                        part.filter_type,
                        part.modulation_type,
                        part.ifx_on,
                        part.level,
                        part.pan,
                        part.mfx_on,
                    ))
                } else {
                    Print("".into())
                })?;
        }
        Ok(())
    }
}

impl TUI for Electribe2SamplesTUI {
    fn layout (&self) -> Layout {
        Layout::Item(Sizing::Fixed(Size(30, 28)), &Blank {})
    }
}
