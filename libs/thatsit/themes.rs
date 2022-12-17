use crate::*;

/// A selection of colors to use for rendering
#[derive(Copy, Clone, Debug)]
pub struct Theme {
    pub bg: Color,
    pub fg: Color,
    pub hi: Color
}

impl Default for Theme {
    fn default () -> Self {
        Theme {
            bg: Color::AnsiValue(232),
            fg: Color::White,
            hi: Color::Yellow
        }
    }
}
