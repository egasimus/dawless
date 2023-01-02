/// Glue macro to quickly implement a TUI trait.
/// This constains the actual type definitions of the trait methods
/// because when they change, they tend to change quite a bit,
/// all at once, in numerous places.
#[macro_export] macro_rules! tui {
    (
        <$l1:lifetime$(,$($id:ident:$ty:path),+)?> $type:ty {
            $(<$l2:lifetime> layout ($self1:ident, $max:ident) $body1:block)?
            $(handle ($self3:ident, $event:ident) $body3:block)?
        }
    ) => {
        impl<$l1$(,$($id:$ty),+)?> TUI<$l1> for $type {
            $(
                /// Describe this widget out of renderable elements
                fn layout <$l2: $l1> (&$l2 $self1, $max: Size)
                    -> Result<Layout<$l2>>
                $body1
            )?
            $(
                /// Handle an input event. Return whether the event was captured.
                fn handle (&mut $self3, $event: &Event)
                    -> Result<bool>
                $body3
            )?
        }
    };
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

/// Compare render output against an expected value.
#[macro_export] macro_rules! assert_rendered {
    ($layout:ident == $expected:expr) => {
        let mut output = Vec::<u8>::new();
        assert_eq!($layout.render(&mut output, Area(Point(5, 5), Size(10, 10))).unwrap(), ());
        assert_eq!(from_utf8(&output).unwrap(), $expected);
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
