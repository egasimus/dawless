use std::io::{Result, Write};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}, mpsc::channel};
use dawless_common::{TUI, render_frame, Menu, handle_menu_focus};
use dawless_korg::Electribe2TUI;
use crossterm::{
    execute, QueueableCommand,
    event::{poll, read, Event, KeyEvent, KeyCode},
    style::{Color, ResetColor, SetForegroundColor, SetBackgroundColor},
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
        execute!(term, EnterAlternateScreen)?;
        enable_raw_mode()?;
        let mut tui = RootTUI::new(exit_thread.clone());
        loop {
            if exit_thread.fetch_and(true, Ordering::Relaxed) == true {
                break
            }
            let (cols, rows) = size()?;
            tui.render(&mut term, 0, 0, cols, rows)?;
            tui.handle(&rx.recv().unwrap())?;
        }
        execute!(term, ResetColor, Show, LeaveAlternateScreen)?;
        disable_raw_mode()
    });

    loop {
        if exit.fetch_and(true, Ordering::Relaxed) == true {
            break
        }
        if poll(std::time::Duration::from_millis(100))? {
            if let Err(_) = tx.send(read()?) {
                break
            }
        }
    }

    Ok(())

}

struct EmptyTUI {}

impl TUI for EmptyTUI {
    fn render (&self, term: &mut dyn Write, col1: u16, row1: u16, _cols: u16, _rows: u16) -> Result<()> {
        Ok(())
    }
}

struct RootTUI {
    exited:  Arc<AtomicBool>,
    devices: Menu<Box<dyn TUI>>,
    focused: bool
}

impl RootTUI {
    fn new (exited: Arc<AtomicBool>) -> Self {
        RootTUI {
            exited,
            focused: true,
            devices: Menu::new(vec![
                ("Korg Electribe".into(),      Box::new(Electribe2TUI::new()) as Box<dyn TUI>),
                ("Korg Triton".into(),         Box::new(EmptyTUI {})),
                ("AKAI S3000XL".into(),        Box::new(EmptyTUI {})),
                ("AKAI MPC2000".into(),        Box::new(EmptyTUI {})),
                ("iConnectivity mioXL".into(), Box::new(EmptyTUI {}))
            ])
        }
    }
    fn exit (&mut self) {
        self.exited.store(true, Ordering::Relaxed);
    }
    fn device <'a> (&'a mut self) -> &'a mut dyn TUI {
        &mut **self.devices.get_mut().unwrap()
    }
}

impl TUI for RootTUI {

    fn render (
        &self, term: &mut dyn Write, col1: u16, row1: u16, _cols: u16, _rows: u16
    ) -> Result<()> {
        term.queue(ResetColor)?
            .queue(Clear(ClearType::All))?
            .queue(Hide)?;
        let bg = Color::AnsiValue(232);
        let fg = Color::White;
        let hi = Color::Yellow;
        render_frame(term, col1 + 1, row1, 23, 9, bg, Some((
            if self.focused { hi } else { bg },
            if self.focused { bg } else { hi },
            "Devices"
        )))?;
        self.devices.render(term, col1 + 1, row1 + 2, 19, 0)?;
        if let Some(device) = self.devices.get() {
            device.render(term, col1 + 25, row1, 50, 30)?;
        }
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
