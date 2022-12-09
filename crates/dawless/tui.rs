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
            app.rect = (app.rect.0, app.rect.1, cols, rows);
            app.render(&mut term)?;
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
    let rect = (0, 0, 0, 0);
    let theme = Theme::default();
    let focused = true;
    let devices = List::default();
    let mut app = AppTUI { rect, theme, focused, devices, exited };
    app.devices
        .add("Korg Electribe",      Box::new(Electribe2TUI::new()))
        .add("Korg Triton",         Box::new(EmptyTUI {}))
        .add("AKAI S3000XL",        Box::new(EmptyTUI {}))
        .add("AKAI MPC2000",        Box::new(EmptyTUI {}))
        .add("iConnectivity mioXL", Box::new(EmptyTUI {}));
    app.layout(cols / 2 - 40, rows / 2 - 15, 0, 0)?;
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
    rect:    Rect,
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

    fn layout (&mut self, x: u16, y: u16, _w: u16, _h: u16) -> Result<()> {
        self.rect = (x, y, 23, 9);
        self.devices.layout(x + 1, y + 2, 19, 0)?;
        self.devices.items[0].1.layout(x + 25, y, 50, 30)?;
        Ok(())
    }

    fn render (&self, term: &mut dyn Write) -> Result<()> {
        clear(term)?;
        let Self { rect, theme, focused, .. } = *self;
        let (col1, row1, ..) = rect;
        Frame { rect: (col1 + 1, row1, 23, 9), theme, focused, title: "Devices" }.render(term)?;
        self.devices.render(term)?;
        if let Some(device) = self.devices.get() { device.render(term)?; }
        term.flush()?;
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
