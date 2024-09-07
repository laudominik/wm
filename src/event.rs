use std::mem;

use x11::xlib::{self, CWBorderWidth, False, Window, XConfigureWindow, XDisplayHeight, XDisplayWidth, XFlush, XGetWindowAttributes, XMapRequestEvent, XMapWindow, XMoveResizeWindow, XSetWindowBorder, XSync, XWindowAttributes, XWindowChanges};

use crate::{state::State, config::STYLE, wm::_Tile};


macro_rules! callback {
    ($state: expr, $fn: ident, $ev:expr) => {
        $fn($state, unsafe { $ev.$fn } )
    };
}

pub fn handle(state: &mut State, ev: xlib::XEvent){
    let ty = ev.get_type();
    println!("Event received: type={}", ty);
    match ty {
        xlib::MapRequest => callback!(state, map_request, ev),
        _ => println!("Unhandled event")
    }
}

fn map_request(state: &mut State, ev: xlib::XMapRequestEvent){
    println!("Map request");
    let mut wa : XWindowAttributes = unsafe { mem::zeroed() };
    if( unsafe { XGetWindowAttributes(state.dpy, ev.window, &mut wa) } == 0) { return };

    state.workspaces[state.active_workspace].windows.push(ev.window);
    state.retile();
    unsafe {XSync(state.dpy, False)};
}   