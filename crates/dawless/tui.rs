use std::io::{Result, Write};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}, mpsc::channel};
use dawless_common::*;
use dawless_korg::Electribe2TUI;
use crossterm::{
    event::{poll, read, Event, KeyEvent, KeyCode},
    terminal::{size}
};

pub(crate) fn main () -> Result<()> {
    let (tx, rx) = channel::<Event>();
    let exit = Arc::new(AtomicBool::new(false));
    let exit_thread = Arc::clone(&exit);
    std::thread::spawn(move || {
        let mut term = std::io::stdout();
        setup(&mut term)?;
        let mut app = AppTUI::new(exit_thread);
        loop {
            if app.exited.fetch_and(true, Ordering::Relaxed) == true { break }
            let (cols, rows) = size()?;
            let (cols, rows) = app.size().clip(cols, rows)?;
            app.layout(&Space::new(0, 0, cols, rows))?;
            app.offset(cols, rows);
            clear(&mut term)?;
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

struct AppTUI {
    exited:  Arc<AtomicBool>,
    space:   Space,
    theme:   Theme,
    focused: bool,
    frame:   Frame,
    menu:    List<Box<dyn TUI>>,
}

impl AppTUI {
    fn new (exited: Arc<AtomicBool>) -> Self {
        let theme = Theme::default();
        let mut menu = List::<Box<dyn TUI>> { theme, ..List::default() };
        menu.add("Korg Electribe",      Box::new(Electribe2TUI::new()))
            .add("Korg Triton",         Box::new(EmptyTUI {}))
            .add("AKAI S3000XL",        Box::new(EmptyTUI {}))
            .add("AKAI MPC2000",        Box::new(EmptyTUI {}))
            .add("iConnectivity mioXL", Box::new(EmptyTUI {}));
        let mut frame = Frame { theme, ..Frame::default() };
        frame.title = "Devices".into();
        Self {
            exited,
            space:   Space::default(),
            theme,
            focused: true,
            frame,
            menu,
        }
    }
    fn exit (&mut self) {
        self.exited.store(true, Ordering::Relaxed);
    }
    fn device <'a> (&'a self) -> &'a dyn TUI {
        &**self.menu.get().unwrap()
    }
    fn device_mut <'a> (&'a mut self) -> &'a mut dyn TUI {
        &mut **self.menu.get_mut().unwrap()
    }
}

impl TUI for AppTUI {

    fn size (&self) -> Size {
        self.menu.size() + self.device().size()
    }

    fn layout (&mut self, space: &Space) -> Result<Space> {
        let menu = self.menu.layout(&space.add(0, 2, 1, 2))?;
        let item = self.device_mut().layout(&menu.right(1))?;
        self.space = menu.join(&item);
        self.frame.space = menu.clone().add(0, -2, 1, 3);
        Ok(self.space)
    }

    fn offset (&mut self, dx: u16, dy: u16) {
        self.space = self.space.offset(dx, dy);
        self.frame.offset(dx, dy);
        self.menu.offset(dx, dy);
        self.device_mut().offset(dx, dy);
    }

    fn render (&self, term: &mut dyn Write) -> Result<()> {
        self.frame.render(term)?;
        self.menu.render(term)?;
        if let Some(device) = self.menu.get() { device.render(term)?; }
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
            if self.device_mut().handle(&event)? {
                return Ok(true)
            }
        }
        if self.menu.handle(event)? {
            return Ok(true)
        }
        handle_menu_focus!(event, self, self.device_mut(), self.focused)
    }

}
