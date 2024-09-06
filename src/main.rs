
use x11::xlib::{self};
use std::{process::exit, ptr};

mod init;
mod error;
mod state;

pub fn main() {    

    match Some(unsafe{&mut(*xlib::XOpenDisplay(ptr::null()))}) {
        None => {
            println!("Cannot initialize display!");
            exit(1);
        },
        Some(dpy) => {
            init::check_other_wms(dpy);
            init::setup(dpy);
        }
    }
}
