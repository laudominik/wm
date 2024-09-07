use x11::xlib::{CWBorderWidth, False, Window, XConfigureWindow, XDisplayHeight, XDisplayWidth, XMapWindow, XMoveResizeWindow, XSetWindowBorder, XSync, XWindowChanges};
use std::{mem, process::exit};

use crate::{config::STYLE, state};

pub struct Space<'a> {
    pub tag: &'a str,
    pub windows: Vec<Window>
}

pub struct _Tile {
    pub coords: (i32, i32),
    pub size: (u32, u32)
}

impl state::State<'_> {
    pub fn cascade_autotiling(&mut self){
        let useless_gap: u32 = STYLE.useless_gap;
        let screen_width = unsafe{XDisplayWidth(self.dpy, self.screen) as u32};
        let screen_height = unsafe{XDisplayHeight(self.dpy, self.screen) as u32};
        let active_workspace = &mut self.workspaces[self.active_workspace];
        
        let maybe_latest_window: Option<&u64> = active_workspace.windows.last();
        if(maybe_latest_window.is_none()) { return };
        let latest_window = maybe_latest_window.unwrap();
        if active_workspace.windows.len() == 1 {
            latest_window.do_map(self, (
                useless_gap as i32, useless_gap as i32, 
                screen_width - useless_gap * 2, screen_height - useless_gap * 2));
            return;
        } 
        
        // it's cascade with special treatment for latest window
        // latest_window.do_map(self, (
        //     useless_gap as i32, useless_gap as i32,
        //     screen_width / 2 - useless_gap * 3/2, screen_height / 2 - useless_gap * 3/2
        // ));

        // active_workspace.windows[0].do_map(self, 
        //     (useless_gap + screen_width / 2, useless_gap + )
        // );
    }
}

trait WindowExt {
    fn do_map(self, state: &mut state::State, rect: (i32, i32, u32, u32));
}

impl WindowExt for Window {
    fn do_map(self, state: &mut state::State, rect: (i32, i32, u32, u32)){
        let mut wc: XWindowChanges = unsafe { mem::zeroed() };
        wc.border_width = STYLE.border_thickness as i32;

        unsafe {
            XConfigureWindow(state.dpy, self, CWBorderWidth.into(), &mut wc as *mut XWindowChanges);
            XSetWindowBorder(state.dpy, self, state.colors.normal.border.pixel);
            XMoveResizeWindow(state.dpy, self, 
                rect.0, rect.1,
                rect.2, rect.3);
            XMapWindow(state.dpy, self);
        }
    }
}

