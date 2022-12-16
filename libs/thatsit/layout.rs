use crate::*;

pub type Unit = u16;

/// A pair of coordinates
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Point (
    /// Column
    pub Unit,
    /// Row
    pub Unit
);

/// A pair of dimensions
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Area (
    /// Width
    pub Unit,
    /// Height
    pub Unit
);

/// (minimum, maximum) area
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct AreaRange (
    /// Minimum
    pub Area,
    /// Maximum
    pub Area
);

/// A range of sizes
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Size {
    /// The minimum allowable size
    pub min: Point,
    /// The maximum allowable size
    pub max: Point
}

/// How flexible is the sizing of a layout item
#[derive(Copy, Clone, Debug)]
pub enum Sizing {
    Grow(Unit),
    Fixed(Area)
}

/// A layout item
#[derive(Clone)]
pub enum Layout<'a> {
    Item(Sizing, &'a dyn TUI),
    Layers(Sizing, Vec<Layout<'a>>),
    Column(Sizing, Vec<Layout<'a>>),
    Row(Sizing, Vec<Layout<'a>>),
    Grid(Sizing, Vec<(Layout<'a>, Area)>),
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

impl Area {
    pub const MIN: Self = Self(0, 0);
    pub const MAX: Self = Self(Unit::MAX, Unit::MAX);

    /// Increase own size to fit other
    pub fn stretch (self, other: Self) -> Self {
        Self(self.0.max(other.0), self.1.max(other.1))
    }

    /// Grow width, stretch height
    pub fn expand_row (self, other: Self) -> Self {
        Self(self.0.saturating_add(other.0), self.1.max(other.1))
    }

    /// Stretch width, grow height
    pub fn expand_column (self, other: Self) -> Self {
        Self(self.0.max(other.0), self.1.saturating_add(other.1))
    }

    /// Return an error if the other area is too small
    pub fn fits (self, other: Self) -> Result<()> {
        if self.0 > other.0 {
            let message = format!("need {} columns", self.0);
            return Err(Error::new(ErrorKind::Other, message))
        }
        if self.1 > other.1 {
            let message = format!("need {} rows", self.0);
            return Err(Error::new(ErrorKind::Other, message))
        }
        Ok(())
    }

    pub fn width (self) -> Unit {
        self.0
    }
    pub fn height (self) -> Unit {
        self.1
    }
}

impl<'a> TUI for Layout<'a> {
    fn min_size (&self) -> Area {
        match self {
            Self::Item(_, item) => item.min_size(),
            Self::Layers(_, layers) => {
                let mut size = Area::MIN;
                for layer in layers.iter() { size = size.stretch(layer.min_size()); }
                size
            },
            Self::Row(_, items) => {
                let mut size = Area::MIN;
                for item in items.iter() { size = size.expand_row(item.min_size()); }
                size
            },
            Self::Column(_, items) => {
                let mut size = Area::MIN;
                for item in items.iter() { size = size.expand_column(item.min_size()); }
                size
            },
            Self::Grid(_, _) => unimplemented!()
        }
    }
    fn max_size (&self) -> Area {
        match self {
            Self::Item(_, item) => item.max_size(),
            Self::Layers(_, layers) => {
                let mut size = Area::MIN;
                for layer in layers.iter() { size = size.stretch(layer.max_size()); }
                size
            },
            Self::Row(_, items) => {
                let mut size = Area::MIN;
                for item in items.iter() { size = size.expand_row(item.max_size()); }
                size
            },
            Self::Column(_, items) => {
                let mut size = Area::MIN;
                for item in items.iter() { size = size.expand_column(item.max_size()); }
                size
            },
            Self::Grid(_, _) => unimplemented!()
        }
    }
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        Ok(match self {
            Self::None => {
            },
            Self::Blank(_) => {
            },
            Self::Item(_sizing, element) => {
                element.render(term, area)?
            },
            Self::Layers(_, layers) => {
                for layer in layers.iter() {
                    layer.render(term, area)?;
                }
            },
            Self::Column(_, elements) => {
                let sizes = flex(space.1.0, *elements, Area::height)
                unimplemented!()
            },
            Self::Row(_, elements) => {
                let sizes = flex(space.1.0, *elements, Area::width)
                unimplemented!()
            },
            Self::Grid(_sizing, _) => {
                unimplemented!()
            },
        })
    }
}

/// Distribute space between widgets
pub fn flex <'a, F: Fn(Area)->Unit> (
    mut remaining: Unit, sizings: Vec<Sizing>, axis: F
) -> Result<Vec<Unit>> {
    let mut denominator = 0;
    for sizing in sizings {
        match sizing {
            Sizing::Fixed(area) => {
                let size = axis(area);
                if size > remaining {
                    return Err(Error::new(ErrorKind::Other, "not enough space"))
                }
                remaining = remaining - size
            },
            Sizing::Grow(proportion) => {
                denominator += proportion
            }
        }
    }
    let sizes = vec![];
    for sizing in sizings {
        match sizing {
            Sizing::Fixed(area) => {
                sizes.push(axis(area))
            },
            Sizing::Grow(proportion) => {
                sizes.push(remaining * proportion / denominator)
            }
        }
    }
    Ok(sizes)
}

#[cfg(test)]
mod test {

    use super::*;

    const ITEM: &'static Layout = &Layout::None;
    const SCREEN: Point = Point(100, 100);

    #[test]
    fn test_min_size () {
        let layout = Layout::None;
        assert_eq!(get_min_size(layout), Point(0, 0));

        let layout = Layout::Item(Sizing::Auto, ITEM);
        assert_eq!(get_min_size(layout), Point(0, 0));

        let layout = Layout::Item(Sizing::Fixed(Point(10, 20)), ITEM);
        assert_eq!(get_min_size(layout), Point(10, 20));

        let layout = Layout::Column(Sizing::Auto, vec![
            Layout::Item(Sizing::Fixed(Point(10, 20)), ITEM),
            Layout::Item(Sizing::Fixed(Point(20, 10)), ITEM)
        ]);
        assert_eq!(get_min_size(layout), Point(20, 30));

        let layout = Layout::Row(Sizing::Auto, vec![
            Layout::Item(Sizing::Fixed(Point(10, 20)), ITEM),
            Layout::Item(Sizing::Fixed(Point(20, 10)), ITEM)
        ]);
        assert_eq!(get_min_size(layout), Point(30, 20));
    }

    #[test]
    fn test_flex () {
    }

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
