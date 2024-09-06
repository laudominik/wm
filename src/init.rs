use x11::xlib::{PropModeReplace, XDefaultGC, XFlush, XMapWindow, XWhitePixel, XA_WINDOW};
use x11::xlib::{self, False, XChangeProperty, XCreateSimpleWindow, XCreateWindow, XSync};
use std::ffi::CStr;
use std::ffi::CString;
use crate::state::{WmAtoms, NetAtoms};

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

macro_rules! init_atom {
    ($dpy:expr, $name:expr) => {{
        let c_name = CString::new($name).unwrap();
        unsafe {xlib::XInternAtom($dpy, c_name.as_ptr(), xlib::False)}
    }};
}

macro_rules! init_atoms {
    ($dpy:expr, [ $($name:expr),* ]) => {{
        vec![
            $( init_atom!($dpy, $name), )*
        ]
    }};
}

pub fn setup(dpy: &mut xlib::Display) {
    let state = state::State {
        wmatom: WmAtoms { 
            protocols: init_atom!(dpy, "WM_PROTOCOLS"), 
            delete: init_atom!(dpy, "WM_DELETE_WINDOW"), 
            state: init_atom!(dpy, "WM_STATE"), 
            take_focus: init_atom!(dpy, "WM_TAKE_FOCUS") 
        },
        netatom: NetAtoms { 
            active_window: init_atom!(dpy, "ACTIVE_WINDOW"), 
            supported: init_atom!(dpy, "_NET_SUPPORTED"), 
            state: init_atom!(dpy, "_NET_WM_STATE"),
            check: init_atom!(dpy, "_NET_SUPPORTING_WM_CHECK"),
            fullscreen: init_atom!(dpy, "FULLSCREEN"),
            wtype: init_atom!(dpy, "_NET_WINDOW_TYPE")
        }
    };

    unsafe {
        let screen =  xlib::XDefaultScreen(dpy);
        let root: u64 = xlib::XRootWindow(dpy, screen);
        let wmwindow : xlib::Window = XCreateSimpleWindow(dpy, root, 10, 10, 100, 100, 1, 0, XWhitePixel(dpy, screen));
        xlib::XFillRectangle(dpy, wmwindow, XDefaultGC(dpy, screen), 20, 20, 10, 10);
        xlib::XMapWindow(dpy, wmwindow);

        xlib::XFlush(dpy);
        
        loop {}
    }

}