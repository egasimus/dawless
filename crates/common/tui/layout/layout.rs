use super::{*, super::*};

pub enum Layout<'a> {
    Solid(Point),
    Stretch(Size),
    Layers(Vec<Layout<'a>>),
    Column(Vec<Layout<'a>>),
    Row(Vec<Layout<'a>>),
    Grid(Vec<(Layout<'a>, Space)>),
    Item(&'a dyn TUI)
}

impl<'a> TUI for Layout<'a> {
    fn layout (&self) -> Layout {
        Layout::Stretch(self.size())
    }
    fn size (&self) -> Size {
        match self {
            Layout::Item(element) => {
                element.size()
            },
            Layout::Solid(point) => {
                Size::from_fixed(*point)
            },
            Layout::Stretch(size) => {
                *size
            },
            Layout::Layers(layers) => {
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
            Layout::Column(elements) => {
                let mut min_w = 0u16;
                let mut min_h = 0u16;
                for element in elements.iter() {
                    let Point(w, h) = element.layout().size().min();
                    min_w = min_w.max(w);
                    min_h += h;
                }
                Size::from_fixed(Point(min_w, min_h))
            },
            Layout::Row(elements) => {
                let mut min_w = 0u16;
                let mut min_h = 0u16;
                for element in elements.iter() {
                    let Point(w, h) = element.layout().size().min();
                    min_w += w;
                    min_h = min_h.max(h);
                }
                Size::from_fixed(Point(min_w, min_h))
            },
            Layout::Grid(_) => {
                unimplemented!()
            }
        }
    }
    fn render (&self, term: &mut dyn Write, space: &Space) -> Result<()> {
        Ok(match self {
            Layout::Solid(_) => {
                ()
            },
            Layout::Stretch(_) => {
                ()
            },
            Layout::Layers(layers) => {
                for layer in layers.iter() {
                    layer.render(term, space)?;
                }
            },
            Layout::Column(elements) => {
                let portion = (space.1.1 / elements.len() as u16).max(1);
                for (index, element) in elements.iter().enumerate() {
                    element.render(term, &Space(
                        Point(space.0.0, space.0.1 + (index as u16 + 0) * portion),
                        Point(space.1.0, portion)
                    ))?
                }
            },
            Layout::Row(elements) => {
                let portion = (space.1.0 / elements.len() as u16).max(1);
                for (index, element) in elements.iter().enumerate() {
                    element.render(term, &Space(
                        Point(space.0.0 + (index as u16 + 0) * portion, space.0.1),
                        Point(portion, space.1.1)
                    ))?
                }
            },
            Layout::Grid(_) => {
                unimplemented!()
            },
            Layout::Item(element) => {
                element.render(term, space)?
            }
        })
    }
}

//#[derive(Default, Clone)]
//pub struct Layout<'a> {
    //width:    Unit,
    //height:   Unit,
    //elements: Vec<(Space, &'a dyn TUI)>
//}

//impl<'a> Layout<'a> {
    //pub fn grid (width: Unit, height: Unit, elements: Vec<(Space, &'a dyn TUI)>) -> Self {
        //Self { width, height, elements }
    //}
    //pub fn column (rows: &[(Unit, &'a dyn TUI)]) -> Self {
        //let mut elements = vec![];
        //let mut y = 0;
        //for (height, element) in rows {
            //elements.push((Space(Point(0, y), Point(1, *height)), *element));
            //y += height;
        //}
        //Self::grid(1, y, elements)
    //}
    //pub fn row (columns: &[(Unit, &'a dyn TUI)]) -> Self {
        //let mut elements = vec![];
        //let mut x = 0;
        //for (width, element) in columns {
            //elements.push((Space(Point(x, 0), Point(*width, 1)), *element));
            //x += width;
        //}
        //Self::grid(x, 1, elements)
    //}
    //pub fn render (&self, term: &mut dyn Write, space: &Space) -> Result<()> {
        //let Space(Point(x_offset, y_offset), Point(w_total, h_total)) = *space;
        //let Layout { width, height, elements } = self;
        //for (portion, element) in elements.iter() {
            //let Space(Point(x, y), Point(w, h)) = *portion;
            //let x = x_offset + x * w_total / width;
            //let y = y_offset + y * h_total / height;
            //let w = w * w_total / width;
            //let h = h * h_total / height;
            //element.render(term, &Space(Point(x, y), Point(w, h)))?;
        //}
        //Ok(())
    //}
//}
