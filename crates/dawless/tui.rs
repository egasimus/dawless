use std::io::Result;

fn fill <W: std::io::Write> (
    out: &mut W, color: crossterm::style::Color, col1: u16, row1: u16, cols: u16, rows: u16
) -> Result<()> {
    use crossterm::queue;
    use crossterm::style::{Print, SetBackgroundColor};
    use crossterm::cursor::MoveTo;
    queue!(out, SetBackgroundColor(color))?;
    for row in row1..row1+rows {
        for col in col1..col1+cols {
            queue!(out, MoveTo(col, row), Print(" "))?;
        }
    }
    Ok(())
}

fn frame <W: std::io::Write> (
    out: &mut W, col1: u16, row1: u16, cols: u16, rows: u16
) -> Result<()> {
    use crossterm::queue;
    use crossterm::style::Print;
    use crossterm::cursor::MoveTo;
    queue!(out, MoveTo(col1, row1),               Print("▄"))?;
    queue!(out, MoveTo(col1 + cols, row1),        Print("▄"))?;
    queue!(out, MoveTo(col1, row1 + rows),        Print("▀"))?;
    queue!(out, MoveTo(col1 + cols, row1 + rows), Print("▀"))?;
    for i in col1+1..col1+cols {
        queue!(out, MoveTo(i, row1),              Print("▄"))?;
        queue!(out, MoveTo(i, row1 + rows),       Print("▀"))?;
    }
    for i in row1+1..row1+rows {
        queue!(out, MoveTo(col1,        i),       Print("█"))?;
        queue!(out, MoveTo(col1 + cols, i),       Print("█"))?;
    }
    Ok(())
}

fn frame_bg <W: std::io::Write> (
    out: &mut W, color: crossterm::style::Color, col1: u16, row1: u16, cols: u16, rows: u16
) -> Result<()> {
    use crossterm::queue;
    use crossterm::style::ResetColor;
    queue!(out, ResetColor);
    frame(out, col1, row1, cols, rows)?;
    fill(out, color, col1 + 1, row1 + 1, cols - 1, rows - 1)?;
    Ok(())
}

trait TUI: Sync {
    fn run (&self, col: u16, row: u16) -> Result<()> {
        loop {
            let (cols, rows) = crossterm::terminal::size()?;
            self.render(col, row, cols - col, rows - row)?;
            if self.update()? {
                break
            };
        }
        Ok(())
    }
    fn render (&self, col1: u16, row1: u16, cols: u16, rows: u16) -> Result<()> {
        Ok(())
    }
    fn update (&self) -> Result<bool> {
        Ok(true)
    }
}

struct RootTUI <'a> {
    devices: Vec<(&'static str, Box<&'a dyn TUI>)>
}

impl <'a> RootTUI <'a> {
    fn new () -> Self {
        RootTUI {
            devices: vec![
                ("Korg Electribe", Box::new(&KorgElectribeTUI {}))
            ]
        }
    }
}

impl <'a> TUI for RootTUI <'a> {
    fn render (&self, col1: u16, row1: u16, cols: u16, rows: u16) -> Result<()> {
        use crossterm::{queue};
        use crossterm::style::{ResetColor, Color, Print};
        use crossterm::cursor::{Hide, MoveTo, MoveToNextLine};
        use crossterm::terminal::{Clear, ClearType};
        let mut out = std::io::stdout();
        queue!(out, ResetColor, Clear(ClearType::All), Hide)?;
        queue!(out, MoveTo(col1, row1))?;
        frame_bg(&mut out, Color::AnsiValue(17), col1 + 1, row1, 20, 40)?;
        frame_bg(&mut out, Color::AnsiValue(16), col1 + 23, row1, 40, 10)?;
        //for _ in 0..cols {
            //queue!(out, Print("|"), MoveToNextLine(1))?;
        //}
        Ok(())
    }
    fn update (&self) -> Result<bool> {
        match read_char()? {
            'q' => {
                return Ok(true)
            },
            _ => {}
        };
        Ok(false)
    }
}

struct KorgElectribeTUI {}

impl TUI for KorgElectribeTUI {}

struct AkaiMPCTUI {}

impl TUI for AkaiMPCTUI {}

pub(crate) fn main() -> Result<()> {
    use crossterm::style::{ResetColor};
    use crossterm::cursor::{Show};
    use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
    let mut out = std::io::stdout();
    crossterm::execute!(out, EnterAlternateScreen)?;
    crossterm::terminal::enable_raw_mode()?;
    RootTUI::new().run(0, 0)?;
    crossterm::execute!(out, ResetColor, Show, LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()
}

fn read_char() -> Result<char> {
    use crossterm::event::{read, Event, KeyEvent, KeyCode};
    loop {
        if let Ok(Event::Key(KeyEvent { code: KeyCode::Char(c), .. })) = read() {
            return Ok(c);
        }
    }
}

//use std::{error::Error, io};
//use cursive::Cursive;
//use cursive::views::*;
//use cursive::traits::*;
//use cursive::theme::*;


        //("Korg Electribe 2", dawless_korg::electribe_2_tui), // Box::new(dawless_korg::e2_tui_init))
        //("AKAI S3000XL",     Box::new(|_|{})), // Box::new(dawless_korg::e2_tui_init))
        //(dawless_akai::AKAI_S3000XL_TUI.0, (*dawless_akai::AKAI_S3000XL_TUI.1).to_vec()),
        //(dawless_korg::ELECTRIBE_2_TUI.0, &dawless_korg::ELECTRIBE_2_TUI.1),
        //("Korg Triton", vec![
            //(KeyCode::F(1), "Import program", None),
            //(KeyCode::F(2), "Import combi",   None),
            //(KeyCode::F(3), "Import multi",   None),
        //])
    //];
//}
//

//fn set_theme (siv: &mut Cursive) {
    //siv.set_theme(cursive::theme::Theme {
        //shadow: true,
        //borders: BorderStyle::Outset,
        //palette: Palette::default().with(|palette| {
            //use cursive::theme::BaseColor::*;
            //use cursive::theme::Color::*;
            //use cursive::theme::PaletteColor::*;
            //palette[Background]   = Black.dark();
            //palette[View]         = Black.light();
            //palette[Primary]      = White.light();
            //palette[Secondary]    = White.light();
            //palette[Tertiary]     = White.light();
            //palette[TitlePrimary] = White.light();
            //palette[Highlight]    = Black.dark();
        //}),
    //});
//}

//fn add_device_menu (siv: &mut Cursive) {
    //let mut buttons = LinearLayout::vertical().child(DummyView);
    //for device in DEVICES.iter() {
        //buttons = buttons.child(Button::new(device.0, device.1))
    //}
    //buttons = buttons.child(DummyView).child(Button::new("Quit", Cursive::quit));
    //let dialog = Dialog::around(buttons).title("Dawless");
    //siv.add_layer(dialog);
//}

//struct MainTui {
    ////terminal: &'a mut Terminal<B>,
    ////state:    ListState,
    ////items:    Vec<ListItem<'a>>,
    ////commands: Vec<(KeyCode, &'static str)>,
    ////devices:  Vec<Box<dyn DeviceTui>>
//}

//impl MainTui {

    //pub fn new () -> Self {
        ////let mut this = Self { ..Default::default() };
        ////for (index, (name, commands)) in DEVICES.iter().enumerate() {
            ////this.items.push(ListItem::new(vec![Spans::from(*name)]));
            ////this.devices.push(commands())
        ////}
        //Self {}
    //}

    //pub fn run (mut self) -> io::Result<()>  {
        //loop {
            //self.render();
            //let done = self.handle();
            //if done {
                //return Ok(())
            //}
        //}
    //}

    //fn render (&mut self) {
        ////self.terminal.draw(|f| {
            ////let (panels, command_area) = layout(&f.size());
            ////let items = List::new(self.items.clone())
                ////.block(Block::default().borders(Borders::RIGHT).title("Devices"))
                ////.highlight_style(Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD))
                ////.highlight_symbol(">> ");
            ////f.render_stateful_widget(items, panels[0], &mut self.state);
            ////if let Some(selected) = self.state.selected() {
                ////let device = &self.devices[selected];
                ////for (widget, area) in device.render(panels[1]) {
                    ////f.render_widget(**widget, *area);
                ////}
                //////let mut commands = String::from("");
                //////for command in DEVICES[selected].1.iter() {
                    //////match command.0 {
                        //////KeyCode::F(n) => commands.push_str(&format!("F{n} ")),
                        //////_ => {}
                    //////};
                    //////commands.push_str(command.1);
                    //////commands.push_str("  ")
                //////};
                //////let commands = Paragraph::new(commands)
                    //////.block(Block::default().borders(Borders::TOP).title("Commands"));
                //////f.render_widget(commands, command_area);
            ////}
        ////}).unwrap();
    //}

    //fn handle (&mut self) -> bool {
        ////if crossterm::event::poll(Duration::from_millis(16)).unwrap() {
            ////if let Event::Key(key) = event::read().unwrap() {
                ////match key.code {
                    ////KeyCode::Char('q') => return true,
                    ////KeyCode::Down => { self.device_next() },
                    ////KeyCode::Up   => { self.device_prev() },
                    ////KeyCode::F(n) => { self.device_command(n) }
                    ////_ => {}
                ////}
            ////}
        ////}
        //return false
    //}

    //fn device_next (&mut self) {
        ////self.state.select(Some(next(self.items.len(), self.state.selected())));
    //}

    //fn device_prev (&mut self) {
        ////self.state.select(Some(prev(self.items.len(), self.state.selected())));
    //}

    //fn device_command (&mut self, n: u8) {
        ////if let Some(selected) = self.state.selected() {
            ////for command in DEVICES[selected].1.iter() {
                ////if command.0 == KeyCode::F(n) {
                    ////if let Some(command) = &command.2 {
                        ////command()
                    ////}
                ////}
            ////}
        ////}
    //}

//}

////fn layout (size: &Rect) -> (Vec<Rect>, Rect) {
    ////let mut size = size.clone();
    ////size.height -= 2;
    ////let panels = Layout::default()
        ////.direction(Direction::Horizontal)
        ////.constraints([
            ////Constraint::Percentage(10),
            ////Constraint::Percentage(10),
            ////Constraint::Percentage(40),
            ////Constraint::Percentage(40)
        ////].as_ref())
        ////.split(size);
    ////let commands = Rect { x: size.x, y: size.height, width: size.width, height: 2 };
    ////(panels, commands)
////}

//fn next (len: u16, selected: Option<u16>) -> u16 {
    //match selected {
        //Some(i) => {
            //if i >= len - 1 {
                //0
            //} else {
                //i + 1
            //}
        //}
        //None => 0,
    //}
//}

//fn prev (len: u16, selected: Option<u16>) -> u16 {
    //match selected {
        //Some(i) => {
            //if i == 0 {
                //len - 1
            //} else {
                //i - 1
            //}
        //}
        //None => 0,
    //}
//}
