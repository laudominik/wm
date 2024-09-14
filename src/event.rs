use std::mem;

use x11::xlib::{self, EnterWindowMask, False, PointerMotionMask, StructureNotifyMask, XGetWindowAttributes, XKeycodeToKeysym, XSelectInput, XSync, XWindowAttributes};

//use crate::init::widget_window;
use crate::state::MOUSEMOTIONS;
use crate::{active_workspace_wins, state::{State, KEYBINDINGS}, };


macro_rules! callback {
    ($state: expr, $fn: ident, $ev:expr) => {
        $fn($state, unsafe { $ev.$fn } )
    };

    ($state: expr, $fn: expr, $ev_name: ident, $ev:expr) => {
        $fn($state, unsafe {$ev.$ev_name})
    }
}


pub fn handle(state: &mut State, ev: xlib::XEvent){
    let ty = ev.get_type();
    println!("xroagwem: event received: type={}", ty);
    match ty {
        xlib::MapRequest => callback!(state, map_request, ev),
        xlib::KeyPress => callback!(state, key, ev),
        xlib::DestroyNotify => callback!(state, destroy_window, ev),
        xlib::EnterNotify => callback!(state, crossing, ev),
        xlib::ButtonPress => callback!(state, button_pressed, button, ev),
        xlib::ButtonRelease => callback!(state, button_released, button, ev),
        xlib::MotionNotify => callback!(state, motion, ev),
        xlib::UnmapNotify => callback!(state, unmap, ev),
        xlib::ConfigureNotify => callback!(state, configure_request, ev),
        xlib::Expose => callback!(state, expose, ev),
        _ => println!("xroagwem: unhandled event")
    }
}

fn map_request(state: &mut State, ev: xlib::XMapRequestEvent) {
    let mut wa : XWindowAttributes = unsafe { mem::zeroed() };
    if( unsafe { XGetWindowAttributes(state.dpy, ev.window, &mut wa) } == 0) { return };

    unsafe { XSelectInput(state.dpy, ev.window, EnterWindowMask | PointerMotionMask | StructureNotifyMask ) };

    state.focus(ev.window);
    active_workspace_wins!(state).push(ev.window);
    state.retile();
    unsafe {XSync(state.dpy, False)};
}   

fn expose(_: &mut State, __: xlib::XExposeEvent) { }

fn configure_request(_: &mut State, __: xlib::XConfigureRequestEvent) { }

fn destroy_window(_: &mut State, __: xlib::XDestroyWindowEvent) {
    /* WARNING: there's some issue with XDestroyWindowEvent.window                    */
    /* it seems to be getting incorrect window value (not the actually destroyed one) */
    /* so for now using unmap (but need to be sure unmap is called AFTER              */
    /* switching active_workspace for changing workspaces                             */
    /* otherwise all windows will commit suicide)                                     */

}

fn unmap(state: &mut State, ev: xlib::XUnmapEvent) { 
    active_workspace_wins!(state).retain(|x| *x != ev.window);
    if ev.window == state.active.window {
        state.focus_next();
    }
    state.retile(); 
}

fn key(state: &mut State, ev: xlib::XKeyEvent) {
    let keysym = unsafe { XKeycodeToKeysym(state.dpy, ev.keycode as u8, 0) } as u32;
    if let Some(binding) = unsafe { KEYBINDINGS.iter() }.find(
        |x| x.key == keysym && x.mdky ==  ev.state
    ) {
        (binding.callback)(state);
    }
}

macro_rules! mm_invoke_callback {

    ($state: expr, $ty: ident, $ev:expr) => {
        mm_invoke_callback!($state, $ty, $ev, 0)
    };

    ($state: expr, $ty: ident, $ev:expr, nobutton) => {
        mm_invoke_callback!($state, $ty, $ev, 0, nobutton)
    };

    ($state: expr, $ty: ident, $ev: expr, $win: expr) => {
        for mm in unsafe{&MOUSEMOTIONS.$ty} {
            if mm.button != $ev.button || (mm.mdky & $ev.state) == 0 {
                continue;
            }
            (mm.callback)($state, ($ev.x_root, $ev.y_root), $win);
        }
    };

    ($state: expr, $ty: ident, $ev: expr, $win:expr, nobutton) => {
        for mm in unsafe{&MOUSEMOTIONS.$ty} {
            if (mm.mdky & $ev.state) == 0 {
                continue;
            }
            (mm.callback)($state, ($ev.x_root, $ev.y_root), $win);
        }
    };
}

fn crossing(state: &mut State, ev: xlib::XCrossingEvent) {
    mm_invoke_callback!(state, on_cross, ev, ev.window, nobutton);
    if ev.window == state.root { return };
    if state.active.focus_locked { return };
    state.focus(ev.window);
    state.retile();
}

fn button_pressed(state: &mut State, ev: xlib::XButtonEvent){ mm_invoke_callback!(state, on_press, ev); }
fn button_released(state: &mut State, ev: xlib::XButtonEvent){ mm_invoke_callback!(state, on_release, ev); }
fn motion(state: &mut State, ev: xlib::XMotionEvent){ mm_invoke_callback!(state, on_move, ev, nobutton); }