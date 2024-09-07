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
        let border = STYLE.border_thickness;
        let screen_width = unsafe{XDisplayWidth(self.dpy, self.screen) as u32};
        let screen_height = unsafe{XDisplayHeight(self.dpy, self.screen) as u32};
        
        let maybe_latest_window: Option<&u64> = self.workspaces[self.active_workspace].windows.last();
        if(maybe_latest_window.is_none()) { return };
        
        let latest_window = maybe_latest_window.unwrap();
        if self.workspaces[self.active_workspace].windows.len() == 1 {
            latest_window.do_map(self, (
                useless_gap as i32, useless_gap as i32, 
                screen_width - useless_gap * 2 - border * 2, screen_height - useless_gap * 2 - border * 2
            ));
            return;
        } 
        
        latest_window.do_map(self, (
            useless_gap as i32, useless_gap as i32,
            screen_width / 2 - useless_gap * 2 - border * 2, screen_height - useless_gap * 2 - border * 2
        ));

        let len_rest = self.workspaces[self.active_workspace].windows.len() - 1;
        let increment = screen_height / len_rest as u32;

        for i in 0..len_rest {
            let start_y = increment * i as u32 + useless_gap;

            self.workspaces[self.active_workspace].windows[i].do_map(self, (
                (useless_gap / 2 + screen_width / 2) as i32, start_y as i32, 
                screen_width / 2 - useless_gap * 2 - border * 2, increment - useless_gap * 2 - border * 2
            ));
        }
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

