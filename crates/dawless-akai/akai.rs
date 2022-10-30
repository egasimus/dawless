#![feature(adt_const_params)]

macro_rules! module {
    ($name:ident) => {
        mod $name;
        pub use $name::*;
    };
}

module!(akai_string);
module!(akai_devices);
module!(akai_disk);
module!(akai_disk_blank);
module!(akai_disk_load);
module!(akai_disk_files);
module!(akai_sample);
