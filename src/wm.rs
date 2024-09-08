use x11::xlib::{CWBorderWidth, CurrentTime, False, NoEventMask, RevertToNone, RevertToPointerRoot, Window, XConfigureWindow, XDestroyWindow, XDisplayHeight, XDisplayWidth, XEvent, XMapWindow, XMoveResizeWindow, XSendEvent, XSetInputFocus, XSetWindowBorder, XSync, XWindowChanges};
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

#[macro_export]
macro_rules! active_workspace_wins {
    ($state: expr) => {
        $state.workspaces[$state.active.workspace].windows
    };
}

impl state::State<'_> {

    pub fn focus(&mut self, window: Window){
        self.active.window = window;
    }

    pub fn focus_next(&mut self){
        let len =  active_workspace_wins!(self).len();
        if len == 0 { return; }

        if let Some(ix) = active_workspace_wins!(self).iter().position(|w| *w == self.active.window) {
            self.active.window = active_workspace_wins!(self)[(ix+1)%len];
        } else {
            self.active.window = active_workspace_wins!(self)[len - 1];
        }

        self.retile();
    }

    pub fn focus_previous(&mut self){
        let len =  active_workspace_wins!(self).len();
        if len == 0 { return; }

        if let Some(ix) = active_workspace_wins!(self).iter().position(|w| *w == self.active.window) {
            let mut previous: usize = 0;
            if ix == 0 { previous = len - 1;} 
            else {previous = ix - 1};
            self.active.window = active_workspace_wins!(self)[previous];
        } else {
            self.active.window = active_workspace_wins!(self)[0];
        }

        self.retile();
    }

    pub fn close_active(&mut self){
        if active_workspace_wins!(self).is_empty() { return;}

        unsafe {    
            XDestroyWindow(self.dpy, self.active.window);
        }
    }

    pub fn cascade_autotiling(&mut self){
        let useless_gap: u32 = STYLE.useless_gap;
        let border = STYLE.border_thickness;
        let screen_width = unsafe{XDisplayWidth(self.dpy, self.screen) as u32};
        let screen_height = unsafe{XDisplayHeight(self.dpy, self.screen) as u32};
        
        let maybe_latest_window: Option<&u64> = active_workspace_wins!(self).last();
        if maybe_latest_window.is_none() { return };
        
        let latest_window = maybe_latest_window.unwrap();
        if self.workspaces[self.active.workspace].windows.len() == 1 {
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

        let len_rest = active_workspace_wins!(self).len() - 1;
        let increment = screen_height / len_rest as u32;

        for i in 0..len_rest {
            let start_y = increment * i as u32 + useless_gap;

            active_workspace_wins!(self)[i].do_map(self, (
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
        let mut border_col = state.colors.normal.border.pixel;
        wc.border_width = STYLE.border_thickness as i32;

        if self == state.active.window { border_col = state.colors.selected.border.pixel; }

        unsafe {
            XConfigureWindow(state.dpy, self, CWBorderWidth.into(), &mut wc as *mut XWindowChanges);
            XSetWindowBorder(state.dpy, self, border_col);
            XMoveResizeWindow(state.dpy, self, 
                rect.0, rect.1,
                rect.2, rect.3);
            XMapWindow(state.dpy, self);
            if self == state.active.window { XSetInputFocus(state.dpy, self, RevertToPointerRoot, CurrentTime); }
        }
    }
}

