use std::cell::RefCell;
use std::io::{Result, Write};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}, mpsc::channel, Mutex};
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
    frame:   Frame { theme: THEME, title: "Default".into(), ..Frame::default() },
    menu:    List { theme: THEME, index: 0, items: vec![] },
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

    // Render app and listen for updates
    loop {
        let mut done = false;
        APP.with(|app| {

            // Clear screen
            clear(&mut term).unwrap();

            // Render to output buffer
            {
                let app = app.borrow();
                if app.exited.fetch_and(true, Ordering::Relaxed) == true {
                    done = true;
                    return
                }
                let (screen_cols, screen_rows) = size().unwrap();
                match app.size().clip(Point(screen_cols, screen_rows)) {
                    Ok(Point(cols, rows)) => {
                        let x = (screen_cols - cols) / 2;
                        let y = (screen_rows - rows) / 2;
                        let space = Space(Point(x, y), Point(cols, rows));
                        if let Err(e) = app.render(&mut term, &space) {
                            write_error(&mut term, format!("{e} {:?}", space).as_str()).unwrap();
                        }
                    },
                    Err(e) => {
                        write_error(&mut term, format!("{e}").as_str()).unwrap();
                    }
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
        Layout::Layers(Sizing::AUTO, vec![
            Layout::Item(Sizing::AUTO, &self.frame),
            Layout::Row(Sizing::AUTO, vec![
                Layout::Item(Sizing::AUTO, &self.menu),
                Layout::Item(Sizing::AUTO, if self.open { self.device() } else { &Blank {} })
            ])
        ])
    }
    fn focus (&mut self, focus: bool) -> bool {
        self.focused = focus;
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
        handle_menu_focus!(event, self, self.device_mut(), self.focused)
    }
}
