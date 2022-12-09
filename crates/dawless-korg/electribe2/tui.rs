use std::io::{Result, Write};
use dawless_common::*;
use laterna;
use crossterm::{
    QueueableCommand,
    style::{
        Color, ResetColor, SetBackgroundColor, SetForegroundColor,
        Attribute, SetAttribute,
        Print
    },
    event::{Event, KeyEvent, KeyCode},
    cursor::MoveTo
};

use crate::electribe2::*;

pub struct Electribe2TUI {
    pub rect:     Rect,
    pub theme:    Theme,
    pub focused:  bool,
    pub patterns: Electribe2PatternsTUI,
    pub samples:  Electribe2SamplesTUI,
    pub menu:     List<usize>,
}

impl Electribe2TUI {

    pub fn new () -> Self {
        let mut menu = List::default();
        menu.add("Edit pattern bank".into(), 0)
            .add("Edit sample bank".into(),  1);
        Self {
            rect:     (0, 0, 0, 0),
            theme:    Theme::default(),
            focused:  false,
            patterns: Electribe2PatternsTUI::default(),
            samples:  Electribe2SamplesTUI::default(),
            menu
        }
    }

    fn feature (&self) -> &dyn TUI {
        match self.menu.get().unwrap() {
            0 => &self.patterns,
            1 => &self.samples,
            _ => unreachable!()
        }
    }

    fn feature_mut (&mut self) -> &mut dyn TUI {
        match self.menu.get().unwrap() {
            0 => &mut self.patterns,
            1 => &mut self.samples,
            _ => unreachable!()
        }
    }

}

impl TUI for Electribe2TUI {

    fn render (&self, term: &mut dyn Write) -> Result<()> {

        let (col1, row1, ..) = self.rect;

        Frame {
            theme: self.theme,
            rect:  (col1, row1, 21, 6),
            title: "Electribe 2", focused: self.focused
        }.render(term)?;

        term.queue(SetBackgroundColor(self.theme.bg))?
            .queue(SetForegroundColor(Color::White))?;

        self.menu.render(term)?;
        (*self.feature()).render(term)?;//, col1 + 22, row1, 0, 0)?;
        //self.render_pattern(&mut out, col1 + 48, row1)?;
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
        new.max_len = 20;
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
        self.patterns = List { items: patterns, ..List::default() };
    }

    fn update_listing (&mut self) {
        let (entries, max_len) = list_current_directory();
        self.entries = List { items: entries, ..List::default() };
        self.max_len = max_len as u16;
    }

}

impl TUI for Electribe2PatternsTUI {
    fn render (&self, term: &mut dyn Write) -> Result<()> {
        let theme = self.theme;
        let (col1, row1, cols, row) = self.rect;
        if let Some(bank) = &self.bank {

            Frame {
                theme,
                rect: (col1, row1, 58, 42),
                title: "Patterns:",
                focused: self.focused
            }(term)?;

            render_pattern_list(
                term, col1 + 1, row1 + 2, 50,
                &bank.patterns,
                self.patterns.index,
                self.offset
            )?;

            render_pattern(
                term, col1 + 59, row1,
                bank.patterns.get(self.patterns.index).unwrap()
            )?;

        } else {

            Frame {
                theme,
                rect: (col1, row1, 4 + self.max_len, 4 + self.entries.items.len() as u16),
                title: "Select ALLPAT file:",
                focused: self.focused
            }(term)?;

            render_directory_listing(
                term, col1 + 1, row1 + 2, self.max_len as usize,
                &self.entries.items,
                self.entries.index
            )?;

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

pub fn render_pattern_list (
    term: &mut dyn Write, col1: u16, row1: u16, pad: usize,
    patterns: &Vec<Electribe2Pattern>,
    selected: usize,
    offset:   usize
) -> Result<()> {
    let bg = Color::AnsiValue(232);
    let fg = Color::White;
    let hi = Color::Yellow;
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
        let row = if let Some(pattern) = patterns.get(index+offset as usize) {
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
        };
        term.queue(SetForegroundColor(if selected == index+offset { hi } else { fg }))?
            .queue(MoveTo(col1, row1 + 2 + index as u16))?
            .queue(Print(row))?;
    }

    render_scrollbar(term, col1 + 55, row1 + 2, patterns.len(), offset, height)?;

    Ok(())
}

pub fn render_pattern (
    term: &mut dyn Write, col1: u16, row1: u16,
    pattern: &Electribe2Pattern
) -> Result<()> {
    let bg = Color::AnsiValue(232);
    let fg = Color::White;
    let hi = Color::Yellow;
    render_frame(term, col1, row1, 46, 30, bg, Some((
        bg,
        hi,
        "Pattern details"
    )))?;
    term.queue(SetForegroundColor(fg))?
        .queue(MoveTo(col1 + 1, row1 + 2))?
        .queue(Print(&pattern.name))?
        .queue(MoveTo(col1 + 21, row1 + 2))?
        .queue(Print(&pattern.level))?
        .queue(MoveTo(col1 + 1, row1 + 3))?
        .queue(Print(&pattern.bpm))?
        .queue(MoveTo(col1 + 21, row1 + 3))?
        .queue(Print(&pattern.swing))?
        .queue(MoveTo(col1 + 1, row1 + 4))?
        .queue(Print(&pattern.length))?
        .queue(MoveTo(col1 + 21, row1 + 4))?
        .queue(Print(&pattern.beats))?
        .queue(MoveTo(col1 + 1, row1 + 5))?
        .queue(Print(&pattern.key))?
        .queue(MoveTo(col1 + 21, row1 + 5))?
        .queue(Print(&pattern.scale))?
        .queue(MoveTo(col1 + 1, row1 + 6))?
        .queue(Print(&pattern.chord_set))?
        .queue(MoveTo(col1 + 21, row1 + 6))?
        .queue(Print(&pattern.gate_arp))?
        .queue(MoveTo(col1 + 1, row1 + 7))?
        .queue(Print(&pattern.mfx_type))?
        .queue(MoveTo(col1 + 1, row1 + 8))?
        .queue(Print(&pattern.alt_13_14))?
        .queue(MoveTo(col1 + 21, row1 + 8))?
        .queue(Print(&pattern.alt_15_16))?
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

#[derive(Default)]
pub struct Electribe2SamplesTUI {
    pub rect:  Rect,
    pub theme: Theme
}

impl TUI for Electribe2SamplesTUI {
    fn render (&self, term: &mut dyn Write) -> Result<()> {
        let bg = Color::AnsiValue(232);
        let (col1, row1, ..) = self.rect;
        render_frame(term,
            col1, row1, 30, 32,
            bg, Some((bg, Color::Yellow, "Samples"))
        )?;
        for i in 1..24 {
            term.queue(SetBackgroundColor(bg))?
                .queue(SetForegroundColor(Color::White))?
                .queue(MoveTo(col1 + 1, row1 + 1 + i))?
                .queue(Print(format!("{:>3} Sample", i)))?;
        }
        Ok(())
    }
}
