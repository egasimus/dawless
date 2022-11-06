use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Corner, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame, Terminal,
};

pub(crate) fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    // create app and run it
    let res = DawlessTUI::new(&mut terminal).run();
    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    if let Err(err) = res {
        println!("{:?}", err)
    }
    Ok(())
}

struct DawlessTUI <'a, B: Backend> {
    terminal: &'a mut Terminal<B>,
    devices:  DeviceList<'a>
}

impl<'a, B: Backend> DawlessTUI<'a, B> {

    pub fn new (terminal: &'a mut Terminal<B>) -> Self {
        Self {
            terminal,
            devices: DeviceList::new()
        }
    }

    pub fn run (mut self) -> io::Result<()>  {
        loop {
            self.render();
            let done = self.handle();
            if done {
                return Ok(())
            }
        }
    }

    fn handle (&mut self) -> bool {
        if crossterm::event::poll(Duration::from_millis(16)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Char('q') => return true,
                    KeyCode::Down => { self.devices.next() },
                    KeyCode::Up   => { self.devices.previous() },
                    _ => {}
                }
            }
        }
        return false
    }

    fn render (&mut self) {
        self.terminal.draw(|f| {
            let chunks = Self::layout(&f);
            self.devices.render(f, chunks[0]);
        }).unwrap();
    }

    fn layout (f: &Frame<B>) -> Vec<Rect> {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(40),
                Constraint::Percentage(40)
            ].as_ref())
            .split(f.size())
    }

}

#[derive(Default)]
struct DeviceList<'a> {
    state: ListState,
    items: Vec<ListItem<'a>>
}

impl<'a> DeviceList<'a> {
    fn new () -> Self {
        let mut devices = Self::default();
        devices.items.push(ListItem::new(vec![Spans::from("Korg Electribe")]));
        devices.items.push(ListItem::new(vec![Spans::from("  Samples")]));
        devices.items.push(ListItem::new(vec![Spans::from("  Patterns")]));
        devices.items.push(ListItem::new(vec![Spans::from("AKAI S3000XL")]));
        devices.items.push(ListItem::new(vec![Spans::from("  Samples")]));
        devices.items.push(ListItem::new(vec![Spans::from("  Programs")]));
        devices.items.push(ListItem::new(vec![Spans::from("  Multis")]));
        devices.items.push(ListItem::new(vec![Spans::from("Korg Triton")]));
        devices.items.push(ListItem::new(vec![Spans::from("  Programs")]));
        devices.items.push(ListItem::new(vec![Spans::from("  Combis")]));
        devices.items.push(ListItem::new(vec![Spans::from("  Multis")]));
        devices.items.push(ListItem::new(vec![Spans::from("iConnectivity mioXL")]));
        devices.items.push(ListItem::new(vec![Spans::from("  Presets")]));
        devices
    }

    pub fn render <B: Backend> (&mut self, f: &mut Frame<B>, rect: Rect) {
        let items = List::new(self.items.clone())
            .block(Block::default().borders(Borders::RIGHT).title("Devices"))
            .highlight_style(Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ");
        f.render_stateful_widget(items, rect, &mut self.state);
    }

        fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
