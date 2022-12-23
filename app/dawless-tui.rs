use std::cell::RefCell;
use std::io::{Result, Write};
use std::sync::{atomic::{AtomicBool, Ordering}, mpsc::channel};
use thatsit::{*, crossterm::{
    event::{Event, KeyEvent, KeyCode},
    terminal::{size},
    style::Color
}};

static THEME: Theme = Theme {
    bg: Color::AnsiValue(232),
    fg: Color::White,
    hi: Color::Yellow
};

static EXITED: AtomicBool = AtomicBool::new(false);

thread_local!(static APP: RefCell<App> = RefCell::new(App {
    exited:  &EXITED,
    focused: true,
    frame:   Frame {
        theme: THEME,
        title: "Select device:".into(),
        focused: true,
        ..Frame::default()
    },
    menu:    FocusColumn {
        theme: THEME,
        ..FocusColumn::default()
    },
    open:    false,
}));

struct App {
    exited:  &'static AtomicBool,
    focused: bool,
    frame:   Frame,
    menu:    FocusColumn<Button>,
    open:    bool,
}

pub(crate) fn main () -> Result<()> {

    // Init app features
    APP.with(|app| {
        app.borrow_mut().menu.items = vec![
            Button::new("Korg Electribe"),
            Button::new("Korg Triton"),
            Button::new("AKAI S3000XL"),
            Button::new("AKAI MPC2000"),
            Button::new("iConnectivity mioXL")
        ];
    });

    // Set up event channel and input thread
    let (tx, rx) = channel::<Event>();
    spawn_input_thread(tx, &EXITED);

    // Setup terminal and panic hook
    let mut term = std::io::stdout();
    setup(&mut term, true)?;

    // Render app and listen for updates
    loop {
        let mut done = false;
        APP.with(|app| {

            // Clear screen
            clear(&mut term).unwrap();

            // Break loop if exited
            if app.borrow().exited.fetch_and(true, Ordering::Relaxed) == true {
                done = true;
                return
            }

            // Check if there is sufficient screen size
            let screen_size: Size = size().unwrap().into();
            let min_size = app.borrow().layout().min_size;
            if let Err(e) = min_size.fits_in(screen_size) {
                write_error(&mut term, format!("{e}").as_str()).unwrap();
            } else {
                // Render to output buffer
                let size = screen_size.crop_to(min_size);
                let xy = Point((screen_size.0 - size.0) / 2, (screen_size.1 - size.1) / 2);
                if let Err(e) = app.borrow().render(&mut term, Area(xy, size)) {
                    write_error(&mut term, format!("{e}").as_str()).unwrap();
                };
            }

            // Flush output buffer
            term.flush().unwrap();

            // Wait for input and update
            app.borrow_mut().handle(&rx.recv().unwrap()).unwrap();

        });
        if done {
            break
        }
    }

    // Clean up
    teardown(&mut term)?;

    Ok(())
}

impl App {
    fn exit (&mut self) {
        self.exited.store(true, Ordering::Relaxed);
    }
}

impl TUI for App {
    fn layout <'a> (&'a self) -> Thunk<'a> {
        //row(|add|{
            //add(stack(|add|{
                //add(&self.frame);
                //add(&self.menu);
                col(|add|{
                    add(&self.menu);
                })
            //}));
            //if self.open { add(self.menu.get().unwrap()); }
        //})
    }
    fn focus (&mut self, focus: bool) -> bool {
        self.focused = focus;
        self.frame.focused = focus;
        true
    }
    fn handle (&mut self, event: &Event) -> Result<bool> {
        if let Event::Key(KeyEvent { code: KeyCode::Char('q'), .. }) = event {
            self.exit();
            return Ok(true)
        }
        if !self.focused {
            if self.menu.get_mut().handle(&event)? {
                return Ok(true)
            }
        }
        if self.menu.handle(event)? {
            return Ok(true)
        }
        let result = handle_menu_focus!(
            event, self, self.menu.get_mut(), self.focused
        );
        if let Ok(true) = result {
            self.open = !self.focused
        }
        result
    }
}
