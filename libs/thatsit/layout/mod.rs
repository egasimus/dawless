use crate::*;

opt_mod::module_flat!(space);
opt_mod::module_flat!(display);

pub type Unit = u16;

/// A pair of coordinates
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Point(pub Unit, pub Unit);

/// A range of sizes
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Size {
    /// The minimum allowable size
    pub min: Point,
    /// The maximum allowable size
    pub max: Point
}

/// How flexible is the sizing of a layout item
#[derive(Copy, Clone, Debug, Default)]
pub enum Sizing {
    #[default] Auto,
    Min,
    Max,
    Fixed(Point),
    Stretch(Size)
}

/// A layout item
#[derive(Clone, Default)]
pub enum Layout<'a> {
    #[default]
    None,
    Blank(Sizing),
    Item(Sizing, &'a dyn TUI),
    Layers(Sizing, Vec<Layout<'a>>),
    Column(Sizing, Vec<Layout<'a>>),
    Row(Sizing, Vec<Layout<'a>>),
    Grid(Sizing, Vec<(Layout<'a>, Space)>),
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add (self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl From<(u16, u16)> for Point {
    fn from ((a, b): (u16, u16)) -> Self {
        Self(a, b)
    }
}

impl Point {
    pub const NUL: Self = Self(0, 0);
    pub const MIN: Self = Self(Unit::MIN, Unit::MIN);
    pub const MAX: Self = Self(Unit::MAX, Unit::MAX);
    pub fn clip (self, other: Self) -> Self {
        Self(self.0.min(other.0), self.1.min(other.1))
    }
}

impl Size {

    /// Any size between minimum and maximum
    pub const ANY: Self = Self { min: Point::MIN, max: Point::MAX };

    /// The minimum size
    pub const MIN: Self = Self { min: Point::MIN, max: Point::MIN };

    /// The maximum size
    pub const MAX: Self = Self { min: Point::MAX, max: Point::MAX };

    /// Create a fixed size
    pub fn fixed (size: Point) -> Self {
        Self { min: size, max: size }
    }

    /// Increment size to fit other
    pub fn stretch (self, other: Size) -> Self {
        let Size { min: Point(old_min_x, old_min_y), max: Point(old_max_x, old_max_y) } = self;
        let Size { min: Point(new_min_x, new_min_y), max: Point(new_max_x, new_max_y) } = other;
        Self {
            min: Point(old_min_x.max(new_min_x), old_min_y.max(new_min_y)),
            max: Point(old_max_x.max(new_max_x), old_max_y.max(new_max_y))
        }
    }
    /// Stretch width and append height
    pub fn add_to_column (self, other: Size) -> Self {
        let Size { min: Point(old_min_x, old_min_y), max: Point(old_max_x, old_max_y) } = self;
        let Size { min: Point(new_min_x, new_min_y), max: Point(new_max_x, new_max_y) } = other;
        Self {
            min: Point(old_min_x.max(new_min_x), old_min_y.saturating_add(new_min_y)),
            max: Point(old_max_x.max(new_max_x), old_max_y.saturating_add(new_max_y))
        }
    }
    /// Stretch height and append width
    pub fn add_to_row (self, other: Size) -> Self {
        let Size { min: Point(old_min_x, old_min_y), max: Point(old_max_x, old_max_y) } = self;
        let Size { min: Point(new_min_x, new_min_y), max: Point(new_max_x, new_max_y) } = other;
        Self {
            min: Point(old_min_x.saturating_add(new_min_x), old_min_y.max(new_min_y)),
            max: Point(old_max_x.saturating_add(new_max_x), old_max_y.max(new_max_y))
        }
    }

    pub fn clip (self, max_size: Point) -> Result<Point> {
        let Point(mut w, mut h) = max_size;
        let Size { min: Point(min_w, min_h), max: Point(max_w, max_h) } = self;
        println!("clip: {w}x{h}, min: {min_w}x{min_h}, max: {max_w}x{max_h}");
        if w < min_w {
            let message = format!("{}x{} < {}x{}", w, h, min_w, min_h);
            return Err(Error::new(ErrorKind::Other, message))
        }
        if h < min_h {
            let message = format!("{}x{} < {}x{}", w, h, min_w, min_h);
            return Err(Error::new(ErrorKind::Other, message))
        }
        if w > max_w {
            w = max_w
        }
        if h > max_h {
            h = max_h
        }
        Ok(Point(w, h))
    }

    pub fn resolve (self, sizing: &Sizing) -> Self {
        match sizing {
            Sizing::Auto          => self,
            Sizing::Min           => Size::fixed(self.min),
            Sizing::Max           => Size::fixed(self.max),
            Sizing::Fixed(point)  => Size::fixed(*point),
            Sizing::Stretch(size) => *size
        }
    }

}

impl<'a> TUI for Layout<'a> {
    fn layout (&self) -> Layout {
        Self::Item(Sizing::Auto, &EmptyTUI {})
    }
    fn size (&self) -> Size {
        match self {
            Self::None => {
                Size::ANY
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
                    match layer {
                        Self::None => {},
                        _ => size = size.stretch(layer.size())
                    }
                }
                size.resolve(sizing)
            },
            Self::Column(sizing, elements) => {
                let mut size = Size::MIN;
                for element in elements.iter() {
                    match element {
                        Self::None => {},
                        _ => size = size.add_to_column(element.size())
                    };
                }
                size.resolve(sizing)
            },
            Self::Row(sizing, elements) => {
                let mut size = Size::default();
                for element in elements.iter() {
                    match element {
                        Self::None => {},
                        _ => size = size.add_to_row(element.size())
                    };
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
            Self::Blank(_) => {
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

    const ITEM: &'static Layout = &Layout::None;
    const SCREEN: Point = Point(100, 100);

    #[test]
    fn test_layout () {

        // Minimum size of empty item is global minimum size
        //let layout = Layout::Item(Sizing::Min, ITEM);
        //assert_eq!(layout.size(), Size::MIN);
        //assert_eq!(layout.size().clip(Point(100, 100)).unwrap(), Size::MIN.min);

        //// Auto size of empty item is any size
        //assert_eq!(
            //Layout::Item(Sizing::Auto, ITEM).size(),
            //Size::ANY
        //);
        //// Maximum size of empty item is global maximum size
        //assert_eq!(
            //Layout::Item(Sizing::Max, ITEM).size(),
            //Size::MAX
        //);

        //// Min size of column containing 1 1x1 item is 1x1
        //assert_eq!(
            //Layout::Column(Sizing::Min, vec![
                //Layout::Item(Sizing::Fixed(Point(1, 1)), ITEM)
            //]).size(),
            //Size::fixed(Point(1, 1))
        //);
        //// Auto size of column containing 1 1x1 item is 1x1
        //assert_eq!(
            //Layout::Column(Sizing::Auto, vec![
                //Layout::Item(Sizing::Fixed(Point(1, 1)), ITEM)
            //]).size(),
            //Size::fixed(Point(1, 1))
        //);
        //// Max size of column containing 1 1x1 item is 1x1
        //assert_eq!(
            //Layout::Column(Sizing::Max, vec![
                //Layout::Item(Sizing::Fixed(Point(1, 1)), ITEM)
            //]).size(),
            //Size::fixed(Point(1, 1))
        //);

        //assert_eq!(
            //Layout::Column(Sizing::Min, vec![
                //Layout::Item(Sizing::Fixed(Point(1, 1)), ITEM),
                //Layout::Item(Sizing::Fixed(Point(1, 1)), ITEM)
            //]).size(),
            //Size::fixed(Point(1, 2))
        //);
        //assert_eq!(
            //Layout::Column(Sizing::Auto, vec![
                //Layout::Item(Sizing::Fixed(Point(1, 1)), ITEM),
                //Layout::Item(Sizing::Fixed(Point(1, 1)), ITEM)
            //]).size(),
            //Size::fixed(Point(1, 2))
        //);
        //assert_eq!(
            //Layout::Column(Sizing::Max, vec![
                //Layout::Item(Sizing::Fixed(Point(1, 1)), ITEM),
                //Layout::Item(Sizing::Fixed(Point(1, 1)), ITEM)
            //]).size(),
            //Size::fixed(Point(1, 2))
        //);

        //assert_eq!(
            //Layout::Column(Sizing::Auto, vec![
                //Layout::Item(Sizing::Auto, ITEM),
                //Layout::Item(Sizing::Auto, ITEM),
                //Layout::Item(Sizing::Auto, ITEM),
                //Layout::Item(Sizing::Auto, ITEM),
                //Layout::Item(Sizing::Auto, ITEM),
            //]).size(),
            //Size::ANY
        //);

        //assert_eq!(
            //Layout::Layers(Sizing::Auto, vec![
                //Layout::Item(Sizing::Auto, ITEM),
                //Layout::Column(Sizing::Auto, vec![
                    //Layout::Item(Sizing::Fixed(Point(10, 1)), ITEM),
                    //Layout::Item(Sizing::Fixed(Point(12, 1)), ITEM),
                    //Layout::Item(Sizing::Fixed(Point(14, 1)), ITEM),
                    //Layout::Item(Sizing::Fixed(Point(13, 1)), ITEM),
                    //Layout::Item(Sizing::Fixed(Point(11, 1)), ITEM)
                //]),
            //]).size(),
            //Size { min: Point(14,5), max: Point::MAX }
        //);

        //assert_eq!(
            //Layout::Layers(Sizing::Auto, vec![
                //Layout::Column(Sizing::Auto, vec![
                    //Layout::Item(Sizing::Fixed(Point(10, 1)), ITEM),
                    //Layout::Item(Sizing::Fixed(Point(12, 1)), ITEM),
                    //Layout::Item(Sizing::Fixed(Point(14, 1)), ITEM),
                    //Layout::Item(Sizing::Fixed(Point(13, 1)), ITEM),
                    //Layout::Item(Sizing::Fixed(Point(11, 1)), ITEM)
                //]),
            //]).size().clip(Point(100, 100)).unwrap(),
            //Point(14,5)
        //);

        //assert_eq!(
            //Layout::Layers(Sizing::Auto, vec![
                //Layout::Item(Sizing::Min, ITEM),
                //Layout::Column(Sizing::Auto, vec![
                    //Layout::Item(Sizing::Fixed(Point(10, 1)), ITEM),
                    //Layout::Item(Sizing::Fixed(Point(12, 1)), ITEM),
                    //Layout::Item(Sizing::Fixed(Point(14, 1)), ITEM),
                    //Layout::Item(Sizing::Fixed(Point(13, 1)), ITEM),
                    //Layout::Item(Sizing::Fixed(Point(11, 1)), ITEM)
                //]),
            //]).size().clip(Point(100, 100)).unwrap(),
            //Point(14,5)
        //);

        //assert_eq!(
            //Layout::Layers(Sizing::Auto, vec![
                //Layout::Item(Sizing::Min, ITEM),
                //Layout::Row(Sizing::Auto, vec![
                    //Layout::Item(Sizing::Fixed(Point(1, 10)), ITEM),
                    //Layout::Item(Sizing::Fixed(Point(1, 12)), ITEM),
                    //Layout::Item(Sizing::Fixed(Point(1, 14)), ITEM),
                    //Layout::Item(Sizing::Fixed(Point(1, 13)), ITEM),
                    //Layout::Item(Sizing::Fixed(Point(1, 11)), ITEM)
                //]),
            //]).size().clip(Point(100, 100)).unwrap(),
            //Point(5, 14)
        //);

        //assert_eq!(
            //Layout::Layers(Sizing::Auto, vec![
                //Layout::Item(Sizing::Min, ITEM),
                //Layout::Column(Sizing::Auto, vec![
                    //Layout::Item(Sizing::Fixed(Point(10, 1)), ITEM),
                    //Layout::Item(Sizing::Fixed(Point(12, 1)), ITEM),
                    //Layout::Item(Sizing::Fixed(Point(14, 1)), ITEM),
                    //Layout::Item(Sizing::Fixed(Point(13, 1)), ITEM),
                    //Layout::Item(Sizing::Fixed(Point(11, 1)), ITEM)
                //]),
                //Layout::Row(Sizing::Auto, vec![
                    //Layout::Item(Sizing::Fixed(Point(1, 10)), ITEM),
                    //Layout::Item(Sizing::Fixed(Point(1, 12)), ITEM),
                    //Layout::Item(Sizing::Fixed(Point(1, 14)), ITEM),
                    //Layout::Item(Sizing::Fixed(Point(1, 13)), ITEM),
                    //Layout::Item(Sizing::Fixed(Point(1, 11)), ITEM)
                //]),
                //Layout::None
            //]).size().clip(Point(100, 100)).unwrap(),
            //Point(14, 14)
        //);


    }

    #[test]
    fn test_layers_clipping () {
        for (sizing, expected) in [
            (Sizing::Min,  Point::MIN),
            (Sizing::Auto, SCREEN),
            (Sizing::Max,  SCREEN)
        ] {
            assert_eq!(
                Layout::Layers(sizing, vec![
                    Layout::Item(Sizing::Auto, ITEM),
                    Layout::Row(Sizing::Auto, vec![
                        Layout::Item(Sizing::Auto, ITEM),
                        Layout::None
                    ])
                ]).size().clip(SCREEN).unwrap(),
                expected
            );
        }
    }

}

/*
/// TODO:
macro_rules! layout {
    ($self:ident, $($layout:tt)+) => {
        fn layout (&$self) -> Layout {
            layout!(@ $($layout)+)
        }
    };
    (@ Item($($layout:tt)+)) => {
        Layout::Item(layout!(@ $($layout)+))
    };
    (@ Min($($layout:tt)+)) => {
        Layout::Min(layout!(@ $($layout)+))
    };
    (@ Max($expr:expr)) => {
        Layout::Max($expr)
    };
    (@ Layers($($op:ident ($($layout:tt)+)),+)) => {
        Layout::Layers(vec![$($op($($layout)+)),+])
    };
    (@ Row($($expr:expr)+)) => {
        Layout::Row(vec![$($expr),+])
    };
    (@ $expr:expr) => {
        $expr
    };
}
*/
