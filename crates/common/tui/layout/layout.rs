use super::{*, super::*};

#[derive(Default, Clone)]
pub struct Layout<'a>(Vec<(Space, &'a dyn TUI)>);

impl<'a> Layout<'a> {
    pub fn column (elements: &[(usize, &'a dyn TUI)]) -> Self {
        let layout = vec![];
        Self(layout)
    }
    pub fn row (elements: &[(usize, &'a dyn TUI)]) -> Self {
        let layout = vec![];
        Self(layout)
    }
    pub fn grid (elements: Vec<(Space, &'a dyn TUI)>) -> Self {
        let layout = vec![];
        Self(layout)
    }
}
