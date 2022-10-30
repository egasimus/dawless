#![feature(adt_const_params)]

macro_rules! module {
    ($name:ident) => {
        mod $name;
        pub use $name::*;
    };
}

module!(akai_string);
module!(akai_device);
module!(akai_disk);
module!(akai_file);

pub(crate) use brailledump::BrailleDump;
