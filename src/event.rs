use std::{mem, os::linux::raw::stat};
use std::error::Error;

use x11::xlib::{self, CWBorderWidth, EnterWindowMask, False, Window, XConfigureWindow, XDisplayHeight, XDisplayWidth, XFlush, XGetWindowAttributes, XKeycodeToKeysym, XMapRequestEvent, XMapWindow, XMoveResizeWindow, XSelectInput, XSetWindowBorder, XSync, XWindowAttributes, XWindowChanges};

use crate::active_workspace;
use crate::{active_workspace_wins, config::STYLE, state::{State, KEYBINDINGS}, wm::_Tile};


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
        xlib::KeyPress => callback!(state, key, ev),
        xlib::UnmapNotify => callback!(state, unmap, ev),
        xlib::EnterNotify => callback!(state, crossing, ev),
        _ => println!("Unhandled event")
    }
}

fn map_request(state: &mut State, ev: xlib::XMapRequestEvent){
    println!("Map request");
    let mut wa : XWindowAttributes = unsafe { mem::zeroed() };
    if( unsafe { XGetWindowAttributes(state.dpy, ev.window, &mut wa) } == 0) { return };

    unsafe { XSelectInput(state.dpy, ev.window, EnterWindowMask) };

    state.focus(ev.window);
    active_workspace_wins!(state).push(ev.window);
    state.retile();
    unsafe {XSync(state.dpy, False)};
}   

fn unmap(state: &mut State, ev: xlib::XUnmapEvent){

    active_workspace_wins!(state).retain(|x| *x != ev.window);
    
    if ev.window == state.active.window {
        state.focus_next();
    }

    state.retile();
    println!("Window destroyed!");
}

fn key(state: &mut State, ev: xlib::XKeyEvent) {
    let keysym = unsafe { XKeycodeToKeysym(state.dpy, ev.keycode as u8, 0) } as u32;
    if let Some(binding) = unsafe { KEYBINDINGS.iter() }.find(
        |x| x.key == keysym && x.mdky ==  ev.state
    ) {
        (binding.callback)(state);
    }
}

fn crossing(state: &mut State, ev: xlib::XCrossingEvent){
    if ev.window == state.root { return };

    state.active.window = ev.window;
    state.retile();
}
