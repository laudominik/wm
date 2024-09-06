
use x11::xlib::{self, XNextEvent};
use std::{mem, process::exit, ptr};

mod init;
mod error;
mod state;
mod event;
mod wm;

pub fn loop_poll_events(state: &mut state::State){
    let mut ev : xlib::XEvent = unsafe { mem::zeroed() };
    while(unsafe { XNextEvent(state.dpy, &mut ev) } == 0) { event::handle(state, ev); }
}

pub fn main() {    

    match Some(unsafe{&mut(*xlib::XOpenDisplay(ptr::null()))}) {
        None => {
            println!("Cannot initialize display!");
            exit(1);
        },
        Some(dpy) => {
            init::check_other_wms(dpy);
            let mut state = init::setup(dpy);

            loop_poll_events(&mut state);
        }
    }    
}
