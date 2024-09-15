
use widgets::widget_refresh;
use x11::xlib::{self, XNextEvent};
use std::{env, mem, process::exit, ptr, thread, time::Duration};

mod init;
mod error;
mod state;
mod event;
mod wm;
mod config;
mod style;
mod util;
mod widgets;

pub fn loop_poll_events(state: &mut state::State){
    let mut ev : xlib::XEvent = unsafe { mem::zeroed() };
    unsafe { xlib::XSync(state.dpy, xlib::False); }
    while(unsafe { XNextEvent(state.dpy, &mut ev) } == 0) { event::handle(state, ev); }
}

pub fn main() {    
    env::set_var("DISPLAY", ":1");
    match Some(unsafe{&mut(*xlib::XOpenDisplay(ptr::null()))}) {
        None => {
            println!("xroagwem: cannot initialize display!");
            exit(1);
        },
        Some(dpy) => {
            init::check_other_wms(dpy);
            let mut state = init::setup(dpy);
            config::make(&mut state);
            init::setup_keybindings(&mut state);
            init::setup_mousemotions(&mut state);
            thread::spawn(|| {
                loop {
                    widget_refresh();
                    thread::sleep(Duration::from_secs(1));
                }
            });
            loop_poll_events(&mut state);
        }
    }    
}
