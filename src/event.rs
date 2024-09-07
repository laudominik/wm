use std::mem;

use x11::xlib::{self, CWBorderWidth, False, Window, XConfigureWindow, XFlush, XGetWindowAttributes, XMapRequestEvent, XMapWindow, XMoveResizeWindow, XSetWindowBorder, XSync, XWindowAttributes, XWindowChanges};

use crate::{state::State, config::STYLE, wm::Tile};


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
    let mut wc: XWindowChanges = unsafe { mem::zeroed() };
    if( unsafe { XGetWindowAttributes(state.dpy, ev.window, &mut wa) } == 0) { return };
    let tile = Tile::new(state, (wa.width, wa.height));
    
    
    unsafe { 
        wc.border_width = STYLE.border_thickness;
        XConfigureWindow(state.dpy, ev.window, CWBorderWidth.into(), &mut wc as *mut XWindowChanges);

        XSetWindowBorder(state.dpy, ev.window, 0x11111111);
        XMoveResizeWindow(state.dpy, ev.window, 
            tile.coords.0, tile.coords.1, 
            tile.size.0, tile.size.1);
        XMapWindow(state.dpy, ev.window);
        XSync(state.dpy, False);
    }
}   