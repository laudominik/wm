use x11::xft::{XftColor, XftColorAllocName};
use x11::xlib::{Button3, ButtonPressMask, ButtonReleaseMask, PointerMotionMask, CWCursor, CWEventMask, GrabModeAsync, PropModeReplace, True, XChangeWindowAttributes, XDefaultColormap, XDefaultGC, XDefaultVisual, XFlush, XGrabButton, XGrabKey, XMapWindow, XSetWindowAttributes, XWhitePixel, XA_WINDOW};
use x11::xlib::{self, False, XChangeProperty, XCreateSimpleWindow, XCreateWindow, XSync};
use x11::xrender::XRenderColor;
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr::null;
use std::mem;

use crate::config::{CustomData, STYLE};
use crate::wm;
use crate::style::{self};
use crate::state::{Active, Cursor, State, KEYBINDINGS, MOUSEMOTIONS};

use super::error;
use super::state;

pub fn check_other_wms(dpy: &mut xlib::Display){
    unsafe {
        xlib::XSetErrorHandler(Some(error::xerror_start));
        xlib::XSelectInput(dpy, xlib::XDefaultRootWindow(dpy), xlib::SubstructureRedirectMask);
        XSync(dpy, False);
        xlib::XSetErrorHandler(Some(error::xerror));
        XSync(dpy, False);
    }
}

macro_rules! init_cursor {
    ($dpy:expr, $ty:expr) => {{
        unsafe {xlib::XCreateFontCursor($dpy, $ty)}
    }};
}

pub fn setup(dpy: &mut xlib::Display) -> state::State {
    let mut state: state::State;
    {
        let screen =  unsafe { xlib::XDefaultScreen(dpy) };
        let root: u64 = unsafe { xlib::XRootWindow(dpy, screen) };
    
        state = state::State {
            screen: screen,
            root: root,
            cursor: Cursor {
                normal: init_cursor!(dpy, 68 /* XC left ptr */),
                resize: init_cursor!(dpy, 120 /* XC sizing */),
                mov: init_cursor!(dpy, 52  /* XC fleur */)
            },
            dpy: dpy,
            workspaces: Vec::new(),
            colors: unsafe { mem::zeroed() },
            active: Active {
                workspace: 0,
                window: root,
                focus_locked: false
            }
        };
    }
    
    state.colors = STYLE.colors.to_xft(&mut state);

    unsafe {
        XChangeWindowAttributes(state.dpy, state.root, CWEventMask | CWCursor,  &mut XSetWindowAttributes {
            background_pixmap: 0,
            background_pixel: 0,
            border_pixmap: xlib::CopyFromParent as u64,
            border_pixel: 0,
            bit_gravity: xlib::ForgetGravity,
            win_gravity: xlib::NorthWestGravity,
            backing_store: 0,
            backing_planes: 1,
            backing_pixel: 0,
            save_under: 0,
            event_mask: xlib::SubstructureRedirectMask | 
                        xlib::SubstructureNotifyMask | 
                        xlib::ButtonPressMask |
                        xlib::ButtonReleaseMask |
                        xlib::PointerMotionMask | 
                        xlib::EnterWindowMask | 
                        xlib::LeaveWindowMask | 
                        xlib::StructureNotifyMask | 
                        xlib::PropertyChangeMask | 
                        xlib::KeyPressMask | 
                        xlib::KeyReleaseMask,
            do_not_propagate_mask: 0,
            override_redirect: 0,
            colormap: xlib::CopyFromParent as u64,
            cursor: state.cursor.normal
        });
    }

    state
}

pub fn setup_keybindings(state: &mut State){
    for binding in unsafe { KEYBINDINGS.iter() } {
        unsafe {
            let keycode = xlib::XKeysymToKeycode(state.dpy, binding.key as u64);

            XGrabKey(
            state.dpy, keycode as i32, 
            binding.mdky, state.root, 
            True, GrabModeAsync, 
            GrabModeAsync);
        }
    }
}

macro_rules! mousemotion_grab {
    ($state: expr, $ty: ident) => {
        for mm in unsafe { MOUSEMOTIONS.$ty.iter() } {
            unsafe {
                XGrabButton(
                    $state.dpy,
                    mm.button,
                    mm.mdky,
                    $state.root,
                    True,
                    (ButtonPressMask | ButtonReleaseMask | PointerMotionMask) as u32,
                    GrabModeAsync,
                    GrabModeAsync,
                    0,
                    0
                );
            }
        }
    };
}

pub fn setup_mousemotions(state: &mut State){
    mousemotion_grab!(state, on_press);
    mousemotion_grab!(state, on_release);
}