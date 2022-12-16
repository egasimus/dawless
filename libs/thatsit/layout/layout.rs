use super::{*, super::*};

pub enum Layout<'a> {
    Empty(Sizing),
    Item(Sizing, &'a dyn TUI),
    Layers(Sizing, Vec<Layout<'a>>),
    Column(Sizing, Vec<Layout<'a>>),
    Row(Sizing, Vec<Layout<'a>>),
    Grid(Sizing, Vec<(Layout<'a>, Space)>),
}

pub enum Sizing {
    Auto,
    Min,
    Max,
    Fixed(Point),
    Stretch(Size)
}

impl<'a> TUI for Layout<'a> {
    fn layout (&self) -> Layout {
        Layout::Item(Sizing::Stretch(self.size()), &EmptyTUI {})
    }
    fn size (&self) -> Size {
        match self {
            Layout::Empty(_sizing) => {
                Size::default()
            },
            Layout::Item(_sizing, element) => {
                element.size()
            },
            Layout::Layers(_sizing, layers) => {
                let mut min_w = None;
                let mut min_h = None;
                let mut max_w = None;
                let mut max_h = None;
                for layer in layers.iter() {
                    let size = layer.size();
                    min_w = add_min(min_w, size.min_w);
                    min_h = add_min(min_h, size.min_h);
                    max_w = add_max(max_w, size.max_w);
                    max_h = add_max(max_h, size.max_h);
                }
                Size { min_w, max_w, min_h, max_h }
            },
            Layout::Column(_sizing, elements) => {
                let mut min_w = 0u16;
                let mut min_h = 0u16;
                for element in elements.iter() {
                    let Point(w, h) = element.layout().size().min();
                    min_w = min_w.max(w);
                    min_h = min_h.saturating_add(h);
                }
                Size::from_fixed(Point(min_w, min_h))
            },
            Layout::Row(_sizing, elements) => {
                let mut min_w = 0u16;
                let mut min_h = 0u16;
                for element in elements.iter() {
                    let Point(w, h) = element.layout().size().min();
                    min_w = min_w.saturating_add(w);
                    min_h = min_h.max(h);
                }
                Size::from_fixed(Point(min_w, min_h))
            },
            Layout::Grid(_sizing, _) => {
                unimplemented!()
            }
        }
    }
    fn render (&self, term: &mut dyn Write, space: &Space) -> Result<()> {
        Ok(match self {
            Layout::Empty(_sizing) => {
            },
            Layout::Item(_sizing, element) => {
                element.render(term, space)?
            },
            Layout::Layers(_sizing, layers) => {
                for layer in layers.iter() {
                    layer.render(term, space)?;
                }
            },
            Layout::Column(_sizing, elements) => {
                let portion = (space.1.1 / elements.len() as u16).max(1);
                for (index, element) in elements.iter().enumerate() {
                    element.render(term, &Space(
                        Point(space.0.0, space.0.1 + (index as u16 + 0) * portion),
                        Point(space.1.0, portion)
                    ))?
                }
            },
            Layout::Row(_sizing, elements) => {
                let portion = (space.1.0 / elements.len() as u16).max(1);
                for (index, element) in elements.iter().enumerate() {
                    element.render(term, &Space(
                        Point(space.0.0 + (index as u16 + 0) * portion, space.0.1),
                        Point(portion, space.1.1)
                    ))?
                }
            },
            Layout::Grid(_sizing, _) => {
                unimplemented!()
            },
        })
    }
}
