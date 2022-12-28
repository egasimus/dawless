use std::io::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use thatsit::{*, crossterm::{event::{Event, KeyEvent, KeyCode}}};
use thatsit_tabs::*;

/// Exit flag. Setting this to true terminates the main loop.
static EXITED: AtomicBool = AtomicBool::new(false);

pub(crate) fn main () -> Result<()> {
    run(&EXITED, &mut std::io::stdout(), App::new()
        .page("Korg Electribe 2",    Box::new(dawless_korg::electribe2::Electribe2TUI::new()))
        .page("Korg Triton",         Box::new(dawless_korg::triton::TritonTUI::new()))
        .page("AKAI S3000XL",        Box::new(dawless_akai::S3000XLTUI::new()))
        .page("AKAI MPC2000",        Box::new(dawless_akai::MPC2000TUI::new()))
        .page("iConnectivity mioXL", Box::new(dawless_iconnectivity::MioXLTUI::new())))
}

/// The main app object, containing a menu of supported devices.
#[derive(Debug)]
struct App {
    /// A reference to the exit flag to end the main loop.
    exited:  &'static AtomicBool,
    /// A tabbed collection of supported devices.
    devices: TabbedVertical<Box<dyn TUI>>,
}

impl App {
    fn new () -> Self { Self { exited: &EXITED, devices: TabbedVertical::default() } }
    /// Set the exit flag, terminating the main loop before the next render.
    fn exit (&mut self) { self.exited.store(true, Ordering::Relaxed); }
    /// Add a device page to the app
    fn page (mut self, label: &str, device: Box<dyn TUI>) -> Self {
        self.devices.add(label.into(), device);
        self
    }
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
