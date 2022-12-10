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
        let mut app = App::new(exited);
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
        let menu = List::default();
        menu.add("Korg Electribe",      Box::new(Electribe2TUI::new()))
            .add("Korg Triton",         Box::new(EmptyTUI {}))
            .add("AKAI S3000XL",        Box::new(EmptyTUI {}))
            .add("AKAI MPC2000",        Box::new(EmptyTUI {}))
            .add("iConnectivity mioXL", Box::new(EmptyTUI {}));
        Self {
            exited,
            space:   Space::default(),
            theme:   Theme::default(),
            focused: true,
            frame:   Frame::default(),
            menu,
        }
    }
    fn exit (&mut self) {
        self.exited.store(true, Ordering::Relaxed);
    }
    fn device <'a> (&'a mut self) -> &'a mut dyn TUI {
        &mut **self.menu.get_mut().unwrap()
    }
}

impl TUI for AppTUI {

    fn layout (&mut self, space: &Space) -> Result<Space> {
        // Start by putting the upper left corner
        // of the layout at the center of the screen
        let (mut x, mut y) = space.center();
        // Offset it by half the size of each displayed widgets
        let menu = self.menu.layout(&space)?;
        let item = self.device().layout(&space)?;
        x = x.saturating_sub(u16::max(menu.w, item.w) / 2);
        y = y.saturating_sub(u16::max(menu.h, item.h) / 2);
        // The layout space is now equal to the centered widgets
        self.space = Space::new(x, y, space.w - x * 2, space.h - y * 2);
        // Apportion space for the device menu
        self.frame.layout(&self.space.sub(0, 0, 23, 8));
        self.menu.layout(&self.space.sub(0, 2, 19, 0))?;
        // Apportion space for the menu items
        let item = self.space.sub(24, 0, 50, 30);
        self.device().layout(&item)?;
        Ok(self.space)
    }

    fn render (&self, term: &mut dyn Write) -> Result<()> {
        clear(term)?;
        let Self { space: Space { x, y, .. }, theme, focused, .. } = *self;
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
            if self.device().handle(&event)? {
                return Ok(true)
            }
        }
        if self.menu.handle(event)? {
            return Ok(true)
        }
        handle_menu_focus!(event, self, self.device(), self.focused)
    }

}
