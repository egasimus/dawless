use std::cell::RefCell;
use std::io::{Result, Write};
use std::sync::{atomic::{AtomicBool, Ordering}, mpsc::channel};
use thatsit::{*, crossterm::{
    event::{Event, KeyEvent, KeyCode},
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
    focused: true,
    exited: &EXITED,
    menu: DeviceMenu::new(),
    label: Label {
        text: "Devices:".into(),
        ..Label::default()
    },
}));

struct App {
    exited:  &'static AtomicBool,
    focused: bool,
    label:   Label,
    menu:    DeviceMenu,
}

pub(crate) fn main () -> Result<()> {
    // Set up event channel and input thread
    let (tx, rx) = channel::<Event>();
    spawn_input_thread(tx, &EXITED);
    // Setup terminal and panic hook
    let mut term = std::io::stdout();
    setup(&mut term, true)?;
    // Render app and listen for updates
    loop {
        let mut done = false;
        APP.with(|app| {
            // Clear screen
            clear(&mut term).unwrap();
            // Break loop if exited
            if app.borrow().exited.fetch_and(true, Ordering::Relaxed) == true {
                done = true;
                return
            }
            // Check if there is sufficient screen size
            let screen_size: Size = size().unwrap().into();
            let min_size = app.borrow().layout().min_size;
            if let Err(e) = min_size.fits_in(screen_size) {
                write_error(&mut term, format!("{e}").as_str()).unwrap();
            } else {
                // Render to output buffer
                let size = screen_size.crop_to(min_size);
                let xy = Point((screen_size.0 - size.0) / 2, (screen_size.1 - size.1) / 2);
                if let Err(e) = app.borrow().render(&mut term, Area(xy, size)) {
                    write_error(&mut term, format!("{e}").as_str()).unwrap();
                };
            }
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
}

impl TUI for App {
    fn layout <'a> (&'a self) -> Thunk<'a> {
        Outset(1).around(row(|add|{add(&self.menu);}))
    }
    fn focus (&mut self, focus: bool) -> bool {
        self.focused = focus;
        true
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        Ok(
            if let Event::Key(KeyEvent { code: KeyCode::Char('q'), .. }) = event {
                self.exit();
                true
            } else {
                self.menu.handle(event)?
            }
        )
    }
}

use std::{rc::Rc};

struct DeviceMenu {
    buttons: FocusColumn<Button>,
    device:  Rc<RefCell<Option<Box<dyn TUI>>>>
}

impl DeviceMenu {
    fn new () -> Self {
        let device = Rc::new(RefCell::new(None));
        let mut menu = Self {
            buttons: FocusColumn::default(),
            device:  Rc::clone(&device)
        };
        menu.buttons.0.items.push(Button::new(
            "Korg Electribe",
            Some(Box::new(move || {
                device.replace(Some(Box::new(dawless_korg::electribe2::Electribe2TUI::new())));
            }))
        ));
        menu.buttons.0.items.push(Button::new(
            "Korg Triton",
            None
        ));
        menu.buttons.0.items.push(Button::new(
            "AKAI S3000XL",
            None
        ));
        menu.buttons.0.items.push(Button::new(
            "AKAI MPC2000",
            None
        ));
        menu.buttons.0.items.push(Button::new(
            "iConnectivity mioXL",
            None
        ));
        menu
        //.add("Korg Electribe",      Box::new(dawless_korg::electribe2::Electribe2TUI::new()))
        //.add("Korg Triton",         Box::new(dawless_korg::triton::TritonTUI::new()))
        //.add("AKAI S3000XL",        Box::new(dawless_akai::S3000XLTUI::new()))
        //.add("AKAI MPC2000",        Box::new(dawless_akai::MPC2000TUI::new()))
        //.add("iConnectivity mioXL", Box::new(dawless_iconnectivity::MioXLTUI::new()));
    }
}

impl TUI for DeviceMenu {
    fn min_size (&self) -> Size {
        self.buttons.min_size().expand_row(self.device.min_size())
    }
    fn max_size (&self) -> Size {
        self.buttons.max_size().expand_row(self.device.max_size())
    }
    fn layout <'a> (&'a self) -> Thunk<'a> {
        row(|add|{ add(&self.buttons); add(&self.device); })
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        let entered = self.device.borrow().is_some();
        if entered {
            let mut device = self.device.borrow_mut();
            if let Some(device) = &mut *device {
                device.handle(event)
            } else {
                unreachable!()
            }
        } else {
            self.buttons.handle(event)
        }
    }
}
