use std::process::exit;
use x11::xlib::{self};

pub unsafe extern "C" fn xerror(_: *mut xlib::Display, ev: *mut xlib::XErrorEvent ) -> i32 {
    println!("ERROR: {} {} {}", (*ev).error_code, (*ev).request_code, (*ev).minor_code);
    return 1;
}

pub unsafe extern "C" fn xerror_start(_: *mut xlib::Display, __: *mut xlib::XErrorEvent ) -> i32 {
    println!("xroagwem: Another wm is already running!");
    exit(1);
}