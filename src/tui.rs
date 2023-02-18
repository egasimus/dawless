use std::io::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use thatsit::{*, engines::tui::crossterm::{style::Color, event::{Event, KeyEvent, KeyCode}}};

/// Exit flag. Setting this to true terminates the main loop.
static EXITED: AtomicBool = AtomicBool::new(false);

pub(crate) fn main () -> Result<()> {
    App::new()
        .page(" Korg Electribe 2 ",    Box::new(crate::korg::electribe2::Electribe2UI::new()))
        .page(" Korg Triton ",         Box::new(crate::korg::triton::TritonUI::new()))
        .page(" AKAI S3000XL ",        Box::new(crate::akai::S3000XLUI::new()))
        .page(" AKAI MPC2000 ",        Box::new(crate::akai::MPC2000UI::new()))
        .page(" iConnectivity mioXL ", Box::new(crate::iconnectivity::MioXLUI::new()))
        .run(&EXITED, &mut std::io::stdout(), )
}

/// The main app object, containing a menu of supported devices.
#[derive(Debug)]
struct App {
    /// A reference to the exit flag to end the main loop.
    exited:  &'static AtomicBool,
    /// A tabbed collection of supported devices.
    devices: Tabs<Box<dyn Widget>>,
}

impl App {
    fn new () -> Self { Self { exited: &EXITED, devices: Tabs::new(TabSide::Left, vec![]) } }
    /// Set the exit flag, terminating the main loop before the next render.
    fn exit (&mut self) { self.exited.store(true, Ordering::Relaxed); }
    /// Add a device page to the app
    fn page (mut self, label: &str, device: Box<dyn Widget>) -> Self {
        self.devices.add(label.into(), device);
        self.devices.pages.select(0);
        self
    }
}

impl Widget for App {
    impl_render!(self, out, area => {
        Aligned(Align::Center, Border(Tall, Outset, Stacked::y(|add|{
            add(&self.devices);
        }))).render(out, area)
    });
    impl_handle!(self, event => {
        Ok(if let Event::Key(KeyEvent { code: KeyCode::Char('q'), .. }) = event {
            self.exit();
            true
        } else {
            self.devices.handle(event)?
        })
    });
}
