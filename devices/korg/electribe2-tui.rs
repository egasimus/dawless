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
        Stacked::x(|add|{
            add(Stacked::y(|row|{
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
                        row(Styled(&|s: String|s.on(Color::Yellow).with(Color::Black).bold(), label.clone()));
                    } else {
                        row(Styled(&|s: String|s.with(Color::White), label.clone()));
                    }
                }
            }));
            if self.1.open && let Some((_,page)) = self.1.pages.get() {
                add(page);
            }
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

}

#[derive(Debug, Default)]
pub struct Electribe2PatternUI(pub Electribe2Pattern, TabsLeft<Electribe2PartUI>);

impl Electribe2PatternUI {
    pub fn new (
        pattern: &Electribe2Pattern
    ) -> Self {
        let mut parts = TabsLeft::<Electribe2PartUI>::default();
        for (index, part) in pattern.parts.iter().enumerate() {
            parts.add(format!("Track {}", index + 1), Electribe2PartUI::new(part));
        }
        parts.pages.select_next();
        parts.open();
        Self(pattern.clone(), parts)
    }
    pub fn field (
        label: &str, label_width: Unit, value: impl Display, value_width: Unit
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

impl Widget for Electribe2PatternUI {
    impl_render!(self, out, area => {
        Stacked::x(|add|{
            add(1);
            add(Stacked::y(|add|{
                add(Stacked::x(|add|{
                    add(Self::field("Pattern name", 13, &self.0.name, 20));
                    add(2);
                    add(Self::field("Level", 6, &self.0.level, 8));
                }));
                add(1);
                add(Stacked::x(|add|{
                    add(Self::field("BPM", 4, format!("{:>5.1}", self.0.bpm), 9));
                    add(2);
                    add(Self::field("Swing", 6, &self.0.swing, 9));
                    add(2);
                    add(Self::field("Length", 7, &self.0.length, 10));
                    add(2);
                    add(Self::field("Beats", 6, &self.0.beats, 10));
                }));
                add(1);
                add(Stacked::x(|add|{
                    add(Self::field("Key", 4, &self.0.key, 9));
                    add(2);
                    add(Self::field("Scale", 6, &self.0.scale, 9));
                    add(2);
                    add(Self::field("Chords", 7, &self.0.chord_set, 10));
                    add(2);
                    add(Self::field("MFX", 6, &self.0.mfx_type, 10));
                }));
                add(1);
                add(Stacked::x(|add|{
                    add(Self::field("Gate arp", 9, &self.0.gate_arp, 9));
                    add(2);
                    add(Self::field("Alt 13/14", 10, &self.0.alt_13_14, 10));
                    add(2);
                    add(Self::field("Alt 15/16", 10, &self.0.alt_15_16, 10));
                }));
                add(3);
                add(&self.1);
            }));
        }).render(out, area)
    });
}

#[derive(Debug, Default)]
pub struct Electribe2PartUI(pub Electribe2Part);

impl Electribe2PartUI {
    pub fn new (part: &Electribe2Part) -> Self {
        Self(part.clone())
    }
    pub fn field (
        label: &str, value: impl Display
    ) -> Fix<Stacked> {
        Fix::XY((10, 4), Stacked::y(|add|{
            add(Styled(&|s: String|s.with(Color::White).bold(), label.to_string()));
            add(Border(InsetTall,
                Styled(&|s: String|s.with(Color::Green), format!(" {}", value.to_string()))
            ));
        }))
    }
}

impl Widget for Electribe2PartUI {

    impl_render!(self, out, area => {
        Stacked::x(|add|{
            add(Stacked::y(|add|{
                add(Stacked::x(|add|{
                    add(Self::field("Sample", &self.0.sample));
                    add(1);
                    add(Self::field("Pitch", &self.0.pitch));
                    add(1);
                    add(Self::field("Osc", &self.0.pitch));
                }));
                add(1);
                add(Stacked::x(|add|{
                    add(Self::field("Filter", &self.0.filter_type));
                    add(1);
                    add(Self::field("Freq", &self.0.filter_type));
                    add(1);
                    add(Self::field("Res", &self.0.filter_type));
                }));
                add(1);
                add(Stacked::x(|add|{
                    add(Self::field("Mod", &self.0.filter_type));
                    add(1);
                    add(Self::field("Speed", &self.0.filter_type));
                    add(1);
                    add(Self::field("Depth", &self.0.filter_type));
                }));
                add(1);
                add(Stacked::x(|add|{
                    add(Self::field("IFX", &self.0.filter_type));
                    add(1);
                    add(Self::field("Type", &self.0.filter_type));
                    add(1);
                    add(Self::field("Param", &self.0.filter_type));
                }));
            }));
            add(1);
            add(laterna::PianoRoll(vec![]));
        }).render(out, area)
    });

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
