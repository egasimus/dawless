use crate::*;

/// Run the main loop. The main thread goes into a render loop. A separate input thread is
/// launched, which sends input to the main thread.
///
/// # Arguments
///
/// * `exited` - Atomic exit flag. Setting this to `true` tells both threads to end.
/// * `term` - A writable output, such as `std::io::stdout()`.
/// * `app` - An instance of the root widget that contains your application.
pub fn run <T: Widget> (exited: &'static AtomicBool, term: &mut dyn Write, app: T) -> Result<()> {
    let app: std::cell::RefCell<T> = RefCell::new(app);
    // Set up event channel and input thread
    let (tx, rx) = channel::<Event>();
    spawn_input_thread(tx, exited);
    // Setup terminal and panic hook
    setup(term, true)?;
    // Render app and listen for updates
    loop {
        // Clear screen
        clear(term).unwrap();
        // Break loop if exited
        if exited.fetch_and(true, Ordering::Relaxed) == true {
            break
        }
        // Render
        let (w, h) = size()?;
        if let Err(error) = app.borrow().render(term, Area(0, 0, w, h)) {
            write_error(term, format!("{error}").as_str()).unwrap();
        }
        // Flush output buffer
        term.flush().unwrap();
        // Wait for input and update
        app.borrow_mut().handle(&rx.recv().unwrap()).unwrap();
    }
    // Clean up
    teardown(term)?;
    Ok(())
}

pub fn setup (term: &mut dyn Write, better_panic: bool) -> Result<()> {
    if better_panic {
        std::panic::set_hook(Box::new(|panic_info| {
            teardown(&mut std::io::stdout()).unwrap();
            ::better_panic::Settings::auto().create_panic_handler()(panic_info);
        }));
    }
    term.execute(EnterAlternateScreen)?.execute(Hide)?;
    enable_raw_mode()
}

pub fn teardown (term: &mut dyn Write) -> Result<()> {
    term.execute(ResetColor)?.execute(Show)?.execute(LeaveAlternateScreen)?;
    disable_raw_mode()
}

pub fn clear (term: &mut dyn Write) -> Result<()> {
    term.queue(ResetColor)?.queue(Clear(ClearType::All))? .queue(Hide)?;
    Ok(())
}

pub fn spawn_input_thread (tx: Sender<Event>, exited: &'static AtomicBool) {
    std::thread::spawn(move || {
        loop {
            if exited.fetch_and(true, Ordering::Relaxed) { break }
            if crossterm::event::poll(std::time::Duration::from_millis(100)).is_ok() {
                if tx.send(crossterm::event::read().unwrap()).is_err() { break }
            }
        }
    });
}

pub fn write_error (term: &mut dyn Write, msg: &str) -> Result<()> {
    clear(term)?;
    term.queue(SetForegroundColor(Color::Red))?;
    write_text(term, 0, 0, msg)
}

pub fn write_text (term: &mut dyn Write, x: Unit, y: Unit, text: &str) -> Result<()> {
    term.execute(MoveTo(x, y))?.execute(Print(text))?;
    Ok(())
}

