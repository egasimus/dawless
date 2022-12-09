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
    focused:  bool,
    features: Menu<Box<dyn TUI>>,
}

impl Electribe2TUI {
    pub fn new () -> Self {
        Self {
            focused: false,
            features: Menu::new(vec![
                ("Edit pattern bank".into(), Box::new(Electribe2PatternsTUI::new()) as Box<dyn TUI>),
                ("Edit sample bank".into(),  Box::new(Electribe2SamplesTUI::new())),
            ])
        }
    }
    fn feature <'a> (&'a mut self) -> &'a mut dyn TUI {
        &mut **self.features.get_mut().unwrap()
    }
}

impl TUI for Electribe2TUI {

    fn render (
        &self, term: &mut dyn Write, col1: u16, row1: u16, cols: u16, rows: u16
    ) -> Result<()> {
        let bg = Color::AnsiValue(232);
        let fg = Color::White;
        let hi = Color::Yellow;
        render_frame(term, col1, row1, 21, 6, bg, Some((
            if self.focused { hi } else { bg },
            if self.focused { bg } else { hi },
            "Electribe 2"
        )))?;
        term.queue(SetBackgroundColor(bg))?
            .queue(SetForegroundColor(Color::White))?;
        self.features.render(term, col1, row1 + 2, 17, 0)?;
        if let Some(feature) = self.features.get() {
            (*feature).render(term, col1 + 22, row1, 0, 0)?;
        }
        //self.render_pattern(&mut out, col1 + 48, row1)?;
        Ok(())
    }

    fn focus (&mut self, focus: bool) -> bool {
        self.focused = focus;
        true
    }

    fn handle (&mut self, event: &Event) -> Result<bool> {
        if !self.focused {
            if self.feature().handle(&event)? {
                return Ok(true)
            }
        }
        if self.features.handle(event)? {
            return Ok(true)
        }
        handle_menu_focus!(event, self, self.feature(), self.focused)
    }
}

struct Electribe2PatternsTUI {
    focused:  bool,
    bank:     Option<Electribe2PatternBank>,
    entries:  Menu<(String, bool)>,
    patterns: Menu<String>,
    offset:   usize,
    max_len:  u16
}

impl Electribe2PatternsTUI {
    pub fn new () -> Self {
        let mut this = Self {
            focused:  false,
            bank:     None,
            entries:  Menu::new(vec![]),
            patterns: Menu::new(vec![]),
            offset:   0,
            max_len:  20
        };
        this.update_listing();
        return this
    }
    pub fn import (&mut self, bank: &std::path::Path) {
        let data = dawless_common::read(bank);
        let bank = Electribe2PatternBank::read(&data);
        self.bank = Some(bank);
        let patterns: Vec<(String, String)> = self.bank.as_ref().unwrap().patterns.iter()
            .map(|pattern|(pattern.name.clone(), pattern.name.clone()))
            .collect();
        self.patterns = Menu::new(patterns);
    }
    fn render_pattern (&self, term: &mut dyn Write, col1: u16, row1: u16) -> Result<()> {
        let bg = Color::AnsiValue(232);
        render_frame(term,
            col1+1, row1, 66, 32,
            bg, Some((bg, Color::Yellow, "Pattern 23 Part 5"))
        )?;
        laterna::demo(term, col1)?;
        Ok(())
    }
    fn update_listing (&mut self) {
        let (entries, max_len) = list_current_directory();
        self.entries = Menu::new(entries);
        self.max_len = max_len as u16;
    }
}

impl TUI for Electribe2PatternsTUI {
    fn render (&self, term: &mut dyn Write, col1: u16, row1: u16, cols: u16, rows: u16) -> Result<()> {
        let bg = Color::AnsiValue(232);
        let fg = Color::White;
        let hi = Color::Yellow;
        if let Some(bank) = &self.bank {

            render_frame(
                term, col1, row1, 58, 42,
                bg, Some((
                    if self.focused { hi } else { bg },
                    if self.focused { bg } else { hi },
                    "Patterns:"
                ))
            )?;

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

            render_frame(
                term, col1, row1, 4 + self.max_len, 4 + self.entries.items.len() as u16,
                bg, Some((
                    if self.focused { hi } else { bg },
                    if self.focused { bg } else { hi },
                    "Select ALLPAT file:"
                ))
            )?;

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

struct Electribe2SamplesTUI {}

impl Electribe2SamplesTUI {
    pub fn new () -> Self {
        Self {}
    }
}

impl TUI for Electribe2SamplesTUI {
    fn render (
        &self, term: &mut dyn Write, col1: u16, row1: u16, cols: u16, rows: u16
    ) -> Result<()> {
        let bg = Color::AnsiValue(232);
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
