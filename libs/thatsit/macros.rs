#[macro_export] macro_rules! is_key {
    ($event:expr => $code:pat => $block:block) => {
        if let Event::Key(KeyEvent { code: $code, .. }) = $event {
            $block
        } else {
            false
        }
    }
}

#[macro_export] macro_rules! match_key {
    (($event:expr) { $($code:pat => $block:block),+ }) => {
        {
            match $event {
                $(Event::Key(KeyEvent { code: $code, .. }) => $block),*,
                _ => false
            }
        }
    }
}

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
