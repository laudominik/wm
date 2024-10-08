use std::ffi::CString;
use std::mem;
use std::ptr;
use chrono::Local;

use sysinfo::System;

use x11::xlib::ExposureMask;
use x11::xlib::{self, XSetWindowAttributes};
use x11::{xft, xrender};
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::state;
use crate::config::STYLE;

#[derive(PartialEq, Clone, Copy)]
pub enum Ctx {
    Retile, 
    Expose   
}


pub struct Widget {
    pub font: *mut xft::XftFont,
    pub wspec: Box<dyn WidgetSpec>
}

impl Widget {
    pub fn draw(&self, state: &mut state::State, context:Ctx){
        self.wspec.draw(state, self, context);
    }

    pub fn new(state: &mut state::State, font: &str, wspec: Box<dyn WidgetSpec>) -> Widget {
        Widget {
            font: unsafe { xft::XftFontOpenName(state.dpy, state.screen, font.as_ptr() as *const i8) },
            wspec: wspec
        }
    }
}

pub trait WidgetSpec {
    fn draw(&self, state: &mut state::State, widget: &Widget, context: Ctx);
}

pub struct TopBar {}
pub struct TaskList {}
pub struct Stats {}

impl WidgetSpec for TopBar {
    fn draw(&self, state: &mut state::State, widget: &Widget, context: Ctx) {
        unsafe {
            let screen_width: u32 = xlib::XDisplayWidth(state.dpy, state.screen) as u32;
            let box_wh = STYLE.paddings.top;
            let pad: i32 = text_width_px(state, widget.font, state.workspaces[0].tag) / 2;
            
            if context == Ctx::Expose {
                xft::XftDrawRect(state.xft_draw, &state.colors.normal.bg, 0, 0, screen_width, STYLE.paddings.top);
            }
        
            for i in 0..state.workspaces.len() {
                let offset = i as u32 * box_wh;
                let mut bgcol = &state.colors.normal.bg;
                let mut fgcol = &state.colors.normal.fg;
                if i == state.active.workspace { 
                    bgcol = &state.colors.normal.fg;
                    fgcol = &state.colors.normal.bg;
                };
                let utf8_string = CString::new(state.workspaces[i].tag).unwrap();
                xft::XftDrawRect(state.xft_draw, bgcol, offset as i32, 0, box_wh, box_wh);
                xft::XftDrawStringUtf8(state.xft_draw, fgcol, widget.font, offset as i32 + pad, box_wh as i32 - pad, utf8_string.as_ptr() as *const u8, utf8_string.to_bytes().len() as i32);
            }
        } 
    }
}

impl WidgetSpec for TaskList {
    fn draw(&self, _: &mut state::State, __: &Widget, ___: Ctx) {}
}

lazy_static! {
    static ref SYS: Mutex<System> = Mutex::new(System::new_all());
}

impl WidgetSpec for Stats {
    fn draw(&self, state: &mut state::State, widget: &Widget, context: Ctx) {
        if context != Ctx::Expose { return }
        let mut sys = SYS.lock().unwrap();
        let screen_width = unsafe { xlib::XDisplayWidth(state.dpy, state.screen) as i32 };
        sys.refresh_all();

        let mem_usg = (sys.used_memory() as f32 / sys.total_memory() as f32) * 100f32;

        let stats = [
            format!("{:02} % CPU", sys.global_cpu_usage() as u32),
            format!("{}", Local::now()),
            format!("{:02} % ({} / {} MiB)", mem_usg as u32, sys.used_memory() / 1024 / 1024, sys.total_memory() / 1024 / 1024)
        ];

        let mut offset = 0;
        let brk = text_width_px(state, widget.font, "A");
        let pad = brk / 2;

        for stat in stats.iter() {
            let w = text_width_px(state, widget.font, stat.as_str());
            offset += w;
            offset += brk;

            unsafe {
                let utf8_string: CString = CString::new((*stat).clone()).unwrap();
                xft::XftDrawRect(state.xft_draw, &state.colors.normal.fg, screen_width - offset - brk, 0, (brk + w + brk) as u32, STYLE.paddings.top);
                xft::XftDrawStringUtf8(state.xft_draw, &state.colors.normal.bg, widget.font, screen_width - offset, STYLE.paddings.top as i32 - pad, utf8_string.as_ptr() as *const u8, utf8_string.to_bytes().len() as i32);
            }
        }
    }
}

pub fn widget_refresh() {

    println!("Refresh!");
    unsafe {
        let dpy: *mut xlib::_XDisplay = xlib::XOpenDisplay(ptr::null());
        let root_window: u64 = xlib::XDefaultRootWindow(dpy);
        
        let mut event = xlib::XEvent {
            expose: xlib::XExposeEvent {
                type_: xlib::Expose,
                serial: 0,
                send_event: 0,
                display: ptr::null_mut(),
                window: 69,
                x: 0,
                y: 0,
                width: 0,
                height: 0,
                count: 0,
            }
        };

        xlib::XSendEvent(dpy, root_window, 0, ExposureMask, &mut event);
        xlib::XFlush(dpy);
    }

}

pub fn widget_window(dpy: *mut xlib::Display ) -> (xlib::Window, *mut xft::XftDraw)  {
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

fn text_width_px(state: &mut state::State, font: *mut xft::XftFont, string: &str) -> i32 {
    unsafe {
        let mut extents: xrender::XGlyphInfo = std::mem::zeroed();
        let utf8_string = CString::new(string).unwrap();
        // Calculate the text extents
        xft::XftTextExtentsUtf8(
            state.dpy,
            font,
            utf8_string.as_ptr() as *const u8,
            utf8_string.to_bytes().len() as i32,
            &mut extents,
        );

        return extents.width as i32;
    }   
}