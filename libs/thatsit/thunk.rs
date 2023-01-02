/// DEPRECATED

struct Thunk<'l>(ThunkFn<'l>, Vec<&'l Thunk<'l>>);//Box<dyn Fn(&mut dyn Write, Area)->Result<()> + 'l>;

fn foo () -> impl TUI {
}

impl<'l> Thunk<'l> {
    pub const NIL: Self = Self(nil, vec![]);
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        (self.0)(self.1.as_slice(), term, area)
    }
}

impl<'l, T: TUI> From<T> for Thunk<'l> {
    fn from (tui: T) -> Thunk<'l> {
        Thunk(render, vec![tui])
    }
}

struct ThunkWrapper<'l>(Thunk<'l>);

impl<'l> TUI for ThunkWrapper<'l> {
    fn handle (&mut self, event: Event) -> Result<bool> {
        unreachable!();
    }
    fn layout <'m> (&self, max: Size) -> Thunk<'m> {
        self.0
    }
    fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        self.0.render(term, area)
    }
}

pub fn thunk <'l> (render: ThunkFn<'l>, items: &[&'l dyn TUI]) -> Thunk<'l> {
    let thunks: Vec<&'l dyn TUI> = vec![];
    for item in items.iter() {
        thunks.push(item);
    }
    let items = items.iter().map(|item|Thunk::from(item)).collect::<Vec<_>>();
    Thunk(render, items)
}

fn nil (thunks: &[&dyn TUI], term: &mut dyn Write, area: Area) -> Result<()> {
    Ok(())
}

fn render (thunks: &[&dyn TUI], term: &mut dyn Write, area: Area) -> Result<()> {
    thunks[0].render(term, area);
    Ok(())
}

fn row (thunks: &[&dyn TUI], term: &mut dyn Write, area: Area) -> Result<()> {
    Ok(())
}

fn column (thunks: &[&dyn TUI], term: &mut dyn Write, area: Area) -> Result<()> {
    Ok(())
}

/// A leaf of the layout tree, containing either a widget or a thunk,
/// alongside sizing, padding, and scrolling preferences.
pub enum ThunkItem<'a> {
    /// A reference to a single widget.
    Ref(&'a dyn TUI),
    /// An owned single widget.
    Box(Box<dyn TUI>),
    /// An owned single widget.
    Fn(Box<dyn Fn(&mut dyn Write, Area)->Result<()> + 'a>),
}

impl<'a> ThunkItem<'a> {
    pub fn render (&self, term: &mut dyn Write, area: Area) -> Result<()> {
        match &self {
            Self::Ref(item) => item.render(term, area),
            Self::Box(item) => item.render(term, area),
            Self::Fn(item)  => item(term, area),
            //Self::Thunk(thunk) => thunk.render(term, area)
        }
    }
}
