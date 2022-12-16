use super::{*, super::*};

#[derive(Copy, Clone, Debug, Default)]
pub enum Sizing {
    #[default] Auto,
    Min,
    Max,
    Fixed(Point),
    Stretch(Size)
}

pub enum Layout<'a> {
    None,
    Blank(Sizing),
    Item(Sizing, &'a dyn TUI),
    Layers(Sizing, Vec<Layout<'a>>),
    Column(Sizing, Vec<Layout<'a>>),
    Row(Sizing, Vec<Layout<'a>>),
    Grid(Sizing, Vec<(Layout<'a>, Space)>),
}

impl<'a> std::fmt::Debug for Layout<'a> {
    fn fmt (&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Layout::{}", match self {
            Self::None        => "None",
            Self::Blank(_)    => "Blank",
            Self::Item(_,_)   => "Item",
            Self::Layers(_,_) => "Layers",
            Self::Column(_,_) => "Column",
            Self::Row(_,_)    => "Row",
            Self::Grid(_,_)   => "Grid"
        })
    }
}

impl<'a> TUI for Layout<'a> {
    fn layout (&self) -> Layout {
        Self::Item(Sizing::Auto, &EmptyTUI {})
    }
    fn size (&self) -> Size {
        match self {
            Self::None => {
                Size::MIN
            }
            Self::Blank(sizing) => {
                Size::ANY.resolve(sizing)
            },
            Self::Item(sizing, element) => {
                element.size().resolve(sizing)
            },
            Self::Layers(sizing, layers) => {
                let mut size = Size::MIN;
                for layer in layers.iter() {
                    size = size.stretch(layer.size())
                }
                size.resolve(sizing)
            },
            Self::Column(sizing, elements) => {
                let mut size = Size::MIN;
                for element in elements.iter() {
                    size = size.add_to_column(element.size());
                }
                size.resolve(sizing)
            },
            Self::Row(sizing, elements) => {
                let mut size = Size::default();
                for element in elements.iter() {
                    size = size.add_to_row(element.size());
                }
                size.resolve(sizing)
            },
            Self::Grid(_sizing, _) => {
                unimplemented!()
            }
        }
    }
    fn render (&self, term: &mut dyn Write, space: &Space) -> Result<()> {
        Ok(match self {
            Self::None => {
            },
            Self::Blank(_sizing) => {
            },
            Self::Item(_sizing, element) => {
                element.render(term, space)?
            },
            Self::Layers(_sizing, layers) => {
                for layer in layers.iter() {
                    layer.render(term, space)?;
                }
            },
            Self::Column(_sizing, elements) => {
                let Point(x, y) = space.0;
                let Point(w, h) = self.size().clip(space.1)?;
                let portion = (h / elements.len() as u16).max(1);
                for (index, element) in elements.iter().enumerate() {
                    element.render(term, &Space(
                        Point(x, y + (index as u16 + 0) * portion),
                        Point(w, portion)
                    ))?
                }
            },
            Self::Row(_sizing, elements) => {
                let Point(x, y) = space.0;
                let Point(w, h) = self.size().clip(space.1)?;
                let portion = (w / elements.len() as u16).max(1);
                for (index, element) in elements.iter().enumerate() {
                    element.render(term, &Space(
                        Point(x + (index as u16 + 0) * portion, y),
                        Point(portion, h)
                    ))?
                }
            },
            Self::Grid(_sizing, _) => {
                unimplemented!()
            },
        })
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_layout () {
        let item = Layout::None;

        // Minimum size of empty item is global minimum size
        assert_eq!(
            Layout::Item(Sizing::Min, &item).size(),
            Size::MIN
        );
        // Auto size of empty item is any size
        assert_eq!(
            Layout::Item(Sizing::Auto, &item).size(),
            Size::ANY
        );
        // Maximum size of empty item is global maximum size
        assert_eq!(
            Layout::Item(Sizing::Max, &item).size(),
            Size::MAX
        );

        // Min size of column containing 1 1x1 item is 1x1
        assert_eq!(
            Layout::Column(Sizing::Min, vec![
                Layout::Item(Sizing::Fixed(Point(1, 1)), &item)
            ]).size(),
            Size::fixed(Point(1, 1))
        );
        // Auto size of column containing 1 1x1 item is 1x1
        assert_eq!(
            Layout::Column(Sizing::Auto, vec![
                Layout::Item(Sizing::Fixed(Point(1, 1)), &item)
            ]).size(),
            Size::fixed(Point(1, 1))
        );
        // Max size of column containing 1 1x1 item is 1x1
        assert_eq!(
            Layout::Column(Sizing::Max, vec![
                Layout::Item(Sizing::Fixed(Point(1, 1)), &item)
            ]).size(),
            Size::fixed(Point(1, 1))
        );

        assert_eq!(
            Layout::Column(Sizing::Min, vec![
                Layout::Item(Sizing::Fixed(Point(1, 1)), &item),
                Layout::Item(Sizing::Fixed(Point(1, 1)), &item)
            ]).size(),
            Size::fixed(Point(1, 2))
        );
        assert_eq!(
            Layout::Column(Sizing::Auto, vec![
                Layout::Item(Sizing::Fixed(Point(1, 1)), &item),
                Layout::Item(Sizing::Fixed(Point(1, 1)), &item)
            ]).size(),
            Size::fixed(Point(1, 2))
        );
        assert_eq!(
            Layout::Column(Sizing::Max, vec![
                Layout::Item(Sizing::Fixed(Point(1, 1)), &item),
                Layout::Item(Sizing::Fixed(Point(1, 1)), &item)
            ]).size(),
            Size::fixed(Point(1, 2))
        );

        assert_eq!(
            Layout::Column(Sizing::Auto, vec![
                Layout::Item(Sizing::Auto, &item),
                Layout::Item(Sizing::Auto, &item),
                Layout::Item(Sizing::Auto, &item),
                Layout::Item(Sizing::Auto, &item),
                Layout::Item(Sizing::Auto, &item),
            ]).size(),
            Size::ANY
        );

        assert_eq!(
            Layout::Layers(Sizing::Auto, vec![
                Layout::Item(Sizing::Auto, &item),
                Layout::Column(Sizing::Auto, vec![
                    Layout::Item(Sizing::Fixed(Point(10, 1)), &item),
                    Layout::Item(Sizing::Fixed(Point(12, 1)), &item),
                    Layout::Item(Sizing::Fixed(Point(14, 1)), &item),
                    Layout::Item(Sizing::Fixed(Point(13, 1)), &item),
                    Layout::Item(Sizing::Fixed(Point(11, 1)), &item)
                ]),
            ]).size(),
            Size { min: Point(14,5), max: Point::MAX }
        );

        assert_eq!(
            Layout::Layers(Sizing::Auto, vec![
                Layout::Column(Sizing::Auto, vec![
                    Layout::Item(Sizing::Fixed(Point(10, 1)), &item),
                    Layout::Item(Sizing::Fixed(Point(12, 1)), &item),
                    Layout::Item(Sizing::Fixed(Point(14, 1)), &item),
                    Layout::Item(Sizing::Fixed(Point(13, 1)), &item),
                    Layout::Item(Sizing::Fixed(Point(11, 1)), &item)
                ]),
            ]).size().clip(Point(100, 100)).unwrap(),
            Point(14,5)
        );

        assert_eq!(
            Layout::Layers(Sizing::Auto, vec![
                Layout::Item(Sizing::Min, &item),
                Layout::Column(Sizing::Auto, vec![
                    Layout::Item(Sizing::Fixed(Point(10, 1)), &item),
                    Layout::Item(Sizing::Fixed(Point(12, 1)), &item),
                    Layout::Item(Sizing::Fixed(Point(14, 1)), &item),
                    Layout::Item(Sizing::Fixed(Point(13, 1)), &item),
                    Layout::Item(Sizing::Fixed(Point(11, 1)), &item)
                ]),
            ]).size().clip(Point(100, 100)).unwrap(),
            Point(14,5)
        )

    }

}
