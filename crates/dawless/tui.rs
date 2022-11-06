use std::{error::Error, io};
use cursive::Cursive;
use cursive::views::*;
use cursive::traits::*;
use cursive::theme::*;

lazy_static::lazy_static! {
    static ref DEVICES: Vec<(
        &'static str,
        fn (&mut Cursive)
        //Box<(dyn (Fn()->Box<dyn DeviceTui>) + Sync)>
        //&'static Vec<(KeyCode, &'static str, Option<Box<(dyn Fn() + Sync)>>)>
    )> = vec![
        ("Korg Electribe 2", dawless_korg::electribe_2_tui), // Box::new(dawless_korg::e2_tui_init))
        //("AKAI S3000XL",     Box::new(|_|{})), // Box::new(dawless_korg::e2_tui_init))
        //(dawless_akai::AKAI_S3000XL_TUI.0, (*dawless_akai::AKAI_S3000XL_TUI.1).to_vec()),
        //(dawless_korg::ELECTRIBE_2_TUI.0, &dawless_korg::ELECTRIBE_2_TUI.1),
        //("Korg Triton", vec![
            //(KeyCode::F(1), "Import program", None),
            //(KeyCode::F(2), "Import combi",   None),
            //(KeyCode::F(3), "Import multi",   None),
        //])
    ];
}

pub(crate) fn main() -> Result<(), Box<dyn Error>> {
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());
    set_theme(&mut siv);
    add_device_menu(&mut siv);
    siv.run();
    Ok(())
}

fn set_theme (siv: &mut Cursive) {
    siv.set_theme(cursive::theme::Theme {
        shadow: true,
        borders: BorderStyle::Outset,
        palette: Palette::default().with(|palette| {
            use cursive::theme::BaseColor::*;
            use cursive::theme::Color::*;
            use cursive::theme::PaletteColor::*;
            palette[Background]   = Black.dark();
            palette[View]         = Black.light();
            palette[Primary]      = White.light();
            palette[Secondary]    = White.light();
            palette[Tertiary]     = White.light();
            palette[TitlePrimary] = White.light();
            palette[Highlight]    = Black.dark();
        }),
    });
}

fn add_device_menu (siv: &mut Cursive) {
    let mut buttons = LinearLayout::vertical().child(DummyView);
    for device in DEVICES.iter() {
        buttons = buttons.child(Button::new(device.0, device.1))
    }
    buttons = buttons.child(DummyView).child(Button::new("Quit", Cursive::quit));
    let dialog = Dialog::around(buttons).title("Dawless");
    siv.add_layer(dialog);
}

struct MainTui {
    //terminal: &'a mut Terminal<B>,
    //state:    ListState,
    //items:    Vec<ListItem<'a>>,
    //commands: Vec<(KeyCode, &'static str)>,
    //devices:  Vec<Box<dyn DeviceTui>>
}

impl MainTui {

    pub fn new () -> Self {
        //let mut this = Self { ..Default::default() };
        //for (index, (name, commands)) in DEVICES.iter().enumerate() {
            //this.items.push(ListItem::new(vec![Spans::from(*name)]));
            //this.devices.push(commands())
        //}
        Self {}
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

    fn render (&mut self) {
        //self.terminal.draw(|f| {
            //let (panels, command_area) = layout(&f.size());
            //let items = List::new(self.items.clone())
                //.block(Block::default().borders(Borders::RIGHT).title("Devices"))
                //.highlight_style(Style::default().fg(Color::LightGreen).add_modifier(Modifier::BOLD))
                //.highlight_symbol(">> ");
            //f.render_stateful_widget(items, panels[0], &mut self.state);
            //if let Some(selected) = self.state.selected() {
                //let device = &self.devices[selected];
                //for (widget, area) in device.render(panels[1]) {
                    //f.render_widget(**widget, *area);
                //}
                ////let mut commands = String::from("");
                ////for command in DEVICES[selected].1.iter() {
                    ////match command.0 {
                        ////KeyCode::F(n) => commands.push_str(&format!("F{n} ")),
                        ////_ => {}
                    ////};
                    ////commands.push_str(command.1);
                    ////commands.push_str("  ")
                ////};
                ////let commands = Paragraph::new(commands)
                    ////.block(Block::default().borders(Borders::TOP).title("Commands"));
                ////f.render_widget(commands, command_area);
            //}
        //}).unwrap();
    }

    fn handle (&mut self) -> bool {
        //if crossterm::event::poll(Duration::from_millis(16)).unwrap() {
            //if let Event::Key(key) = event::read().unwrap() {
                //match key.code {
                    //KeyCode::Char('q') => return true,
                    //KeyCode::Down => { self.device_next() },
                    //KeyCode::Up   => { self.device_prev() },
                    //KeyCode::F(n) => { self.device_command(n) }
                    //_ => {}
                //}
            //}
        //}
        return false
    }

    fn device_next (&mut self) {
        //self.state.select(Some(next(self.items.len(), self.state.selected())));
    }

    fn device_prev (&mut self) {
        //self.state.select(Some(prev(self.items.len(), self.state.selected())));
    }

    fn device_command (&mut self, n: u8) {
        //if let Some(selected) = self.state.selected() {
            //for command in DEVICES[selected].1.iter() {
                //if command.0 == KeyCode::F(n) {
                    //if let Some(command) = &command.2 {
                        //command()
                    //}
                //}
            //}
        //}
    }

}

//fn layout (size: &Rect) -> (Vec<Rect>, Rect) {
    //let mut size = size.clone();
    //size.height -= 2;
    //let panels = Layout::default()
        //.direction(Direction::Horizontal)
        //.constraints([
            //Constraint::Percentage(10),
            //Constraint::Percentage(10),
            //Constraint::Percentage(40),
            //Constraint::Percentage(40)
        //].as_ref())
        //.split(size);
    //let commands = Rect { x: size.x, y: size.height, width: size.width, height: 2 };
    //(panels, commands)
//}

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
