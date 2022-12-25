use std::cell::RefCell;
use std::io::{Result, Write};
use std::sync::{atomic::{AtomicBool, Ordering}, mpsc::channel};
use thatsit::{*, crossterm::{event::{Event, KeyEvent, KeyCode}, terminal::{size}}};

/// Exit flag. Setting this to true terminates the main loop.
static EXITED: AtomicBool = AtomicBool::new(false);

/// The main app object, containing a menu of supported devices.
struct App {
    /// A reference to the exit flag to end the main loop.
    exited:  &'static AtomicBool,
    /// A tabbed collection of supported devices.
    devices: TabbedVertical<Box<dyn TUI>>,
}

thread_local!(
    /// A global instance of the app object, owned by the render thread.
    static APP: RefCell<App> = RefCell::new(App {
        exited:  &EXITED,
        devices: TabbedVertical::new(vec![
            (Button::new("Korg Electribe 2",    None),
                Box::new(dawless_korg::electribe2::Electribe2TUI::new())),
            (Button::new("Korg Triton",         None),
                Box::new(dawless_korg::triton::TritonTUI::new())),
            (Button::new("AKAI S3000XL",        None),
                Box::new(dawless_akai::S3000XLTUI::new())),
            (Button::new("AKAI MPC2000",        None),
                Box::new(dawless_akai::MPC2000TUI::new())),
            (Button::new("iConnectivity mioXL", None),
                Box::new(dawless_iconnectivity::MioXLTUI::new())),
        ])
    })
);

impl App {
    /// Set the exit flag, terminating the main loop before the next render.
    fn exit (&mut self) { self.exited.store(true, Ordering::Relaxed); }
}

impl TUI for App {
    fn layout <'a> (&'a self) -> Thunk<'a> {
        Outset(1).around(self.devices.layout())
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        Ok(if let Event::Key(KeyEvent { code: KeyCode::Char('q'), .. }) = event {
            self.exit();
            true
        } else {
            self.devices.handle(event)?
        })
    }
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
                let xy   = Point((screen_size.0 - size.0) / 2, (screen_size.1 - size.1) / 2);
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
