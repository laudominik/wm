use x11::xlib::{self, XGrabServer};
use std::mem;

use crate::{config::{CustomData, STYLE}, state};

pub struct Space<'a> {
    pub tag: &'a str,
    pub windows: Vec<xlib::Window>,
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

    pub fn focus(&mut self, window: xlib::Window){
        unsafe { xlib::XRaiseWindow(self.dpy, window) };       
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
            let mut previous: usize = len - 1;
            if ix != 0 { previous = ix - 1; } 
            self.active.window = active_workspace_wins!(self)[previous];
        } else {
            self.active.window = active_workspace_wins!(self)[0];
        }

        self.retile();
    }

    fn set_workspace(&mut self, no: usize){
        for window in active_workspace_wins!(self).iter() {
            unsafe { xlib::XUnmapWindow(self.dpy, *window) };
        }

        self.active.workspace = no;
        self.retile();
    }

    pub fn next_workspace(&mut self){
        self.set_workspace((self.active.workspace+1)%self.workspaces.len());
    }

    pub fn prev_workspace(&mut self){
        let mut new_no: usize = self.workspaces.len() - 1;

        if self.active.workspace != 0 {
            new_no = self.active.workspace - 1;
        } 
        self.set_workspace(new_no);
    }

    pub fn send_active_window_to_workspace(&mut self, workspace_no: usize) {
        if workspace_no >= self.workspaces.len() { return }
        active_workspace_wins!(self).retain(|x| *x != self.active.window);
        unsafe { xlib::XUnmapWindow(self.dpy, self.active.window) };
        self.workspaces[workspace_no].windows.push(self.active.window);
        self.active.window = 0;
        self.retile();
    }

    pub fn goto_workspace(&mut self, workspace_no: usize){
        if workspace_no >= self.workspaces.len() { return }
        self.set_workspace(workspace_no);
    }

    pub fn close_active(&mut self){
        if active_workspace_wins!(self).is_empty() { return; }
        active_workspace_wins!(self).retain(|x| *x != self.active.window);
        self.retile();
        self.kill_window_process(self.active.window);
    }

    fn window_exists(&self, window: xlib::Window) -> bool {
        for workspace in self.workspaces.iter() {
            for win in workspace.windows.iter() {
                if *win == window { return true; }
            }
        }
        return false;
    }

    pub fn kill_window_process(&mut self, window: xlib::Window) {
        unsafe {
            let wm_protocols = xlib::XInternAtom(self.dpy, "WM_PROTOCOLS".as_ptr() as *const i8, 0);
            let wm_delete_window = xlib::XInternAtom(self.dpy, "WM_DELETE_WINDOW".as_ptr() as *const i8, 0);
    
            let mut event = x11::xlib::XClientMessageEvent {
                type_: xlib::ClientMessage,
                serial: 0,
                send_event: 1,
                display: self.dpy,
                window: window,
                message_type: wm_protocols,
                format: 32,
                data: std::mem::zeroed(),
            };
    
            event.data.set_long(0, wm_delete_window as i64);
            event.data.set_long(0, x11::xlib::CurrentTime as i64);
    
            xlib::XSendEvent(
                self.dpy,
                window,
                0,
                0,
                &mut event as *mut _ as *mut _,
            );
    
            xlib::XSync(self.dpy, 0);

            XGrabServer(self.dpy);
            xlib::XSetCloseDownMode(self.dpy, xlib::DestroyAll);
            xlib::XKillClient(self.dpy, window);
            xlib::XSync(self.dpy, xlib::False);
            xlib::XUngrabServer(self.dpy);
        }
    }

    pub fn cascade_autotiling(&mut self, windows: Vec<xlib::Window>){

        for window in windows.iter() {
            unsafe { xlib::XLowerWindow(self.dpy, *window) };       
        }

        let useless_gap: u32 = STYLE.useless_gap;
        let border = STYLE.border_thickness;
        let screen_width: u32 = unsafe{xlib::XDisplayWidth(self.dpy, self.screen) as u32};
        let screen_height = unsafe{xlib::XDisplayHeight(self.dpy, self.screen) as u32};
        
        let maybe_latest_window: Option<&u64> = windows.last();
        if maybe_latest_window.is_none() { return };
        
        let latest_window = maybe_latest_window.unwrap();
        if windows.len() == 1 {
            latest_window.do_map(self, (
                (useless_gap + STYLE.paddings.left) as i32, (useless_gap + STYLE.paddings.top) as i32, 
                screen_width - useless_gap * 2 - border * 2 - STYLE.paddings.left - STYLE.paddings.right, screen_height - useless_gap * 2 - border * 2 - STYLE.paddings.top - STYLE.paddings.bottom
            ));
            return;
        } 

        let mut middle = screen_width / 2;
        if let Some(custom) = &active_workspace!(self).custom {
            middle = custom.separator;
        }
        
        latest_window.do_map(self, (
            (useless_gap + STYLE.paddings.left) as i32, (useless_gap  + STYLE.paddings.top) as i32,
            middle - useless_gap * 2 - border * 2 - STYLE.paddings.left, screen_height - useless_gap * 2 - border * 2 - STYLE.paddings.top - STYLE.paddings.bottom
        ));

        let len_rest = windows.len() - 1;
        let increment = (screen_height - STYLE.paddings.top - STYLE.paddings.bottom) / len_rest as u32;

        for i in 0..len_rest {
            let start_y = increment * i as u32 + useless_gap;

            windows[i].do_map(self, (
                (useless_gap / 2 + middle) as i32, (start_y + STYLE.paddings.top) as i32, 
                (screen_width - middle) - useless_gap * 2 - border * 2 - STYLE.paddings.right, increment - useless_gap * 2 - border * 2
            ));
        }
    }
}

pub trait WindowExt {
    fn do_map(self, state: &mut state::State, rect: (i32, i32, u32, u32));
    fn get_rect(self, state: &mut state::State) -> (i32, i32, u32, u32);
}

impl WindowExt for xlib::Window {
    fn do_map(self, state: &mut state::State, rect: (i32, i32, u32, u32)){
        if !state.window_exists(self) { 
            println!("xroagwem: warning - mapping a deleted window");
            return 
        }

        let mut wc: xlib::XWindowChanges = unsafe { mem::zeroed() };
        let mut border_col = state.colors.normal.border.pixel;
        wc.border_width = STYLE.border_thickness as i32;

        if self == state.active.window { border_col = state.colors.selected.border.pixel; }

        unsafe {
            xlib::XConfigureWindow(state.dpy, self, xlib::CWBorderWidth.into(), &mut wc as *mut xlib::XWindowChanges);
            xlib::XSetWindowBorder(state.dpy, self, border_col);
            xlib::XMoveResizeWindow(state.dpy, self, 
                rect.0, rect.1,
                rect.2, rect.3);
                xlib::XMapWindow(state.dpy, self);
            if self == state.active.window { xlib::XSetInputFocus(state.dpy, self, xlib::RevertToPointerRoot, xlib::CurrentTime); }
        }
    }

    fn get_rect(self, state: &mut state::State) -> (i32, i32, u32, u32) {
        let mut wa : xlib::XWindowAttributes = unsafe { mem::zeroed() };
        if( unsafe { xlib::XGetWindowAttributes(state.dpy, self, &mut wa) } == 0) { return (0,0,0,0); };
        return (wa.x, wa.y, wa.width as u32, wa.height as u32)
    }
}

