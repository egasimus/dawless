use crate::*;

pub trait Handle {
    fn handle (&mut self, _event: &Event) -> Result<bool> {
        Ok(false)
    }
}

/// Generate an `Event::Key(KeyEvent { ... })` variant
#[macro_export] macro_rules! key {
    ($code:ident) => {
        crossterm::event::Event::Key(crossterm::event::KeyEvent {
            code:      crossterm::event::KeyCode::$code,
            modifiers: crossterm::event::KeyModifiers::NONE,
            kind:      crossterm::event::KeyEventKind::Press,
            state:     crossterm::event::KeyEventState::NONE
        })
    };
    (Ctrl-$code:ident) => {
        crossterm::event::Event::Key(KeyEvent {
            code:      crossterm::event::KeyCode::$code,
            modifiers: crossterm::event::KeyModifiers::CONTROL,
            kind:      crossterm::event::KeyEventKind::Press,
            state:     crossterm::event::KeyEventState::NONE
        })
    }
}

/// Match an input event against a specified key event
#[macro_export] macro_rules! if_key {
    ($event:expr => $code:ident => $block:block) => {
        if $event == &key!($code) {
            $block
        } else {
            false
        }
    }
}

/// Match an input event against a list of key events
#[macro_export] macro_rules! match_key {
    (($event:expr) { $($code:expr => $block:block),+ }) => {
        {
            if let Event::Key(event) = $event {
                $(if event.code == $code $block else)* { false }
            } else {
                false
            }
        }
    }
}