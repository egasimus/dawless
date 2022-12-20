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
    pub label: Label,
    pub file_list: FileList,
    pub bank: Option<Electribe2PatternBank>,
    pub patterns: PatternList,
    pub pattern: Pattern,
    pub offset: usize,
}

impl Electribe2PatternsTUI {
    pub fn new () -> Self {
        let mut new = Self::default();
        new.label.text = "Select pattern bank:".into();
        new.update_listing();
        return new
    }
    pub fn import (&mut self, bank: &std::path::Path) {
        let data = crate::read(bank);
        let bank = Electribe2PatternBank::read(&data);
        self.bank = Some(bank);
        let patterns: Vec<(String, Electribe2Pattern)> = self.bank.as_ref().unwrap().patterns.iter()
            .map(|pattern|(pattern.name.clone(), pattern.clone()))
            .collect();
        self.patterns.0.replace(patterns);
    }
    fn update_listing (&mut self) {
        let (entries, _) = list_current_directory();
        self.file_list.list.replace(entries);
    }
}

impl TUI for Electribe2PatternsTUI {
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        let Self { focused, offset, .. } = *self;
        Layout::Layers(Sizing::Pad(1, &Sizing::AUTO), vec![
            Layout::Item(Sizing::AUTO, &self.frame),
            if let Some(bank) = &self.bank {
                Layout::Row(Sizing::AUTO, vec![
                    Layout::Item(Sizing::AUTO, &self.patterns),
                    Layout::Item(Sizing::AUTO, &self.pattern),
                ])
            } else {
                Layout::Column(Sizing::AUTO, vec![
                    Layout::Item(Sizing::Min, &self.label),
                    Layout::Item(Sizing::Pad(1, &Sizing::AUTO), &self.file_list),
                ])
            }
        ]).render(term, area)
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
                    self.patterns.0.items.len(), self.patterns.0.index, 36, self.offset
                );
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(
                is_key!(event => KeyCode::Enter => {
                    let (path, is_dir) = &self.file_list.list.items.get(self.file_list.list.index).unwrap().1;
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

#[derive(Debug, Default)]
pub struct PatternList(List<Electribe2Pattern>);

impl TUI for PatternList {
    fn min_size (&self) -> Size {
        Size(24, 10)
    }
    fn max_size (&self) -> Size {
        Size(24, 10)
    }
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        return self.0.render(term, area);
        return Layout::Item(
            Sizing::Range(self.min_size(), self.max_size()), &self.0
        ).render(term, area);
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
    }
}

#[derive(Debug, Default)]
pub struct Pattern {
    pattern: Electribe2Pattern
}

impl TUI for Pattern {
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        return Layout::Item(Sizing::Min, &DebugBox { bg: Color::AnsiValue(100) })
            .render(term, area);

        let Area(Point(x, y), Size(w, h)) = area;
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
