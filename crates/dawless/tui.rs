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
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
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
                    KeyCode::Up   => { self.devices.prev() },
                    KeyCode::F(n) => { self.devices.command(n) }
                    _ => {}
                }
            }
        }
        return false
    }

    fn render (&mut self) {
        self.terminal.draw(|f| {
            let (panels, commands) = Self::layout(&f);
            self.devices.render(f, panels[0], commands);
        }).unwrap();
    }

    fn layout (f: &Frame<B>) -> (Vec<Rect>, Rect) {
        let mut size = f.size().clone();
        size.height -= 2;
        let panels = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(10),
                Constraint::Percentage(40),
                Constraint::Percentage(40)
            ].as_ref())
            .split(size);
        let commands = Rect { x: size.x, y: size.height, width: size.width, height: 2 };
        (panels, commands)
    }

}

lazy_static::lazy_static! {
    static ref DEVICES: Vec<(&'static str, Vec<(KeyCode, &'static str, Option<fn()>)>)> = vec![
        (dawless_akai::AKAI_S3000XL_TUI.0, (*dawless_akai::AKAI_S3000XL_TUI.1).to_vec()),
        (dawless_korg::ELECTRIBE_2_TUI.0, (*dawless_korg::ELECTRIBE_2_TUI.1).to_vec()),
        ("Korg Triton", vec![
            (KeyCode::F(1), "Import program", None),
            (KeyCode::F(2), "Import combi",   None),
            (KeyCode::F(3), "Import multi",   None),
        ])
    ];
}

#[derive(Default)]
struct DeviceList<'a> {
    state:    ListState,
    items:    Vec<ListItem<'a>>,
    commands: Vec<(KeyCode, &'static str)>
}

impl<'a> DeviceList<'a> {
    fn new () -> Self {
        let mut this = Self::default();
        for (index, (name, commands)) in DEVICES.iter().enumerate() {
            this.items.push(ListItem::new(vec![Spans::from(*name)]));
        }
        this
    }

    pub fn render <B: Backend> (&mut self, f: &mut Frame<B>, panel_area: Rect, command_area: Rect) {
        let items = List::new(self.items.clone())
            .block(Block::default().borders(Borders::RIGHT).title("Devices"))
            .highlight_style(Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ");
        f.render_stateful_widget(items, panel_area, &mut self.state);
        if let Some(selected) = self.state.selected() {
            let mut commands = String::from("");
            for command in DEVICES[selected].1.iter() {
                match command.0 {
                    KeyCode::F(n) => commands.push_str(&format!("F{n} ")),
                    _ => {}
                };
                commands.push_str(command.1);
                commands.push_str("  ")
            };
            let commands = Paragraph::new(commands)
                .block(Block::default().borders(Borders::TOP).title("Commands"));
            f.render_widget(commands, command_area);
        }
    }

    fn next (&mut self) {
        self.state.select(Some(next(self.items.len(), self.state.selected())));
    }

    fn prev (&mut self) {
        self.state.select(Some(prev(self.items.len(), self.state.selected())));
    }

    fn command (&mut self, n: u8) {
        if let Some(selected) = self.state.selected() {
            for command in DEVICES[selected].1.iter() {
                if command.0 == KeyCode::F(n) {
                    panic!("{}", command.1)
                }
            }
        }
    }
}

fn next (len: usize, selected: Option<usize>) -> usize {
    match selected {
        Some(i) => {
            if i >= len - 1 {
                0
            } else {
                i + 1
            }
        }
        None => 0,
    }
}

fn prev (len: usize, selected: Option<usize>) -> usize {
    match selected {
        Some(i) => {
            if i == 0 {
                len - 1
            } else {
                i - 1
            }
        }
        None => 0,
    }
}
