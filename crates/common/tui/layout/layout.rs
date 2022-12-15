use super::{*, super::*};

pub enum Layout<'a> {
    Solid(Point),
    Column(Vec<&'a dyn TUI>),
    Row(Vec<&'a dyn TUI>),
    Grid(Vec<(&'a dyn TUI, Space)>)
}

impl<'a> Layout<'a> {
    pub fn min_size (&self) -> Point {
        Point(0, 0)
    }
    pub fn render (&self, term: &mut dyn Write, space: &Space) -> Result<()> {
        Ok(())
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
