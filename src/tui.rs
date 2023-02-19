use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use thatsit::{
    *,
    layouts::*,
    engines::tui::{*, crossterm::{style::Color, event::{Event, KeyEvent, KeyCode}}},
    widgets::tui::*
};

/// Exit flag. Setting this to true terminates the main loop.
static EXITED: AtomicBool = AtomicBool::new(false);

pub(crate) fn main () -> std::io::Result<()> {
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
struct App<A, B, C> {
    /// A reference to the exit flag to end the main loop.
    exited:  &'static AtomicBool,
    /// A tabbed collection of supported devices.
    devices: Tabbed<Box<dyn Widget<A, B, C>>>,
}

impl<A, B, C> App<A, B, C> {
    fn new () -> Self { Self { exited: &EXITED, devices: Tabbed::new(TabSide::Left, vec![]) } }
    /// Set the exit flag, terminating the main loop before the next render.
    fn exit (&mut self) { self.exited.store(true, Ordering::Relaxed); }
    /// Add a device page to the app
    fn page (mut self, label: &str, device: Box<dyn Widget<A, B, C>>) -> Self {
        self.devices.add(label.into(), device);
        self.devices.pages.select(0);
        self
    }
}

impl<E, I, O> Output<E, O> for App<E, I, O> {
    fn render (&self, engine: &mut E) -> Result<Option<O>> {
        Aligned(Align::Center, Rows::new().border(Tall, Outset).add(&self.devices)).render(engine)
    }
}

impl<E, I, O> Input<E, I> for App<E, I, O> {
    fn handle (&mut self, engine: &mut E) -> Result<Option<I>> {
        Ok(if let Event::Key(KeyEvent { code: KeyCode::Char('q'), .. }) = engine.event {
            self.exit();
            true
        } else {
            self.devices.handle(engine.event)?
        })
    }
}
