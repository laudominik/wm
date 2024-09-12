use x11::xlib::{Atom, CWBorderWidth, ClientMessage, ClientMessageData, CurrentTime, False, NoEventMask, PropModeReplace, RevertToNone, RevertToPointerRoot, SubstructureNotifyMask, SubstructureRedirectMask, True, Window, XChangeProperty, XClientMessageEvent, XConfigureWindow, XDestroyWindow, XDisplayHeight, XDisplayWidth, XEvent, XFlush, XGetWindowAttributes, XInternAtom, XLowerWindow, XMapWindow, XMoveResizeWindow, XSendEvent, XSetInputFocus, XSetWindowBorder, XSync, XUnmapWindow, XWindowAttributes, XWindowChanges, XA_CARDINAL};
use std::{ffi::CString, mem, process::exit};

use crate::{config::{CustomData, STYLE}, state};

pub struct Space<'a> {
    pub tag: &'a str,
    pub windows: Vec<Window>,
    pub custom: Option<CustomData> /* custom config for active workspace*/
}

pub struct _Tile {
    pub coords: (i32, i32),
    pub size: (u32, u32)
}

#[macro_export]
macro_rules! active_workspace {
    ($state: expr) => {
        $state.workspaces[$state.active.workspace]
    };
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

    fn set_workspace(&mut self, no: usize){
        let current_desktop: Atom = unsafe { XInternAtom(self.dpy,  CString::new("_NET_CURRENT_DESKTOP").unwrap().as_ptr(), False) };
        let raw = &no as *const _ as *const u8;
        unsafe { XChangeProperty(
            self.dpy, 
            self.root, 
            current_desktop, XA_CARDINAL, 32, PropModeReplace, raw, 1);
        }
        let mut msg_data = ClientMessageData::new();
        msg_data.set_long(0, no as i64);
        msg_data.set_long(1, CurrentTime as i64);

        let msg = XClientMessageEvent {
            type_: ClientMessage,
            serial: 0,
            send_event: False,
            display: self.dpy,
            window: self.root,
            message_type: 0,
            format: 32,
            data: msg_data
        };

        let raw2 = &msg as *const _ as *mut XEvent;
        
        unsafe { 
            XSendEvent(self.dpy, self.root, False, SubstructureNotifyMask | SubstructureRedirectMask, raw2);
            XFlush(self.dpy);
        };
        
        for window in active_workspace_wins!(self).iter() {
            unsafe { XUnmapWindow(self.dpy, *window) };
        }

        self.active.workspace = no;
        self.retile();
    }

    pub fn next_workspace(&mut self){
        self.set_workspace((self.active.workspace+1)%self.workspaces.len());
    }

    pub fn prev_workspace(&mut self){
        let mut new_no: usize = 0;

        if self.active.workspace == 0 {
            new_no = self.workspaces.len() - 1;
        } else {
            new_no = self.active.workspace - 1;
        }
        self.set_workspace(new_no);
    }

    pub fn send_active_window_to_workspace(&mut self, workspace_no: usize) {
        if workspace_no >= self.workspaces.len() { return }
        active_workspace_wins!(self).retain(|x| *x != self.active.window);
        unsafe { XUnmapWindow(self.dpy, self.active.window) };
        self.workspaces[workspace_no].windows.push(self.active.window);
        self.active.window = 0;
        self.retile();
    }

    pub fn goto_workspace(&mut self, workspace_no: usize){
        if workspace_no >= self.workspaces.len() { return }
        self.set_workspace(workspace_no);
    }

    pub fn close_active(&mut self){
        if active_workspace_wins!(self).is_empty() { return;}

        unsafe {    
            XDestroyWindow(self.dpy, self.active.window);
        }
    }

    pub fn cascade_autotiling(&mut self, windows: Vec<Window>){

        for window in windows.iter() {
            unsafe { XLowerWindow(self.dpy, *window) };       
        }

        let useless_gap: u32 = STYLE.useless_gap;
        let border = STYLE.border_thickness;
        let screen_width: u32 = unsafe{XDisplayWidth(self.dpy, self.screen) as u32};
        let screen_height = unsafe{XDisplayHeight(self.dpy, self.screen) as u32};
        
        let maybe_latest_window: Option<&u64> = windows.last();
        if maybe_latest_window.is_none() { return };
        
        let latest_window = maybe_latest_window.unwrap();
        if windows.len() == 1 {
            latest_window.do_map(self, (
                useless_gap as i32, useless_gap as i32, 
                screen_width - useless_gap * 2 - border * 2, screen_height - useless_gap * 2 - border * 2
            ));
            return;
        } 

        let mut middle = screen_width / 2;
        if let Some(custom) = &active_workspace!(self).custom {
            middle = custom.separator;
        }
        
        latest_window.do_map(self, (
            useless_gap as i32, useless_gap as i32,
            middle - useless_gap * 2 - border * 2, screen_height - useless_gap * 2 - border * 2
        ));

        let len_rest = windows.len() - 1;
        let increment = screen_height / len_rest as u32;

        for i in 0..len_rest {
            let start_y = increment * i as u32 + useless_gap;

            windows[i].do_map(self, (
                (useless_gap / 2 + middle) as i32, start_y as i32, 
                (screen_width - middle) - useless_gap * 2 - border * 2, increment - useless_gap * 2 - border * 2
            ));
        }
    }
}

pub trait WindowExt {
    fn do_map(self, state: &mut state::State, rect: (i32, i32, u32, u32));
    fn get_rect(self, state: &mut state::State) -> (i32, i32, u32, u32);
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

    fn get_rect(self, state: &mut state::State) -> (i32, i32, u32, u32) {
        let mut wa : XWindowAttributes = unsafe { mem::zeroed() };
        if( unsafe { XGetWindowAttributes(state.dpy, self, &mut wa) } == 0) { return (0,0,0,0); };
        return (wa.x, wa.y, wa.width as u32, wa.height as u32)
    }
}

