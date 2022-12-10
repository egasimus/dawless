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

#[derive(Debug)]
pub struct Electribe2TUI {
    pub space:     Space,
    pub theme:    Theme,
    pub focused:  bool,
    pub patterns: Toggle<Label, Electribe2PatternsTUI>,
    pub samples:  Toggle<Label, Electribe2SamplesTUI>,
    pub section:  List<Electribe2TUIFeature>
}

#[derive(Debug, Default)]
pub enum Electribe2TUIFeature {
    #[default]
    Patterns,
    Samples
}

impl Electribe2TUI {

    pub fn new () -> Self {
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
            space:     Space::default(),
            theme:    Theme::default(),
            focused:  false,
            section,
            patterns,
            samples
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

    fn layout (&mut self, space: &Space) -> Result<Space> {
        let Space { x, y, w, h } = *space;

        let patterns_h = 1 + if self.patterns.toggle {
            self.patterns.open.entries.len() as u16
        } else {
            0
        };

        self.patterns.layout(&Space::new(
            x + if self.patterns.toggle { 0 } else { 1 },
            y + 2,
            0,
            patterns_h
        ));

        let samples_h = if self.samples.toggle {
            self.samples.open.space.h
        } else {
            0
        };

        self.space = Space::new(
            x,
            y,
            space.w.min(22
                .max(self.patterns.open.space.w + 2)
                .max(self.samples.open.space.w + 2)),
            space.h.min(6
                + patterns_h
                + samples_h)
        );

        self.samples.layout(&Space::new(
            x + if self.samples.toggle { 0 } else { 1 },
            y + 3 + patterns_h,
            0,
            samples_h
        ));

        Ok(self.space)
    }

    fn render (&self, term: &mut dyn Write) -> Result<()> {
        let Self { space, theme, focused, .. } = *self;
        Frame { space, theme, focused, title: "Electribe 2" }.render(term)?;
        self.patterns.render(term);
        self.samples.render(term);
        //self.menu.render(term)?;
        //(*self.feature()).render(term)?;
        Ok(())
    }

    fn focus (&mut self, focus: bool) -> bool {
        self.focused = focus;
        self.layout(&self.space.clone());
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

#[derive(Debug, Default)]
pub struct Electribe2PatternsTUI {
    pub space:     Space,
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

    fn layout (&mut self, space: &Space) -> Result<Space> {
        self.space = space.clone();
        self.entries.space = Space::new(space.x + 1, space.y + 1, 0, 0);
        Ok(self.space)
    }

    fn render (&self, term: &mut dyn Write) -> Result<()> {
        let Self { theme, focused, space: Space { x, y, w, .. }, offset, .. } = *self;

        if let Some(bank) = &self.bank {

            let space = Space::new(x, y, 58, 42);
            Frame { theme, focused, space, title: "Patterns:" }.render(term)?;

            let space = Space::new(x + 1, y + 2, 50, 0);
            let patterns = &bank.patterns;
            let selected = self.patterns.index;
            PatternList { theme, space, patterns, selected, offset }.render(term)?;

            let space = Space::new(x + 59, y, 0, 0);
            let pattern = bank.patterns.get(self.patterns.index).unwrap() ;
            Pattern { theme, space, pattern }.render(term)?;

        } else {

            let space = Space::new(x, y, 4 + self.max_len, 4 + self.entries.items.len() as u16);
            let title = "Select ALLPAT file (Esc to exit)";
            Frame { theme, focused, space, title }.render(term)?;

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
    pub space:     Space,
    pub theme:    Theme,
    pub patterns: &'a Vec<Electribe2Pattern>,
    pub selected: usize,
    pub offset:   usize
}

impl<'a> TUI for PatternList<'a> {
    fn render (&self, term: &mut dyn Write) -> Result<()> {
        let Self { theme, space, patterns, selected, offset, .. } = *self;
        let Theme { bg, fg, hi } = theme;
        let Space { x, y, w, .. } = space;
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

            Label {
                theme,
                focused: selected == index + offset,
                col: x,
                row: y + 2 + index as u16,
                text: if let Some(pattern) = patterns.get(index+offset as usize) {
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

        let space = Space::new(x + 55, y + 2, 0, height as u16);
        Scrollbar { theme, space, offset, length: patterns.len() }.render(term)?;

        Ok(())
    }
}

struct Pattern <'a> {
    space: Space,
    theme: Theme,
    pattern: &'a Electribe2Pattern
}

impl <'a> TUI for Pattern <'a> {
    fn render (&self, term: &mut dyn Write) -> Result<()> {
        let Self { space, theme, pattern, .. } = *self;
        let Theme { bg, fg, hi } = theme;
        let Space { x, y, .. } = space;
        let space  = Space { x, y, w: 46, h: 30 };
        let title = "Pattern details";
        Frame { theme, focused: true, space, title }.render(term)?;
        term.queue(SetForegroundColor(fg))?
            .queue(MoveTo(x +  1, y + 2))?.queue(Print(&pattern.name))?
            .queue(MoveTo(x + 21, y + 2))?.queue(Print(&pattern.level))?
            .queue(MoveTo(x +  1, y + 3))?.queue(Print(&pattern.bpm))?
            .queue(MoveTo(x + 21, y + 3))?.queue(Print(&pattern.swing))?
            .queue(MoveTo(x +  1, y + 4))?.queue(Print(&pattern.length))?
            .queue(MoveTo(x + 21, y + 4))?.queue(Print(&pattern.beats))?
            .queue(MoveTo(x +  1, y + 5))?.queue(Print(&pattern.key))?
            .queue(MoveTo(x + 21, y + 5))?.queue(Print(&pattern.scale))?
            .queue(MoveTo(x +  1, y + 6))?.queue(Print(&pattern.chord_set))?
            .queue(MoveTo(x + 21, y + 6))?.queue(Print(&pattern.gate_arp))?
            .queue(MoveTo(x +  1, y + 7))?.queue(Print(&pattern.mfx_type))?
            .queue(MoveTo(x +  1, y + 8))?.queue(Print(&pattern.alt_13_14))?
            .queue(MoveTo(x + 21, y + 8))?.queue(Print(&pattern.alt_15_16))?
            .queue(MoveTo(x + 1, y + 10))?
            .queue(SetAttribute(Attribute::Bold))?
            .queue(Print("Part  Snd  Pit  Fil  Mod  IFX  Vol  Pan  MFX"))?
            .queue(SetAttribute(Attribute::Reset))?
            .queue(SetBackgroundColor(bg))?
            .queue(SetForegroundColor(fg))?;
        for index in 0..17 {
            term.queue(MoveTo(x + 1, y + 12 + index))?
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

#[derive(Debug, Default)]
pub struct Electribe2SamplesTUI {
    pub space: Space,
    pub theme: Theme,
    pub bank:  Option<Electribe2SampleBank>,
}

impl TUI for Electribe2SamplesTUI {

    fn layout (&mut self, space: &Space) -> Result<Space> {
        self.space = space.sub(0, 0, 30, 32);
        Ok(self.space)
    }

    fn render (&self, term: &mut dyn Write) -> Result<()> {
        let Self { space, theme, .. } = *self;
        let Theme { bg, fg, .. } = theme;
        let Space { x, y, .. } = space;
        let space = Space { x, y, w: 30, h: 32 };
        Frame { theme, space, title: "Samples", focused: false }.render(term)?;
        for i in 1..24 {
            let text = format!("{:>3} Sample", i);
            Label { theme, focused: false, col: x + 1, row: y + 1 + i, text }.render(term)?;
        }
        Ok(())
    }

}
