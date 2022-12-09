use std::io::{Result, Write};
use dawless_common::*;
use laterna;
use crossterm::{
    queue,
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
    fn render (&self, col1: u16, row1: u16, cols: u16, rows: u16) -> Result<()> {
        let out = &mut std::io::stdout();
        let bg = Color::AnsiValue(232);
        let fg = Color::White;
        let hi = Color::Yellow;
        render_frame(out, col1, row1, 21, 6, bg, Some((
            if self.focused { hi } else { bg },
            if self.focused { bg } else { hi },
            "Electribe 2"
        )))?;
        queue!(out,
            SetBackgroundColor(bg),
            SetForegroundColor(Color::White)
        )?;
        self.features.render(col1, row1 + 2, 17, 0)?;
        if let Some(feature) = self.features.get() {
            (*feature).render(col1 + 22, row1, 0, 0)?;
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
    fn render_pattern <W: Write> (&self, out: &mut W, col1: u16, row1: u16) -> Result<()> {
        let out = &mut std::io::stdout();
        let bg = Color::AnsiValue(232);
        render_frame(out,
            col1+1, row1, 66, 32,
            bg, Some((bg, Color::Yellow, "Pattern 23 Part 5"))
        )?;
        laterna::demo(out, col1)?;
        Ok(())
    }
    fn update_listing (&mut self) {
        let (entries, max_len) = list_current_directory();
        self.entries = Menu::new(entries);
        self.max_len = max_len as u16;
    }
}

impl TUI for Electribe2PatternsTUI {
    fn render (&self, col1: u16, row1: u16, cols: u16, rows: u16) -> Result<()> {
        let out = &mut std::io::stdout();
        let bg = Color::AnsiValue(232);
        let fg = Color::White;
        let hi = Color::Yellow;
        if let Some(bank) = &self.bank {

            render_frame(out,
                col1, row1, 58, 42,
                bg, Some((
                    if self.focused { hi } else { bg },
                    if self.focused { bg } else { hi },
                    "Patterns:"
                ))
            )?;

            render_pattern_list(
                out, col1 + 1, row1 + 2, 50,
                &bank.patterns,
                self.patterns.index,
                self.offset
            )?;

        } else {

            render_frame(out,
                col1, row1, 4 + self.max_len, 4 + self.entries.items.len() as u16,
                bg, Some((
                    if self.focused { hi } else { bg },
                    if self.focused { bg } else { hi },
                    "Select ALLPAT file:"
                ))
            )?;

            render_directory_listing(
                out, col1 + 1, row1 + 2, self.max_len as usize,
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

pub fn render_pattern_list <W: Write> (
    out: &mut W, col1: u16, row1: u16, pad: usize,
    patterns: &Vec<Electribe2Pattern>,
    selected: usize,
    offset:   usize
) -> Result<()> {
    let bg = Color::AnsiValue(232);
    let fg = Color::White;
    let hi = Color::Yellow;
    queue!(out,
        SetBackgroundColor(bg),
        SetForegroundColor(fg),
        SetAttribute(Attribute::Bold),
        MoveTo(col1, row1),
        Print(format!("{:>3}  {:<16}  {:<5} {:>3}  {:>3}  {:>3}  {:>3}",
            "#",
            "Name",
            "BPM",
            "Length",
            "Beats",
            "Key",
            "Scale"
        )),
        SetAttribute(Attribute::Reset),
        SetBackgroundColor(bg),
    )?;
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
        queue!(out,
            SetForegroundColor(if selected == index+offset { hi } else { fg }),
            MoveTo(col1, row1 + 2 + index as u16),
            Print(row)
        )?;
    }

    render_scrollbar(out, col1 + 55, row1 + 2, patterns.len(), offset, height)?;

    Ok(())
}

struct Electribe2SamplesTUI {}

impl Electribe2SamplesTUI {
    pub fn new () -> Self {
        Self {}
    }
}

impl TUI for Electribe2SamplesTUI {
    fn render (&self, col1: u16, row1: u16, cols: u16, rows: u16) -> Result<()> {
        let out = &mut std::io::stdout();
        let bg = Color::AnsiValue(232);
        render_frame(out,
            col1, row1, 30, 32,
            bg, Some((bg, Color::Yellow, "Samples"))
        )?;
        for i in 1..24 {
            queue!(out,
                SetBackgroundColor(bg),
                SetForegroundColor(Color::White),
                MoveTo(col1 + 1, row1 + 1 + i),
                Print(format!("{:>3} Sample", i))
            )?;
        }
        Ok(())
    }
}
