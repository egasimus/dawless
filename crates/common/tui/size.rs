#[derive(Default, Clone, Debug)]
pub struct Size {
    pub min_w: Option<u16>,
    pub max_w: Option<u16>,
    pub min_h: Option<u16>,
    pub max_h: Option<u16>
}

impl std::ops::Add for Size {
    type Output = Self;
    fn add (self, other: Self) -> Self {
        Self {
            min_w: match (self.min_w, other.min_w) {
                (Some(w1), Some(w2)) => Some(w1 + w2),
                (Some(w1), None)     => Some(w1),
                (None,     Some(w2)) => Some(w2),
                (None,     None)     => None
            },
            max_w: match (self.max_w, other.max_w) {
                (Some(w1), Some(w2)) => Some(w1 + w2),
                (Some(__), None)     => None,
                (None,     Some(__)) => None,
                (None,     None)     => None
            },
            min_h: match (self.min_h, other.min_h) {
                (Some(h1), Some(h2)) => Some(h1 + h2),
                (Some(h1), None)     => Some(h1),
                (None,     Some(h2)) => Some(h2),
                (None,     None)     => None
            },
            max_h: match (self.max_h, other.max_h) {
                (Some(h1), Some(h2)) => Some(h1 + h2),
                (Some(__), None)     => None,
                (None,     Some(__)) => None,
                (None,     None)     => None
            }
        }
    }
}
