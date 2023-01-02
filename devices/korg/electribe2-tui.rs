use crate::electribe2::*;
use crate::*;
use std::{fmt::Display, slice::Iter};
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
        let mut selector = TabsLeft::<Box<dyn Widget>>::default();
        selector.add("Edit patterns".into(), Box::new(Electribe2PatternsUI::new()));
        selector.add("Edit samples".into(),  Box::new(Electribe2SamplesUI::new()));
        selector.pages.select_next();
        Self(selector)
    }
}

impl Widget for Electribe2UI {
    impl_render!(self, out, area => self.0.render(out, area));
    impl_handle!(self, event => {
        self.0.handle(event)
    });
}

/// UI for editing a Korg Electribe 2 pattern bank
#[derive(Debug)]
pub struct Electribe2PatternsUI {
    /// File explorer for selecting a pattern bank
    pub file_list: FileList,
    /// The currently selected pattern bank
    pub bank:      Option<Electribe2PatternBank>,
    /// Selector for editing individual patterns
    pub patterns:  Electribe2PatternListUI,
}

impl Widget for Electribe2PatternsUI {

    impl_render!(self, out, area => {
        if self.bank.is_some() {
            Stacked::y(|row|{
                row(" Patterns in this file:");
                row(Border(InsetTall, &self.patterns));
            })
        } else {
            Stacked::y(|row|{
                row(" Select pattern bank:");
                row(Border(InsetTall, Stacked::y(|row|{
                    row(&self.file_list);
                })));
            })
        }.render(out, area)
    });

    impl_handle!(self, event => {
        Ok(if self.bank.is_some() {
            self.patterns.handle(event)?
        } else {
            self.file_list.handle(event)? || if_key!(event => Enter => {
                if let Some(FileEntry { path, is_dir, .. }) = self.file_list.selected() {
                    if *is_dir {
                        std::env::set_current_dir(path)?;
                        self.update();
                    } else {
                        self.import(&std::path::PathBuf::from(path));
                    }
                    true
                } else {
                    false
                }
            })
        })
    });
}

impl Electribe2PatternsUI {
    pub fn new () -> Self {
        let mut file_list = FileList::default();
        file_list.update();
        Self {
            bank: None,
            patterns: Electribe2PatternListUI::default(),
            file_list,
        }
    }
    pub fn update (&mut self) {
        self.file_list.update();
    }
    pub fn import (&mut self, bank: &std::path::Path) {
        let data = crate::read(bank);
        let bank = Electribe2PatternBank::read(&data);
        self.bank = Some(bank);
        self.patterns.replace(self.bank.as_ref().unwrap());
    }
}

#[derive(Debug, Default)]
pub struct Electribe2PatternListUI(
    /// Scroll offset
    usize,
    /// Collection of pattern editors
    TabsLeft<Electribe2PatternUI>
);

impl Widget for Electribe2PatternListUI {

    impl_render!(self, out, area => {
        Stacked::x(|column|{
            column(self.render_list(area));
            column(self.render_detail(area));
        }).render(out, area)
    });

    impl_handle!(self, event => Ok(
        if *event == key!(Ctrl-Up) {
            self.swap_up()
        } else if *event == key!(Ctrl-Down) {
            self.swap_down()
        } else if *event == key!(Up) {
            self.1.pages.select_prev()
        } else if *event == key!(Down) {
            self.1.pages.select_next()
        } else if *event == key!(Enter) {
            self.1.open()
        } else {
            false
        }
    ));

}

impl Electribe2PatternListUI {

    pub fn format_header (
        index:  impl Display,
        name:   impl Display,
        bpm:    impl Display,
        length: impl Display,
        beats:  impl Display,
        key:    impl Display,
        scale:  impl Display
    ) -> String {
        format!(
            "{:>3} │ {:<16} │ {:>5} │ {:>6} │ {:>5} │ {:>3} │ {:>5} │",
            index,
            name,
            bpm,
            length,
            beats,
            key,
            scale,
        )
    }
    #[inline]
    pub fn len (&self) -> usize {
        self.1.len()
    }
    pub fn replace (&mut self, bank: &Electribe2PatternBank) {
        self.1.pages.replace(bank.patterns.iter().enumerate()
            .map(|(index,pattern)|(Self::format_header(
                index,
                pattern.name.trim(),
                pattern.bpm as u64,
                pattern.length,
                pattern.beats,
                pattern.key,
                pattern.scale,
            ), Electribe2PatternUI::new(pattern))).collect::<Vec<_>>());
        self.1.pages.select_next();
    }
    #[inline]
    pub fn selected (&self) -> Option<usize> {
        self.1.pages.selected()
    }
    #[inline]
    pub fn iter (&self) -> Iter<(String, Electribe2PatternUI)> {
        self.1.pages.iter()
    }
    pub fn swap_up (&mut self) -> bool {
        if let Some(index) = self.selected() {
            if index > 0 {
                //self.patterns.tabs.items.items.swap(index, index-1);
                self.1.pages.items_mut().swap(index, index-1);
                return true
            }
        }
        false
    }
    pub fn swap_down (&mut self) -> bool {
        if let Some(index) = self.selected() {
            if index < self.len() - 1 {
                //self.patterns.tabs.items.items.swap(index, index+1);
                self.1.pages.items_mut().swap(index, index+1);
                return true
            }
        }
        false
    }
    //pub fn index (&self) -> usize { self.0.index() }

    pub fn render_list (&self, area: Area) -> Stacked {
        Stacked::y(|row|{
            row(Self::format_header(
                "#", "Name", "BPM", "Length", "Beats", "Key", "Scale"
            ).with(Color::White).bold());

            let max_height = area.h() - 1; // TODO determine automatically by Stacked
                                           // by providing shrunken Area

            for (index, (label, _)) in self.iter().enumerate().skip(self.0) {
                if index as Unit >= max_height {
                    break
                }
                if let Some(selected) = self.selected() && selected == index {
                    row(Styled(&|s: String|s.with(Color::Yellow), label.clone()));
                } else {
                    row(Styled(&|s: String|s.with(Color::White), label.clone()));
                }
            }
        })
    }

    pub fn render_detail (&self, area: Area) -> Option<Stacked> {
        if self.1.open && let Some((_,page)) = self.1.pages.get() {
            Some(Stacked::x(|column|{
                column((1, 0));
                column(Border(InsetTall, Stacked::y(|row|{
                    row(Stacked::x(|column|{
                        column(1);
                        column(self.render_field("Pattern name", &page.pattern.name));
                        column(self.render_field("Level",        &page.pattern.level));
                    }));
                    row(Stacked::x(|column|{
                        column(1);
                        column(self.render_field("BPM",          &page.pattern.bpm));
                        column(self.render_field("Length",       &page.pattern.length));
                        column(self.render_field("Beats",        &page.pattern.beats));
                    }));
                })));
            }))
        } else {
            None
        }
    }

    pub fn render_field (&self, label: &str, value: impl Display) -> Stacked {
        Stacked::x(|column|{
            column(Styled(&|s: String|s.with(Color::White).bold(), label.to_string()));
            column(Border(InsetTall, value.to_string()));
        })
    }
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

impl Widget for Electribe2PatternUI {
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
pub struct Electribe2SamplesUI {
    pub focused: bool,
    pub file_list: FileList,
    pub bank: Option<Electribe2SampleBank>,
    pub sample_list: (),//List<String>,
    pub sample: ()
}

impl Widget for Electribe2SamplesUI {
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

impl Electribe2SamplesUI {
    pub fn new () -> Self {
        Self::default().update()
    }
    fn update (mut self) -> Self {
        self.file_list.update();
        self
    }
}
