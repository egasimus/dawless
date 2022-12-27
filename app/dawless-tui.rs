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

impl App {
    /// Set the exit flag, terminating the main loop before the next render.
    fn exit (&mut self) { self.exited.store(true, Ordering::Relaxed); }
}

impl TUI for App {
    fn layout <'a> (&'a self, max: Size) -> Result<Thunk<'a>> {
        Ok(Centered.around(Outset(1).around(self.devices.layout(max)?)))
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
    run(&EXITED, &mut std::io::stdout(), App {
        exited: &EXITED,
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
}
