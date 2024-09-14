use std::mem;

use x11::xlib::{self, XSetWindowAttributes};
use x11::xft;

use crate::state;
use crate::config::STYLE;


pub trait Widget {
    fn draw(&self, state: &mut state::State);
}

pub struct TopBar {}

impl Widget for TopBar {
    fn draw(&self, state: &mut state::State) {
        unsafe {
            let screen_width: u32 = xlib::XDisplayWidth(state.dpy, state.screen) as u32;
            let box_wh = STYLE.paddings.top;
            xft::XftDrawRect(state.xft_draw, &state.colors.normal.bg, 0, 0, screen_width, STYLE.paddings.top);

            for i in 0..state.workspaces.len() {
                let offset = i as u32 * box_wh;
                let mut bgcol = &state.colors.normal.bg;
                if i == state.active.workspace { bgcol = &state.colors.normal.fg };
                xft::XftDrawRect(state.xft_draw, bgcol, offset as i32, 0, box_wh, box_wh);
            }
        } 
    }
}

pub fn widget_window(dpy: &mut xlib::Display ) -> (xlib::Window, *mut xft::XftDraw)  {
    unsafe {
        let screen = xlib::XDefaultScreen(dpy);
        let root: u64 = xlib::XRootWindow(dpy, screen);
        let screen_width: u32 = xlib::XDisplayWidth(dpy, screen) as u32;
        let _screen_height = xlib::XDisplayHeight(dpy, screen) as u32;

        let mut wa : XSetWindowAttributes = mem::zeroed();
        wa.override_redirect = xlib::True;
        wa.background_pixmap = xlib::ParentRelative as u64;
        wa.background_pixel = 0;
        wa.event_mask = xlib::ButtonPressMask | xlib::ExposureMask;

        let win = xlib::XCreateWindow(dpy, root, 0, 0, screen_width, STYLE.paddings.top, 0, xlib::XDefaultDepth( dpy, screen),
        xlib::CopyFromParent as u32, xlib::XDefaultVisual(dpy, screen),
        xlib::CWEventMask, &mut wa);
        xlib::XMapWindow(dpy, win);

        let xft_draw = xft::XftDrawCreate(dpy, win, xlib::XDefaultVisual(dpy, screen), xlib::XDefaultColormap(dpy, screen));
        return (win, xft_draw)
    }
}