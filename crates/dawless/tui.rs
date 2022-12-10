use std::io::{Result, Write};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}, mpsc::channel};
use dawless_common::*;
use dawless_korg::Electribe2TUI;
use crossterm::{
    QueueableCommand, ExecutableCommand,
    event::{poll, read, Event, KeyEvent, KeyCode},
    style::{ResetColor},
    cursor::{Show, Hide},
    terminal::{
        size, enable_raw_mode, disable_raw_mode,
        EnterAlternateScreen, LeaveAlternateScreen,
        Clear, ClearType,
    }
};

pub(crate) fn main () -> Result<()> {
    let (tx, rx) = channel::<Event>();
    let exit = Arc::new(AtomicBool::new(false));
    let exit_thread = Arc::clone(&exit);
    std::thread::spawn(move || {
        let mut term = std::io::stdout();
        let mut app  = setup(&mut term, exit_thread)?;
        loop {
            if app.exited.fetch_and(true, Ordering::Relaxed) == true { break }
            let (cols, rows) = size()?;
            app.layout(&Space::new(0, 0, cols, rows))?;
            app.render(&mut term)?;
            term.flush()?;
            app.handle(&rx.recv().unwrap())?;
        }
        teardown(&mut term)
    });
    loop {
        if exit.fetch_and(true, Ordering::Relaxed) == true { break }
        if poll(std::time::Duration::from_millis(100))? {
            if tx.send(read()?).is_err() { break }
        }
    }
    Ok(())
}

fn setup (term: &mut dyn Write, exited: Arc<AtomicBool>) -> Result<AppTUI> {
    term.execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let (cols, rows) = size()?;
    let space = Space::new(0, 0, 0, 0);
    let theme = Theme::default();
    let focused = true;
    let devices = List::default();
    let mut app = AppTUI { space, theme, focused, devices, exited };
    app.devices
        .add("Korg Electribe",      Box::new(Electribe2TUI::new()))
        .add("Korg Triton",         Box::new(EmptyTUI {}))
        .add("AKAI S3000XL",        Box::new(EmptyTUI {}))
        .add("AKAI MPC2000",        Box::new(EmptyTUI {}))
        .add("iConnectivity mioXL", Box::new(EmptyTUI {}));
    app.layout(&Space::new(cols / 2 - 40, rows / 2 - 15, 0, 0))?;
    Ok(app)
}

fn teardown (term: &mut dyn Write) -> Result<()> {
    term.execute(ResetColor)?
        .execute(Show)?
        .execute(LeaveAlternateScreen)?;
    disable_raw_mode()
}

fn clear (term: &mut dyn Write) -> Result<()> {
    term.queue(ResetColor)?
        .queue(Clear(ClearType::All))?
        .queue(Hide)?;
    Ok(())
}

struct AppTUI {
    space:    Space,
    theme:   Theme,
    exited:  Arc<AtomicBool>,
    devices: List<Box<dyn TUI>>,
    focused: bool
}

impl AppTUI {
    fn exit (&mut self) {
        self.exited.store(true, Ordering::Relaxed);
    }
    fn device <'a> (&'a mut self) -> &'a mut dyn TUI {
        &mut **self.devices.get_mut().unwrap()
    }
}

impl TUI for AppTUI {

    fn layout (&mut self, space: &Space) -> Result<Space> {
        // Take the entire screen
        self.space = space.clone();
        // Apportion space for the device menu
        self.devices.layout(&space.sub(1, 2, 19, 0))?;
        // Apportion space for the menu items
        self.devices.items[0].1.layout(&space.sub(25, 0, 50, 30))?;
        Ok(self.space)
    }

    fn render (&self, term: &mut dyn Write) -> Result<()> {
        clear(term)?;
        let Self { space: Space { x, y, .. }, theme, focused, .. } = *self;

        let space = Space::new(x + 1, y, 23, 8);
        Frame { space, theme, focused, title: "Devices" }.render(term)?;

        self.devices.render(term)?;
        if let Some(device) = self.devices.get() { device.render(term)?; }

        Ok(())
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
            if self.device().handle(&event)? {
                return Ok(true)
            }
        }
        if self.devices.handle(event)? {
            return Ok(true)
        }
        handle_menu_focus!(event, self, self.device(), self.focused)
    }

}
