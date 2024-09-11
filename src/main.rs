
use x11::xlib::{self, XNextEvent};
use std::{env, mem, process::exit, ptr};

mod init;
mod error;
mod state;
mod event;
mod wm;
mod config;
mod style;
mod util;

pub fn loop_poll_events(state: &mut state::State){
    let mut ev : xlib::XEvent = unsafe { mem::zeroed() };
    while(unsafe { XNextEvent(state.dpy, &mut ev) } == 0) { event::handle(state, ev); }
}

pub fn main() {    
    env::set_var("DISPLAY", ":1");
    match Some(unsafe{&mut(*xlib::XOpenDisplay(ptr::null()))}) {
        None => {
            println!("Cannot initialize display!");
            exit(1);
        },
        Some(dpy) => {
            init::check_other_wms(dpy);
            let mut state = init::setup(dpy);
            config::make(&mut state);
            init::setup_keybindings(&mut state);
            init::setup_mousemotions(&mut state);
            loop_poll_events(&mut state);
        }
    }    
}
