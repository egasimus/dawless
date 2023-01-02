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
            column(self.layout_list(area));
            column(self.layout_detail());
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

    pub fn layout_list (&self, area: Area) -> Stacked {
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

    pub fn layout_detail (&self) -> Option<Border<InsetTall, Stacked>> {
        if self.1.open && let Some((_,page)) = self.1.pages.get() {
            Some(Border(InsetTall, Stacked::x(|add|{
                add(1);
                add(Stacked::y(|add|{
                    add(Stacked::x(|add|{
                        add(self.layout_field("Pattern name", 13, &page.pattern.name, 24));
                        add(2);
                        add(self.layout_field("Level", 6, &page.pattern.level, 8));
                    }));
                    add(1);
                    add(Stacked::x(|add|{
                        add(self.layout_field("BPM", 4, format!("{:>5.1}", page.pattern.bpm), 9));
                        add(2);
                        add(self.layout_field("Swing", 6, &page.pattern.swing, 9));
                        add(2);
                        add(self.layout_field("Length", 7, &page.pattern.length, 10));
                        add(2);
                        add(self.layout_field("Beats", 6, &page.pattern.beats, 10));
                    }));
                    add(1);
                    add(Stacked::x(|add|{
                        add(self.layout_field("Key", 4, &page.pattern.key, 9));
                        add(2);
                        add(self.layout_field("Scale", 6, &page.pattern.scale, 9));
                        add(2);
                        add(self.layout_field("Chords", 7, &page.pattern.chord_set, 10));
                        add(2);
                        add(self.layout_field("MFX", 6, &page.pattern.mfx_type, 10));
                    }));
                    add(1);
                    add(Stacked::x(|add|{
                        add(self.layout_field("Gate arp", 9, &page.pattern.gate_arp, 9));
                        add(2);
                        add(self.layout_field("Alt 13/14", 10, &page.pattern.alt_13_14, 10));
                        add(2);
                        add(self.layout_field("Alt 15/16", 10, &page.pattern.alt_15_16, 10));
                    }))
                }));
            })))
        } else {
            None
        }
    }

    pub fn layout_field (
        &self, label: &str, label_width: Unit, value: impl Display, value_width: Unit
    ) -> Fix<Stacked> {
        Fix::XY((label_width + value_width, 3), Stacked::x(|add|{
            add(Fix::X(label_width,
                Styled(&|s: String|s.with(Color::White).bold(), label.to_string())
            ));
            add(Fix::X(value_width, Border(InsetTall,
                Styled(&|s: String|s.with(Color::Green), format!(" {}", value.to_string()))
            )));
        }))
    }
}

#[derive(Debug, Default)]
pub struct Electribe2PatternUI {
    pattern: Electribe2Pattern,
}

impl Electribe2PatternUI {
    fn new (pattern: &Electribe2Pattern) -> Self {
        Self { pattern: pattern.clone(), }
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
