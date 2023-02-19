use std::{io::Write, fmt::Display, slice::Iter};
use super::*;
use thatsit::{
    *,
    layouts::*,
    engines::tui::{
        *,
        crossterm::style::Color
    },
    widgets::tui::*
};

use laterna;

/// UI for managing Korg Electribe 2 patterns and samples
pub struct Electribe2UI<E, I, O>(
    /// Tabs containing the editors for pattern banks and sample banks.
    Tabbed<Box<dyn Widget<E, I, O>>>
);

impl<E, I, O> Electribe2UI<E, I, O> {
    pub fn new () -> Self {
        let mut selector = Tabbed::<Box<dyn Widget<E, I, O>>>::new(TabSide::Left, vec![]);
        selector.add("Edit patterns".into(), Box::new(Electribe2PatternsUI::new()));
        selector.add("Edit samples".into(),  Box::new(Electribe2SamplesUI::new()));
        selector.pages.select_next();
        Self(selector)
    }
}

impl<E, I, O> Input<E, I> for Electribe2UI<E, I, O> {
    fn handle (&mut self, engine: &mut E) -> Result<Option<I>> {
        self.0.handle(engine)
    }
}

impl<E, I, O> Output<E, O> for Electribe2UI<E, I, O> {
    fn render (&self, engine: &mut E) -> Result<Option<O>> {
        self.0.render(engine)
    }
}

/// UI for editing a Korg Electribe 2 pattern bank
#[derive(Debug, Default)]
pub struct Electribe2PatternsUI {
    /// File explorer for selecting a pattern bank
    pub file_list: FileList,
    /// The currently selected pattern bank
    pub bank:      Option<Electribe2PatternBank>,
    /// Selector for editing individual patterns
    pub patterns:  Tabbed<Electribe2PatternUI>,
}

impl<W: Write> Output<TUI<W>, [u16;2]> for Electribe2PatternsUI {

    fn render (&self, engine: &mut TUI<W>) -> Result<Option<[u16;2]>> {

        if self.bank.is_some() {

            let style1 = |s: String|s.on(Color::Yellow).with(Color::Black).bold();
            let style2 = |s: String|s.with(Color::White);

            let max_height = engine.area.h() - 4;
            // TODO make this determined automatically by Rows/Cols by providing shrunken Area

            self.patterns.scroll.size.set(max_height as usize);

            let offset = self.patterns.scroll.offset;

            let mut patterns = Rows::new().add(
                Self::format_header("#", "Name", "BPM", "Length", "Beats", "Key", "Scale")
                    .with(Color::White)
                    .bold()
            );

            for (index, (label, _)) in self.iter().enumerate().skip(offset) {
                if index as Unit >= max_height + offset as Unit {
                    break
                }
                patterns = patterns.add(label.clone().style(&if self.selected() == Some(index) {
                    style1
                } else {
                    style2
                }));
            }

            Rows::new()
                .add(" Patterns in this file:")
                .add(Columns::new()
                    .border(Tall, Inset)
                    .add(patterns)
                    .add(if self.patterns.open && let Some((_,page)) = self.patterns.pages.get() {
                        Some(page)
                    } else {
                        None
                    }))

        } else {

            Rows::new()
                .add(" Select pattern bank:")
                .add(Rows::new().border(Tall, Inset).add(&self.file_list))

        }.render(engine)
    }

}

impl Input<TUIInputEvent, bool> for Electribe2PatternsUI {

    fn handle (&mut self, event: TUIInputEvent) -> Result<Option<bool>> {
        Ok(if self.bank.is_some() {
            if *event == key!(Alt-Up) {
                self.swap_up() && self.select_prev()
            } else if *event == key!(Alt-Down) {
                self.swap_down() && self.select_next()
            } else if *event == key!(Up) {
                self.select_prev()
            } else if *event == key!(Down) {
                self.select_next()
            } else if *event == key!(Enter) {
                self.patterns.open()
            } else {
                false
            }
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
    }

}

impl Electribe2PatternsUI {

    /// Create a new pattern bank editor
    pub fn new () -> Self {
        let mut new = Self::default();
        new.update();
        new
    }

    /// Update the file list
    pub fn update (&mut self) {
        self.file_list.update();
    }

    /// Import a pattern bank from a file
    pub fn import (&mut self, bank: &std::path::Path) {
        self.load_bank(Electribe2PatternBank::read(&super::super::read(bank)));
        self.patterns.open();
    }

    /// Load a pattern bank
    pub fn load_bank (&mut self, bank: Electribe2PatternBank) {
        let new_pages = bank.patterns.iter().enumerate()
            .map(|(index,pattern)|(Self::format_header(
                index + 1,
                pattern.name.trim(),
                pattern.bpm as u64,
                pattern.length,
                pattern.beats,
                pattern.key,
                pattern.scale,
            ), Electribe2PatternUI::new(pattern))).collect::<Vec<_>>();
        self.patterns.scroll.total = new_pages.len();
        self.patterns.pages.replace(new_pages);
        self.patterns.pages.select_next();
        self.bank = Some(bank);
    }

    /// Format a row in the pattern list
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
            "{:>4} │ {:<16} │ {:>5} │ {:>6} │ {:>5} │ {:>3} │ {:>5} │",
            index, name, bpm, length, beats, key, scale,
        )
    }

    /// Switch the places of the current and previous patterns
    pub fn swap_up (&mut self) -> bool {
        if let Some(index) = self.selected() {
            if index > 0 {
                //self.patterns.tabs.items.items.swap(index, index-1);
                self.patterns.pages.items_mut().swap(index, index-1);
                return true
            }
        }
        false
    }

    /// Switch the places of the current and next patterns
    pub fn swap_down (&mut self) -> bool {
        if let Some(index) = self.selected() {
            if index < self.len() - 1 {
                //self.patterns.tabs.items.items.swap(index, index+1);
                self.patterns.pages.items_mut().swap(index, index+1);
                return true
            }
        }
        false
    }

    #[inline] pub fn len (&self) -> usize {
        self.patterns.len()
    }

    #[inline] pub fn open (&mut self) -> bool {
        self.patterns.open()
    }

    #[inline] pub fn selected (&self) -> Option<usize> {
        self.patterns.selected()
    }

    #[inline] pub fn select_prev (&mut self) -> bool {
        self.patterns.select_prev()
    }

    #[inline] pub fn select_next (&mut self) -> bool {
        self.patterns.select_next()
    }

    #[inline] pub fn iter (&self) -> Iter<(String, Electribe2PatternUI)> {
        self.patterns.pages.iter()
    }
}

#[derive(Debug, Default)]
pub struct Electribe2PatternUI(pub Electribe2Pattern, Tabbed<Electribe2PartUI>);

impl Electribe2PatternUI {

    pub fn new (
        pattern: &Electribe2Pattern
    ) -> Self {
        let mut parts = Tabbed::<Electribe2PartUI>::new(TabSide::Left, vec![]);
        for (index, part) in pattern.parts.iter().enumerate() {
            parts.add(format!("Track {}", index + 1), Electribe2PartUI::new(part));
        }
        parts.pages.select_next();
        parts.open();
        Self(pattern.clone(), parts)
    }

    pub fn field <U, V> (
        label: &str, width: U, value: impl Display
    ) -> Fixed<U, V> {
        Fixed::Y(3, Layers::new()
            .add(Fixed::X(width, label.to_string().style(&|s: String|s.with(Color::White).bold())))
            .add(Fixed::X(width, format!(" {}", value.to_string())
                .style(&|s: String|s.with(Color::Green))
                .border(Tall, Inset))))
    }

}

impl<W: Write> Output<TUI<W>, [u16;2]> for Electribe2PatternUI {

    fn render (&self, context: &mut TUI<W>) -> Result<Option<[u16;2]>> {
        Columns::new()
            .add(1)
            .add(Rows::new()
                .add(Columns::new()
                    .add(Self::field("Pattern name", 20, &self.0.name))
                    .add(Self::field("Level",        10, &self.0.level)))
                .add(Columns::new()
                    .add(Self::field("BPM",          10, format!("{:>5.1}", self.0.bpm)))
                    .add(Self::field("Swing",        10, &self.0.swing))
                    .add(Self::field("Length",       10, &self.0.length))
                    .add(Self::field("Beats",        10, &self.0.beats)))
                .add(Columns::new()
                    .add(Self::field("Key",          10, &self.0.key))
                    .add(Self::field("Scale",        10, &self.0.scale))
                    .add(Self::field("Chords",       10, &self.0.chord_set))
                    .add(Self::field("MFX",          10, &self.0.mfx_type)))
                .add(Columns::new()
                    .add(Self::field("Gate arp",     10, &self.0.gate_arp))
                    .add(Self::field("Alt 13/14",    10, &self.0.alt_13_14))
                    .add(Self::field("Alt 15/16",    10, &self.0.alt_15_16)))
                .add(2)
                .add(&self.1))
            .render(context)
    }

}

#[derive(Debug, Default)]
pub struct Electribe2PartUI(pub Electribe2Part);

impl Electribe2PartUI {
    pub fn new (part: &Electribe2Part) -> Self {
        Self(part.clone())
    }

    pub fn field <T, U> (
        label: &str, value: impl Display
    ) -> Fixed<T, U> {
        let white = |s: String|s.with(Color::White).bold();
        let green = |s: String|s.with(Color::Green);
        Fixed::XY((10, 3), Layers::new()
            .add(Columns::new()
                .add((2, 1))
                .add(label.to_string().style(&white)))
            .add(format!(" {}", value.to_string())
                .style(&green)
                .border(Tall, Inset)))
    }

    pub fn layout_metadata <T, U> (&self) -> Rows<T, U> {
        Rows::new()
            .add(Columns::new()
                .add(Self::field("Sample", &self.0.sample))
                .add(Self::field("Pitch",  &self.0.pitch))
                .add(Self::field("Osc",    &self.0.pitch)))
            .add(1)
            .add(Columns::new()
                .add(Self::field("Filter", &self.0.filter_type))
                .add(Self::field("Freq",   &self.0.filter_type))
                .add(Self::field("Res",    &self.0.filter_type)))
            .add(1)
            .add(Columns::new()
                .add(Self::field("Mod",    &self.0.filter_type))
                .add(Self::field("Speed",  &self.0.filter_type))
                .add(Self::field("Depth",  &self.0.filter_type)))
            .add(1)
            .add(Columns::new()
                .add(Self::field("IFX",    &self.0.filter_type))
                .add(Self::field("Type",   &self.0.filter_type))
                .add(Self::field("Param",  &self.0.filter_type)))
    }

    pub fn layout_piano_roll (&self) -> laterna::PianoRoll {
        let mut events = vec![];
        for (index, step) in self.0.steps.iter().enumerate() {
            events.push((index, step.note_1 as usize));
        }
        laterna::PianoRoll(events)
    }

}

impl<W: Write> Output<TUI<W>, [u16;2]> for Electribe2PartUI {

    fn render (&self, engine: &mut TUI<W>) -> Result<Option<[u16;2]>> {
        Columns::new()
            .border(Tall, Inset)
            .add(self.layout_metadata())
            .add(1)
            .add(self.layout_piano_roll())
            .render(engine)
    }

}

#[derive(Debug, Default)]
pub struct Electribe2SamplesUI {
    pub focused: bool,
    pub file_list: FileList,
    pub bank: Option<Electribe2SampleBank>,
    pub sample_list: (),//List<String>,
    pub sample: ()
}

impl<W: Write> Output<TUI<W>, [u16;2]> for Electribe2SamplesUI {

    fn render (&self, engine: &mut TUI<W>) -> Result<Option<[u16;2]>> {
        if self.bank.is_some() {
            Rows::new().add(&self.sample_list).add(&self.sample)
        } else {
            Rows::new().add(&self.file_list)
        }
            .border(Tall, Inset)
            .render(engine)
    }

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
