use std::cell::RefCell;
use std::io::{Result, Write};
use std::sync::{atomic::{AtomicBool, Ordering}, mpsc::channel};
use thatsit::{*, crossterm::{
    event::{poll, read, Event, KeyEvent, KeyCode},
    terminal::{size},
    style::Color
}};

static THEME: Theme = Theme {
    bg: Color::AnsiValue(232),
    fg: Color::White,
    hi: Color::Yellow
};

static EXITED: AtomicBool = AtomicBool::new(false);

thread_local!(static APP: RefCell<App> = RefCell::new(App {
    exited:  &EXITED,
    focused: true,
    frame:   Frame { theme: THEME, title: "Select device:".into(), focused: true, ..Frame::default() },
    menu:    List { theme: THEME, ..List::default() },
    open:    false
}));

struct App {
    exited:  &'static AtomicBool,
    focused: bool,
    frame:   Frame,
    menu:    List<Box<dyn TUI>>,
    open:    bool
}

pub(crate) fn main () -> Result<()> {

    // Init app features
    APP.with(|app| {
        app.borrow_mut().menu
            .add("Korg Electribe",      Box::new(dawless_korg::electribe2::Electribe2TUI::new()))
            .add("Korg Triton",         Box::new(dawless_korg::triton::TritonTUI::new()))
            .add("AKAI S3000XL",        Box::new(dawless_akai::S3000XLTUI::new()))
            .add("AKAI MPC2000",        Box::new(dawless_akai::MPC2000TUI::new()))
            .add("iConnectivity mioXL", Box::new(dawless_iconnectivity::MioXLTUI::new()));
    });

    // Set up event channel
    let (tx, rx) = channel::<Event>();

    // Spawn IO thread
    std::thread::spawn(move || {
        loop {
            if EXITED.fetch_and(true, Ordering::Relaxed) == true { break }
            if poll(std::time::Duration::from_millis(100)).is_ok() {
                if tx.send(read().unwrap()).is_err() { break }
            }
        }
    });

    // Setup terminal
    let mut term = std::io::stdout();
    setup(&mut term)?;

    // Setup panic hook
    std::panic::set_hook(Box::new(|panic_info| {
        teardown(&mut std::io::stdout()).unwrap();
        better_panic::Settings::auto().create_panic_handler()(panic_info);
    }));

    // Render app and listen for updates
    loop {
        let mut done = false;
        APP.with(|app| {

            // Clear screen
            clear(&mut term).unwrap();

            // Get screen size
            let screen_size: Size = size().unwrap().into();

            // Render to output buffer
            {
                let app = app.borrow();
                if app.exited.fetch_and(true, Ordering::Relaxed) == true {
                    done = true;
                    return
                }
                let layout = app.layout();
                let min_size = layout.min_size();
                if let Err(e) = min_size.fits_in(screen_size) {
                    write_error(&mut term, format!("{e}").as_str()).unwrap();
                } else {
                    let size = screen_size.crop_to(match layout.size() {
                        Some(size) => size,
                        None => layout.max_size()
                    });
                    let xy = Point((screen_size.0 - size.0) / 2, (screen_size.1 - size.1) / 2);
                    //write_text(&mut term, 0, 0, &format!("{screen_size} {size} {xy}")).unwrap();
                    if let Err(e) = app.render(&mut term, Area(xy, size)) {
                        write_error(&mut term, format!("{e}").as_str()).unwrap();
                    };
                }
            };

            // Flush output buffer
            term.flush().unwrap();

            // Wait for input and update
            app.borrow_mut().handle(&rx.recv().unwrap()).unwrap();

        });
        if done {
            break
        }
    }

    // Clean up
    teardown(&mut term)?;

    Ok(())
}

impl App {
    fn exit (&mut self) {
        self.exited.store(true, Ordering::Relaxed);
    }
    fn device <'a> (&'a self) -> &'a dyn TUI {
        &**self.menu.get().unwrap()
    }
    fn device_mut <'a> (&'a mut self) -> &'a mut dyn TUI {
        &mut **self.menu.get_mut().unwrap()
    }
}

impl TUI for App {
    fn layout (&self) -> Layout {
        Layout::Row(Sizing::Min, vec![
            //Layout::Item(Sizing::Min, &DebugBox { bg: Color::AnsiValue(100) }),
            //Layout::Item(Sizing::Min, &DebugBox { bg: Color::AnsiValue(125) }),
            //Layout::Column(Sizing::Min, vec![
                //Layout::Item(Sizing::Min, &DebugBox { bg: Color::AnsiValue(150) }),
                //Layout::Item(Sizing::Min, &DebugBox { bg: Color::AnsiValue(175) }),
            //]),
            Layout::Layers(Sizing::Min, vec![
                //Layout::Item(Sizing::Min, &DebugBox { bg: Color::AnsiValue(100) }),
                Layout::Item(Sizing::AUTO, &self.frame),
                Layout::Item(Sizing::Pad(1, &Sizing::AUTO), &self.menu)
            ]),
            if self.open {
                Layout::Item(Sizing::Min, self.device())
            } else {
                Layout::None
            }
        ])
    }
    fn focus (&mut self, focus: bool) -> bool {
        self.focused = focus;
        self.frame.focused = focus;
        true
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        if let Event::Key(KeyEvent { code: KeyCode::Char('q'), .. }) = event {
            self.exit();
            return Ok(true)
        }
        if !self.focused {
            if self.device_mut().handle(&event)? {
                return Ok(true)
            }
        }
        if self.menu.handle(event)? {
            return Ok(true)
        }
        let result = handle_menu_focus!(event, self, self.device_mut(), self.focused);
        if let Ok(true) = result {
            self.open = !self.focused
        }
        result
    }
}
