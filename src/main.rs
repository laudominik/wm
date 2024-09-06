
use x11::xlib::{self, XNextEvent};
use std::{mem, process::exit, ptr};

mod init;
mod error;
mod state;

pub fn loop_poll_events(dpy: &mut xlib::Display){
    let mut ev : xlib::XEvent = unsafe { mem::zeroed() };

    while(unsafe { XNextEvent(dpy, &mut ev) } != 0){
        println!("Event received: type={}", ev.get_type()); // Example
    }
}



pub fn main() {    

    match Some(unsafe{&mut(*xlib::XOpenDisplay(ptr::null()))}) {
        None => {
            println!("Cannot initialize display!");
            exit(1);
        },
        Some(dpy) => {
            init::check_other_wms(dpy);
            init::setup(dpy);

            loop_poll_events(dpy);
        }
    }

    //XEvent ev;
    
}
