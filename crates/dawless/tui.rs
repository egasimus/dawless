use std::cell::RefCell;
use std::io::{Result, Write};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}, mpsc::channel, Mutex};
use dawless_common::*;
use crossterm::{
    event::{poll, read, Event, KeyEvent, KeyCode},
    terminal::{size},
    style::Color
};

static THEME: Theme = Theme {
    bg: Color::AnsiValue(232),
    fg: Color::White,
    hi: Color::Yellow
};

static EXITED: AtomicBool = AtomicBool::new(false);

thread_local!(static APP: RefCell<App> = RefCell::new(App {
    exited:  &EXITED,
    focused: true,
    menu:    List { theme: THEME, index: 0, items: vec![] }
}));

struct App {
    exited:  &'static AtomicBool,
    focused: bool,
    menu:    List<Box<dyn TUI>>,
}

pub(crate) fn main () -> Result<()> {

    // Init app features
    APP.with(|app| {
        app.borrow_mut().menu
            .add("Korg Electribe",      Box::new(dawless_korg::Electribe2TUI::new()))
            .add("Korg Triton",         Box::new(dawless_korg::TritonTUI::new()))
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

            // Render
            clear(&mut term).unwrap();
            {
                let app = app.borrow();
                if app.exited.fetch_and(true, Ordering::Relaxed) == true {
                    done = true;
                    return
                }
                let (screen_cols, screen_rows) = size().unwrap();
                let size = app.size().clip(Point(screen_cols, screen_rows)).unwrap();
                let Point(cols, rows) = size;
                let x = (screen_cols - cols) / 2;
                let y = (screen_rows - rows) / 2;
                let space = Space(Point(x, y), size);
                app.render(&mut term, &space).unwrap();
            };
            term.flush().unwrap();

            // Update on input
            app.borrow_mut().handle(&rx.recv().unwrap()).unwrap();

        });
        if done {
            break
        }
    }

    // Clean up
    teardown(&mut term)
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
    fn size (&self) -> Size {
        if self.menu.get().is_some() {
            self.menu.size().add_w(self.device().size())
        } else {
            self.menu.size()
        }.inc_w(2).inc_h(3)
    }
    fn render (&self, term: &mut dyn Write, space: &Space) -> Result<()> {
        let title = format!("Devices");
        Frame { theme: THEME, title, ..Frame::default() }
            .render(term, space)?;
        if self.menu.get().is_some() {
            Layout::row(&[
                (1, &self.menu),
                (1, self.device())
            ]).render(term, &space.inset(1).offset(Point(0, 1)))
        } else {
            Layout::row(&[
                (1, &self.menu),
            ]).render(term, &space.inset(1).offset(Point(0, 1)))
        }
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
