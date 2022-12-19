use crate::electribe2::*;
use thatsit::{*, crossterm::{
    QueueableCommand,
    event::{Event, KeyEvent, KeyCode},
    cursor::MoveTo,
    style::{
        Color, ResetColor, SetBackgroundColor, SetForegroundColor,
        Attribute, SetAttribute,
        Print
    },
}};
use thatsit_fs::*;

#[derive(Debug, Default)]
pub struct Electribe2PatternsTUI {
    pub focused: bool,
    pub frame: Frame,
    pub file_list: FileList,
    pub bank: Option<Electribe2PatternBank>,
    pub patterns: List<String>,
    pub pattern_list: PatternList,
    pub pattern: Pattern,
    pub offset: usize,
}

#[derive(Debug, Default)]
pub struct PatternList {
    pub patterns: Vec<Electribe2Pattern>,
    pub selected: usize,
    pub offset:   usize
}

#[derive(Debug, Default)]
pub struct Pattern {
    pattern: Electribe2Pattern
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
        let (entries, _) = list_current_directory();
        self.file_list.0.replace(entries);
    }
}

impl TUI for Electribe2PatternsTUI {
    fn layout (&self) -> Layout {
        let Self { focused, offset, .. } = *self;
        if let Some(bank) = &self.bank {
            Layout::Layers(Sizing::AUTO, vec![
                Layout::Item(Sizing::AUTO, &self.frame),
                Layout::Column(Sizing::AUTO, vec![
                    Layout::Item(Sizing::AUTO, &self.pattern_list),
                    Layout::Item(Sizing::AUTO, &self.pattern),
                ])
            ])
        } else {
            Layout::Layers(Sizing::AUTO, vec![
                Layout::Item(Sizing::AUTO, &self.frame),//Frame { theme: THEME, focused, title: "Select ALLPAT file (Esc to exit)".into() }),
                Layout::Item(Sizing::AUTO, &self.file_list),//FileList(&self.entries))
            ])
        }
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
            Ok(
                is_key!(event => KeyCode::Enter => {
                    let (path, is_dir) = &self.file_list.0.items.get(self.file_list.0.index).unwrap().1;
                    if *is_dir {
                        std::env::set_current_dir(path)?;
                        self.update_listing();
                    } else {
                        let path = std::path::PathBuf::from(path);
                        self.import(&path);
                    }
                    true
                }) || self.file_list.handle(event)?
            )
        }
    }
}


impl TUI for PatternList {
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

impl TUI for Pattern {
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
