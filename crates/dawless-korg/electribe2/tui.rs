use crate::electribe2::*;

use dawless_common::*;
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

pub struct Electribe2TUI {
    pub rect:     Rect,
    pub theme:    Theme,
    pub focused:  bool,
    pub patterns: Electribe2PatternsTUI,
    pub samples:  Electribe2SamplesTUI,
    pub menu:     List<Electribe2TUIFeature>,
}

#[derive(Default)]
pub enum Electribe2TUIFeature {
    #[default]
    Patterns,
    Samples
}

impl Electribe2TUI {

    pub fn new () -> Self {
        let mut menu = List::default();
        menu.add("Edit pattern bank".into(), Electribe2TUIFeature::Patterns)
            .add("Edit sample bank".into(),  Electribe2TUIFeature::Samples);
        Self {
            rect:     (0, 0, 0, 0),
            theme:    Theme::default(),
            focused:  false,
            patterns: Electribe2PatternsTUI::new(),
            samples:  Electribe2SamplesTUI::default(),
            menu
        }
    }

    fn feature (&self) -> &dyn TUI {
        match self.menu.get().unwrap() {
            Electribe2TUIFeature::Patterns => &self.patterns,
            Electribe2TUIFeature::Samples  => &self.samples,
        }
    }

    fn feature_mut (&mut self) -> &mut dyn TUI {
        match self.menu.get().unwrap() {
            Electribe2TUIFeature::Patterns => &mut self.patterns,
            Electribe2TUIFeature::Samples  => &mut self.samples,
        }
    }

}

impl TUI for Electribe2TUI {

    fn layout (&mut self, col1: u16, row1: u16, cols: u16, rows: u16) -> Result<()> {
        self.rect = (col1, row1, 21, 5);
        self.menu.layout(col1, row1 + 2, 17, 0);
        self.patterns.layout(col1 + 22, row1, 0, 0);
        self.samples.rect = (self.rect.0 + 22, self.rect.1, 0, 0);
        Ok(())
    }

    fn render (&self, term: &mut dyn Write) -> Result<()> {
        let Self { rect, theme, focused, .. } = *self;
        Frame { rect, theme, focused, title: "Electribe 2" }.render(term)?;
        term.queue(SetBackgroundColor(self.theme.bg))?
            .queue(SetForegroundColor(Color::White))?;
        self.menu.render(term)?;
        (*self.feature()).render(term)?;
        Ok(())
    }

    fn focus (&mut self, focus: bool) -> bool {
        self.focused = focus;
        true
    }

    fn handle (&mut self, event: &Event) -> Result<bool> {
        if !self.focused {
            if self.feature_mut().handle(&event)? {
                return Ok(true)
            }
        }
        if self.menu.handle(event)? {
            return Ok(true)
        }
        handle_menu_focus!(event, self, self.feature_mut(), self.focused)
    }

}

#[derive(Default)]
pub struct Electribe2PatternsTUI {
    pub rect:     Rect,
    pub theme:    Theme,
    pub focused:  bool,
    pub bank:     Option<Electribe2PatternBank>,
    pub entries:  List<(String, bool)>,
    pub patterns: List<String>,
    pub offset:   usize,
    pub max_len:  u16
}

impl Electribe2PatternsTUI {

    pub fn new () -> Self {
        let mut new = Self::default();
        new.update_listing();
        return new
    }

    pub fn import (&mut self, bank: &std::path::Path) {
        let data = dawless_common::read(bank);
        let bank = Electribe2PatternBank::read(&data);
        self.bank = Some(bank);
        let patterns: Vec<(String, String)> = self.bank.as_ref().unwrap().patterns.iter()
            .map(|pattern|(pattern.name.clone(), pattern.name.clone()))
            .collect();
        self.patterns.items = patterns;
    }

    fn update_listing (&mut self) {
        let (entries, max_len) = list_current_directory();
        self.entries.items = entries;
        self.max_len = u16::max(max_len as u16, 20);
    }

}

impl TUI for Electribe2PatternsTUI {

    fn layout (&mut self, x: u16, y: u16, w: u16, h: u16) -> Result<()> {
        self.rect = (x, y, w, h);
        self.entries.rect = (x + 1, y + 1, 0, 0);
        Ok(())
    }

    fn render (&self, term: &mut dyn Write) -> Result<()> {
        let Self { theme, focused, rect, offset, .. } = *self;
        let (x, y, cols, _) = rect;
        if let Some(bank) = &self.bank {

            let rect = (x, y, 58, 42);
            Frame { theme, focused, rect, title: "Patterns:" }.render(term)?;

            let rect = (x + 1, y + 2, 50, 0);
            let patterns = &bank.patterns;
            let selected = self.patterns.index;
            PatternList { theme, rect, patterns, selected, offset }.render(term)?;

            let rect = (x + 59, y, 0, 0);
            let pattern = bank.patterns.get(self.patterns.index).unwrap() ;
            Pattern { theme, rect, pattern }.render(term)?;

        } else {

            let rect = (x, y, 4 + self.max_len, 4 + self.entries.items.len() as u16);
            Frame { theme, focused, rect, title: "Select ALLPAT file:" }.render(term)?;

            FileList(&self.entries).render(term)?;

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

struct PatternList<'a> {
    pub rect:     Rect,
    pub theme:    Theme,
    pub patterns: &'a Vec<Electribe2Pattern>,
    pub selected: usize,
    pub offset:   usize
}

impl<'a> TUI for PatternList<'a> {
    fn render (&self, term: &mut dyn Write) -> Result<()> {
        let Self { theme, rect, patterns, selected, offset, .. } = *self;
        let Theme { bg, fg, hi } = theme;
        let (col1, row1, pad, ..) = rect;
        term.queue(SetBackgroundColor(bg))?
            .queue(SetForegroundColor(fg))?
            .queue(SetAttribute(Attribute::Bold))?
            .queue(MoveTo(col1, row1))?
            .queue(Print(format!("{:>3}  {:<16}  {:<5} {:>3}  {:>3}  {:>3}  {:>3}",
                "#", "Name", "BPM", "Length", "Beats", "Key", "Scale"
            )))?
            .queue(SetAttribute(Attribute::Reset))?
            .queue(SetBackgroundColor(bg))?;

        let height = 36;

        for index in 0..0+height {

            Label {
                bg,
                fg: if selected == index+offset { hi } else { fg },
                col: col1,
                row: row1 + 2 + index as u16,
                text: &if let Some(pattern) = patterns.get(index+offset as usize) {
                    format!("{:>3}  {:<16} {:>5.1}   {:>3}    {:>3}    {:>3}   {:>3}",
                        index + offset + 1,
                        pattern.name.trim(),
                        pattern.bpm,
                        pattern.length,
                        pattern.beats,
                        pattern.key,
                        pattern.scale,
                    )
                } else {
                    "".into()
                }
            }.render(term)?;

        }

        let rect = (col1 + 55, row1 + 2, 0, height as u16);
        Scrollbar { theme, rect, offset, length: patterns.len() }.render(term)?;

        Ok(())
    }
}

struct Pattern <'a> {
    rect: Rect,
    theme: Theme,
    pattern: &'a Electribe2Pattern
}

impl <'a> TUI for Pattern <'a> {
    fn render (&self, term: &mut dyn Write) -> Result<()> {
        let Self { rect, theme, pattern, .. } = *self;
        let Theme { bg, fg, hi } = theme;
        let (col1, row1, ..) = rect;
        Frame {
            theme, focused: true, rect: (col1, row1, 46, 30), title: "Pattern details"
        }.render(term)?;
        term.queue(SetForegroundColor(fg))?
            .queue(MoveTo(col1 +  1, row1 + 2))?.queue(Print(&pattern.name))?
            .queue(MoveTo(col1 + 21, row1 + 2))?.queue(Print(&pattern.level))?
            .queue(MoveTo(col1 +  1, row1 + 3))?.queue(Print(&pattern.bpm))?
            .queue(MoveTo(col1 + 21, row1 + 3))?.queue(Print(&pattern.swing))?
            .queue(MoveTo(col1 +  1, row1 + 4))?.queue(Print(&pattern.length))?
            .queue(MoveTo(col1 + 21, row1 + 4))?.queue(Print(&pattern.beats))?
            .queue(MoveTo(col1 +  1, row1 + 5))?.queue(Print(&pattern.key))?
            .queue(MoveTo(col1 + 21, row1 + 5))?.queue(Print(&pattern.scale))?
            .queue(MoveTo(col1 +  1, row1 + 6))?.queue(Print(&pattern.chord_set))?
            .queue(MoveTo(col1 + 21, row1 + 6))?.queue(Print(&pattern.gate_arp))?
            .queue(MoveTo(col1 +  1, row1 + 7))?.queue(Print(&pattern.mfx_type))?
            .queue(MoveTo(col1 +  1, row1 + 8))?.queue(Print(&pattern.alt_13_14))?
            .queue(MoveTo(col1 + 21, row1 + 8))?.queue(Print(&pattern.alt_15_16))?
            .queue(MoveTo(col1 + 1, row1 + 10))?
            .queue(SetAttribute(Attribute::Bold))?
            .queue(Print("Part  Snd  Pit  Fil  Mod  IFX  Vol  Pan  MFX"))?
            .queue(SetAttribute(Attribute::Reset))?
            .queue(SetBackgroundColor(bg))?
            .queue(SetForegroundColor(fg))?;
        for index in 0..17 {
            term.queue(MoveTo(col1 + 1, row1 + 12 + index))?
                .queue(if let Some(part) = pattern.parts.get(index as usize) {
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

#[derive(Default)]
pub struct Electribe2SamplesTUI {
    pub rect:  Rect,
    pub theme: Theme
}

impl TUI for Electribe2SamplesTUI {

    fn layout (&mut self, col1: u16, row1: u16, cols: u16, rows: u16) -> Result<()> {
        self.rect = (col1, row1, 30, 32);
        Ok(())
    }

    fn render (&self, term: &mut dyn Write) -> Result<()> {
        let Self { rect, theme, .. } = *self;
        let Theme { bg, fg, .. } = theme;
        let (x, y, ..) = rect;
        Frame { theme, rect: (x, y, 30, 32), title: "Samples", focused: false }.render(term)?;
        for i in 1..24 {
            Label { bg, fg, col: x + 1, row: y + 1 + i, text: &format!("{:>3} Sample", i) }.render(term)?;
        }
        Ok(())
    }

}
