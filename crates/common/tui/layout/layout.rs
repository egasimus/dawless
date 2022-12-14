use super::{*, super::*};

#[derive(Default, Clone)]
pub struct Layout<'a> {
    elements: Vec<(Space, &'a dyn TUI)>
}

impl<'a> Layout<'a> {
    pub fn grid (elements: Vec<(Space, &'a dyn TUI)>) -> Self {
        Self { elements }
    }
    pub fn column (rows: &[(u16, &'a dyn TUI)]) -> Self {
        let mut elements = vec![];
        let mut y = 0;
        for (height, element) in rows {
            elements.push((Space(Point(0, y), Point(1, *height)), *element));
            y += height;
        }
        Self::grid(elements)
    }
    pub fn row (columns: &[(u16, &'a dyn TUI)]) -> Self {
        let mut elements = vec![];
        let mut x = 0;
        for (width, element) in columns {
            elements.push((Space(Point(x, 0), Point(*width, 1)), *element));
            x += width;
        }
        Self::grid(elements)
    }
    pub fn render (&self, term: &mut dyn Write, space: &Space) -> Result<()> {
        let Space(Point(x_offset, y_offset), Point(w_total, h_total)) = *space;
        for (portion, element) in self.elements.iter() {
            let Space(Point(x, y), Point(w, h)) = *portion;
            element.render(term, &Space(
                Point(x_offset + x/w_total, y_offset + y/h_total),
                Point(w/w_total, h/h_total),
            ))?;
        }
        Ok(())
    }
}
