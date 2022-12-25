/// Generate an `Event::Key(KeyEvent { ... })` variant
#[macro_export] macro_rules! key {
    ($code:ident) => {
        Event::Key(KeyEvent {
            code:      KeyCode::$code,
            modifiers: KeyModifiers::NONE,
            kind:      KeyEventKind::Press,
            state:     KeyEventState::NONE
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

/// Compare render output against an expected value.
#[macro_export] macro_rules! assert_rendered {
    ($layout:ident == $expected:expr) => {
        let mut output = Vec::<u8>::new();
        assert_eq!($layout.render(&mut output, Area(Point(5, 5), Size(10, 10))).unwrap(), ());
        assert_eq!(from_utf8(&output).unwrap(), $expected);
    }
}

/// Implement a focus flag
#[macro_export] macro_rules! impl_focus {
    ($field:ident) => {
        fn focused (&self) -> bool { self.$field }
        fn focus (&mut self, focus: bool) -> bool { self.$field = focus; true }
    }
}

// TODO:
/*
macro_rules! layout {
    ($self:ident, $($layout:tt)+) => {
        fn layout (&$self) -> Layout {
            layout!(@ $($layout)+)
        }
    };
    (@ Item($($layout:tt)+)) => {
        Layout::Item(layout!(@ $($layout)+))
    };
    (@ Min($($layout:tt)+)) => {
        Layout::Min(layout!(@ $($layout)+))
    };
    (@ Max($expr:expr)) => {
        Layout::Max($expr)
    };
    (@ Layers($($op:ident ($($layout:tt)+)),+)) => {
        Layout::Layers(vec![$($op($($layout)+)),+])
    };
    (@ Row($($expr:expr)+)) => {
        Layout::Row(vec![$($expr),+])
    };
    (@ $expr:expr) => {
        $expr
    };
}
*/
