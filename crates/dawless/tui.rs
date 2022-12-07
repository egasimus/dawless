use std::io::{Result, Write};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}, mpsc::channel};
use dawless_common::{TUI, draw_box};
use dawless_korg::KorgElectribe2TUI;
use crossterm::{
    execute, queue,
    event::{poll, read, Event, KeyEvent, KeyCode},
    style::{Print, Color, ResetColor, SetForegroundColor, SetBackgroundColor},
    cursor::{Show, MoveTo},
    terminal::{
        size, enable_raw_mode, disable_raw_mode,
        EnterAlternateScreen, LeaveAlternateScreen
    }
};

pub(crate) fn main() -> Result<()> {

    let (tx, rx) = channel::<Event>();
    let exit = Arc::new(AtomicBool::new(false));
    let exit_thread = Arc::clone(&exit);

    std::thread::spawn(move || {
        let mut out = std::io::stdout();
        execute!(out, EnterAlternateScreen)?;
        enable_raw_mode()?;
        let mut tui = RootTUI::new(exit_thread.clone());
        loop {
            if exit_thread.fetch_and(true, Ordering::Relaxed) == true {
                break
            }
            let (cols, rows) = size()?;
            tui.render(0, 0, cols, rows)?;
            tui.update(rx.recv().unwrap())?;
        }
        execute!(out, ResetColor, Show, LeaveAlternateScreen)?;
        disable_raw_mode()
    });

    loop {
        if exit.fetch_and(true, Ordering::Relaxed) == true {
            break
        }
        if poll(std::time::Duration::from_millis(100))? {
            if let Err(_) = tx.send(read()?) {
                break
            }
        }
    }

    Ok(())

}

struct RootTUI <'a> {
    exited:  Arc<AtomicBool>,
    devices: Vec<(&'static str, Box<&'a dyn TUI>)>,
    device:  usize
}

impl <'a> RootTUI <'a> {
    fn new (exited: Arc<AtomicBool>) -> Self {
        RootTUI {
            exited,
            devices: vec![
                ("Korg Electribe",      Box::new(&KorgElectribe2TUI {})),
                ("Korg Triton",         Box::new(&KorgElectribe2TUI {})),
                ("AKAI S3000XL",        Box::new(&KorgElectribe2TUI {})),
                ("AKAI MPC2000",        Box::new(&KorgElectribe2TUI {})),
                ("iConnectivity mioXL", Box::new(&KorgElectribe2TUI {}))
            ],
            device: 0
        }
    }
}

impl <'a> TUI for RootTUI <'a> {
    fn render (&self, col1: u16, row1: u16, _cols: u16, _rows: u16) -> Result<()> {
        use crossterm::{cursor::{Hide}, terminal::{Clear, ClearType}};
        let mut out = std::io::stdout();
        queue!(out, ResetColor, Clear(ClearType::All), Hide)?;
        let bg = Color::AnsiValue(232);
        let fg = Color::White;
        let hi = Color::Yellow;
        draw_box(&mut out, col1 + 1, row1, 21, 9, bg, Some((bg, fg, "Devices")))?;
        for (index, device) in self.devices.iter().enumerate() {
            queue!(out,
                SetBackgroundColor(bg),
                SetForegroundColor(if index == self.device { hi } else { fg }),
                MoveTo(col1 + 1, row1 + 2 + (index as u16)),
                Print(format!(" {:<19} ", device.0))
            )?;
            if index == self.device {
                device.1.render(col1 + 23, row1, 50, 30)?;
            }
        }
        out.flush()?;
        Ok(())
    }
    fn update (&mut self, event: Event) -> Result<()> {
        match event {
            Event::Key(KeyEvent { code: KeyCode::Char('q'), .. }) => {
                self.exited.store(true, Ordering::Relaxed);
            },
            _ => {}
        }
        Ok(())
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
