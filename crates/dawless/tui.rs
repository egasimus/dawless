use std::io::{Result, Write};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}, mpsc::channel};
use dawless_common::*;
use dawless_korg::Electribe2TUI;
use crossterm::{
    execute, QueueableCommand,
    event::{poll, read, Event, KeyEvent, KeyCode},
    style::{Color, ResetColor},
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

        let (cols, rows) = size()?;

        let mut tui = RootTUI {
            rect:    (cols / 2 - 24, rows / 2 - 15, 0, 0),
            theme:   Theme::default(),
            focused: true,
            devices: List::default(),
            exited:  exit_thread.clone(),
        };

        let mut e2tui = Electribe2TUI::new();
        e2tui.rect = (tui.rect.0 + 25, tui.rect.1, 50, 30);
        e2tui.menu.rect = (e2tui.rect.0, e2tui.rect.1 + 2, 17, 0);
        e2tui.patterns.rect = (e2tui.rect.0 + 22, e2tui.rect.1, 0, 0);
        e2tui.samples.rect = (e2tui.rect.0 + 22, e2tui.rect.1, 0, 0);

        tui.devices
            .add("Korg Electribe",      Box::new(e2tui))
            .add("Korg Triton",         Box::new(EmptyTUI {}))
            .add("AKAI S3000XL",        Box::new(EmptyTUI {}))
            .add("AKAI MPC2000",        Box::new(EmptyTUI {}))
            .add("iConnectivity mioXL", Box::new(EmptyTUI {}));

        tui.devices.rect = (tui.rect.0 + 1, tui.rect.1 + 2, 19, 0);

        loop {
            if exit_thread.fetch_and(true, Ordering::Relaxed) == true {
                break
            }
            let (cols, rows) = size()?;
            tui.rect = (tui.rect.0, tui.rect.1, cols, rows);
            tui.render(&mut term)?;
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

struct RootTUI {
    rect:    Rect,
    theme:   Theme,
    exited:  Arc<AtomicBool>,
    devices: List<Box<dyn TUI>>,
    focused: bool
}

impl RootTUI {
    fn exit (&mut self) {
        self.exited.store(true, Ordering::Relaxed);
    }
    fn device <'a> (&'a mut self) -> &'a mut dyn TUI {
        &mut **self.devices.get_mut().unwrap()
    }
}

impl TUI for RootTUI {

    fn render (&self, term: &mut dyn Write) -> Result<()> {

        term.queue(ResetColor)?
            .queue(Clear(ClearType::All))?
            .queue(Hide)?;

        let (col1, row1, ..) = self.rect;

        Frame {
            rect:    (col1 + 1, row1, 23, 9),
            theme:   self.theme,
            title:   "Devices",
            focused: self.focused,
        }(term)?;

        self.devices.render(term)?;

        if let Some(device) = self.devices.get() {
            device.render(term)?;
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
