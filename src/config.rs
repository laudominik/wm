use std::collections::HashSet;
use std::f32::INFINITY;
use std::process::Command;
use std::sync::Arc;
use std::mem;

use x11::keysym::{XK_Down, XK_Left, XK_Return, XK_Right, XK_Up, XK_c, XK_downarrow, XK_f, XK_h, XK_j, XK_k, XK_l, XK_minus, XK_plus, XK_r, XK_space, XK_uparrow, XK_w, XK_1, XK_2, XK_3, XK_4};
use x11::xlib::{ControlMask, Mod1Mask, Mod3Mask, Mod4Mask, ShiftMask, Window, XDisplayHeight, XDisplayWidth, XGetWindowAttributes, XRaiseWindow, XWindowAttributes};

use crate::state::{self, Keybinding, Mousemotion, State, KEYBINDINGS, MOUSEMOTIONS};
use crate::style::{ColorScheme, ColorSchemes, Style};
use crate::wm::WindowExt;
use crate::{active_workspace, active_workspace_wins, set_keybinding, set_mousemotion, set_spaces, spawn_with_shell, wm};

macro_rules! toggle_active_window_prop {
    ($state: expr, $set: ident) => {
        if let Some(custom) = &mut active_workspace!($state).custom {
            if let Some(_)  = custom.$set.iter().position(|x| *x == $state.active.window) {
                custom.$set.remove(&$state.active.window);
            } else {
                custom.$set.insert($state.active.window);
            }
            $state.retile();
        }
    };
}

pub static STYLE: Style = Style {
    colors: ColorSchemes {
       normal:  ColorScheme {
            fg: "#024442",
            bg: "#ffffff",
            border: "#8b9458"
       },
       selected: ColorScheme {
            fg: "#ffff00",
            bg: "#ffffff",
            border: "#ffff00"
       }
    },
    border_thickness: 5,
    useless_gap: 5
};

const MODKEY: u32 = Mod4Mask;
const MODKEY_SHIFT: u32 = MODKEY | ShiftMask;
const MODKEY_CTRL: u32 = MODKEY | ControlMask;

/* your private config goes here */
pub fn make(state: &mut state::State){
    /* mouse motion */
    {
        set_mousemotion!( modkey: MODKEY, callback: |state, pt| {state.rightclick_grab(pt)}, mousebutton: 3, onpress );
        set_mousemotion!( modkey: MODKEY, callback: |state, pt| {state.rightclick_release(pt)}, mousebutton: 3, onrelease );
        set_mousemotion!( modkey: MODKEY, callback: |state, pt| {state.leftclick_grab(pt)}, mousebutton: 1, onpress );
        set_mousemotion!( modkey: MODKEY, callback: |state, pt| {state.leftclick_release(pt)}, mousebutton: 1, onrelease );
        set_mousemotion!( modkey: MODKEY, callback: |state, pt| {state.mouse_move(pt)}, onmove );
    }
    
    /* keybindings */
    {
        set_keybinding!( modkey: MODKEY, callback: |_| {spawn_with_shell!("alacritty");}, key: XK_Return );
        set_keybinding!( modkey: MODKEY, callback: |_| {spawn_with_shell!("dmenu_run");}, key: XK_r );
        set_keybinding!( modkey: MODKEY, callback: |state| {state.focus_next();}, key: XK_j );
        set_keybinding!( modkey: MODKEY, callback: |state| {state.focus_previous();}, key: XK_k );
        set_keybinding!( modkey: MODKEY, callback: |state| {toggle_active_window_prop!(state, fullscreen_windows);}, key: XK_f );
        set_keybinding!( modkey: MODKEY, callback: |state| {state.separator_modify(40)}, key: XK_l );
        set_keybinding!( modkey: MODKEY, callback: |state| {state.separator_modify(-40)}, key: XK_h );
        set_keybinding!( modkey: MODKEY, callback: |state| { state.next_workspace(); }, key: XK_Right );
        set_keybinding!( modkey: MODKEY, callback: |state| { state.prev_workspace(); }, key: XK_Left );
        set_keybinding!( modkey: MODKEY, callback: |state| { state.goto_workspace(0); }, key: XK_1 );
        set_keybinding!( modkey: MODKEY, callback: |state| { state.goto_workspace(1); }, key: XK_2 );
        set_keybinding!( modkey: MODKEY, callback: |state| { state.goto_workspace(2); }, key: XK_3 );
        set_keybinding!( modkey: MODKEY, callback: |state| { state.goto_workspace(3); }, key: XK_4 );
        set_keybinding!( modkey: MODKEY_SHIFT, callback: |state| { state.send_active_window_to_workspace(0); }, key: XK_1 );
        set_keybinding!( modkey: MODKEY_SHIFT, callback: |state| { state.send_active_window_to_workspace(1); }, key: XK_2 );
        set_keybinding!( modkey: MODKEY_SHIFT, callback: |state| { state.send_active_window_to_workspace(2); }, key: XK_3 );
        set_keybinding!( modkey: MODKEY_SHIFT, callback: |state| { state.send_active_window_to_workspace(3); }, key: XK_4 );
        set_keybinding!( modkey: MODKEY_SHIFT, callback: |state| {state.close_active();}, key: XK_c );
        set_keybinding!( modkey: MODKEY_SHIFT, callback: |state| {state.active_floating_move(0, 40);}, key: XK_Down );
        set_keybinding!( modkey: MODKEY_SHIFT, callback: |state| {state.active_floating_move(0, -40);}, key: XK_Up );
        set_keybinding!( modkey: MODKEY_SHIFT, callback: |state| {state.active_floating_move(-40, 0);}, key: XK_Left );
        set_keybinding!( modkey: MODKEY_SHIFT, callback: |state| {state.active_floating_move(40, 0);}, key: XK_Right );
        set_keybinding!( modkey: MODKEY_SHIFT, callback: |state| {state.active_floating_resize(40, 40);}, key: XK_plus );
        set_keybinding!( modkey: MODKEY_SHIFT, callback: |state| {state.active_floating_resize(-40, -40);}, key: XK_minus );
        set_keybinding!( modkey: MODKEY_CTRL, callback: |state| {toggle_active_window_prop!(state, floating_windows);}, key: XK_space );
    }

    /* startup apps */
    {
        spawn_with_shell!("nitrogen", ["--restore"]);
        spawn_with_shell!("picom");
    }

    /* default workspaces config */
    {
        set_spaces!(state, ["一", "二", "三", "四"]);
        let screen_width = unsafe{XDisplayWidth(state.dpy, state.screen) as u32};

        for space in state.workspaces.iter_mut() {
            space.custom = Some(CustomData {
                separator: screen_width/2,
                fullscreen_windows: HashSet::new(),
                floating_windows: HashSet::new(),
                rightclick_grab_origin: (0,0),
                rightclick_grab_window: 0,
                rightclick_grabbing: false,
                leftclick_grab_window: 0,
                leftclick_d: (0, 0),
                leftclick_grabbing: false
            });
        }
    }
}

pub struct CustomData {
    pub separator: u32 /* used by cascade_autotiling */,
    pub fullscreen_windows: HashSet<Window>,
    pub floating_windows: HashSet<Window>,
    pub rightclick_grab_origin: (i32, i32),
    pub rightclick_grab_window: Window,
    pub rightclick_grabbing: bool,
    pub leftclick_d: (i32, i32),
    pub leftclick_grab_window: Window,
    pub leftclick_grabbing: bool
}

macro_rules! is_floating {
    ($state: expr, $window: expr) => {
        active_workspace!($state).custom.as_ref().unwrap().floating_windows.contains($window)
    };
}

macro_rules! custom {
    ($state: expr) => {
        active_workspace!($state).custom.as_mut().unwrap()
    };
}

impl state::State<'_> {
    pub fn retile(&mut self){
        /* configurable grouping logic */
        let mut tiled_windows: Vec<Window> = Vec::new();
        let mut fullscreen_windows: Vec<Window> = Vec::new();
        let mut floating_windows: Vec<Window> = Vec::new();

        if let Some(_) = &mut active_workspace!(self).custom {
            for window in active_workspace!(self).windows.iter(){
                if active_workspace!(self).custom.as_ref().unwrap().fullscreen_windows.contains(window) {
                    fullscreen_windows.push(*window);
                    continue;
                } else if is_floating!(self, window) {
                    floating_windows.push(*window);
                    continue;
                }
                tiled_windows.push(*window);
            }
        } else {
            tiled_windows =  active_workspace_wins!(self).clone();
        }   

        /* configurable tiling logic */
        self.cascade_autotiling(tiled_windows);     
        self.draw_floating_windows(&floating_windows);
        self.draw_fullscreen_windows(&fullscreen_windows);
    }

    fn separator_modify(&mut self, modifier: i32) {
        if let Some(custom ) = &mut active_workspace!(self).custom {
            custom.separator = (custom.separator as i32 + modifier).clamp(100, 1760) as u32;
            self.retile();
        }
    }
    
    fn draw_fullscreen_windows(&mut self, windows: &Vec<Window>){
        let screen_width: u32 = unsafe{XDisplayWidth(self.dpy, self.screen) as u32};
        let screen_height = unsafe{XDisplayHeight(self.dpy, self.screen) as u32};
    
        for window in windows {
            unsafe { XRaiseWindow(self.dpy, *window) };       
            window.do_map(self, (0, 0, screen_width, screen_height));
        }
    }

    fn draw_floating_windows(&mut self, windows: &Vec<Window>){
        for window in windows.iter() {
            let rect = window.get_rect(self);
            window.do_map(self,  rect);    
        }
    }

    fn active_floating_resize(&mut self, dw: i32, dh: i32) {
        if let Some(custom) = &active_workspace!(self).custom {
            if !custom.floating_windows.contains(&self.active.window) { return };
            let mut rect = self.active.window.get_rect(self);
            rect.2 = (rect.2 as i32 + dw).clamp(100, i32::MAX) as u32;
            rect.3 = (rect.3 as i32 + dh).clamp(100, i32::MAX) as u32;
            self.active.window.do_map(self,  rect);
        }
        
    }

    fn active_floating_move(&mut self, dx: i32, dy: i32) {
        if let Some(custom) = &active_workspace!(self).custom {
            if !custom.floating_windows.contains(&self.active.window) { return };
            let mut rect = self.active.window.get_rect(self);
            rect.0 += dx;
            rect.1 += dy;
            self.active.window.do_map(self,  rect);
        }
    }

    fn leftclick_grab(&mut self, (x, y): (i32, i32)){
        if active_workspace!(self).custom.is_none() { return };
        custom!(self).leftclick_grab_window = self.active.window;
        custom!(self).leftclick_grabbing = true;
        self.active.focus_locked = true;
        let rect = self.active.window.get_rect(self);
        custom!(self).leftclick_d = (x - rect.0, y - rect.1);
    }

    fn leftclick_release(&mut self, pt: (i32, i32)){
        if active_workspace!(self).custom.is_none() { return };
        custom!(self).leftclick_grabbing = false;
        self.active.focus_locked = false;
    }

    fn rightclick_grab(&mut self, pt: (i32, i32)){
        if active_workspace!(self).custom.is_none() { return };
        custom!(self).rightclick_grab_origin = pt;
        custom!(self).rightclick_grabbing = true;
        self.active.focus_locked = true;
        custom!(self).rightclick_grab_window = self.active.window;
    }

    fn rightclick_release(&mut self, (x, y): (i32, i32)){
        if let Some(custom) = &mut active_workspace!(self).custom {
            custom.rightclick_grab_origin = (0,0);
            custom.rightclick_grabbing = false;
            self.active.focus_locked = false;
        }
    }
    
    fn mouse_move(&mut self, (x, y): (i32, i32)){
        if active_workspace!(self).custom.is_none() { return; }
        
        if active_workspace!(self).custom.as_ref().unwrap().rightclick_grabbing {
            if is_floating!(self, &active_workspace!(self).custom.as_ref().unwrap().rightclick_grab_window) {
                let rect: (i32, i32, u32, u32) = custom!(self).rightclick_grab_window.get_rect(self);
                let difference_x = (x - rect.0).abs() as u32;
                let difference_y = (y - rect.1).abs() as u32;

                if (x - custom!(self).rightclick_grab_origin.0).abs() > 50
                || (y - custom!(self).rightclick_grab_origin.1).abs() > 50
                {
                    custom!(self).rightclick_grab_window.do_map(self, (rect.0, rect.1, difference_x, difference_y));
                    custom!(self).rightclick_grab_origin.0 = x;
                    custom!(self).rightclick_grab_origin.1 = y;
                }
                return;
            }

            active_workspace!(self).custom.as_mut().unwrap().separator = x as u32;
            if (x - active_workspace!(self).custom.as_ref().unwrap().rightclick_grab_origin.0).abs() > 50 {
                self.retile();
                active_workspace!(self).custom.as_mut().unwrap().rightclick_grab_origin.0 = x;
            }
            return;
        }

        
        if active_workspace!(self).custom.as_ref().unwrap().leftclick_grabbing {
            if is_floating!(self, &active_workspace!(self).custom.as_ref().unwrap().leftclick_grab_window) {
                let rect: (i32, i32, u32, u32) = custom!(self).leftclick_grab_window.get_rect(self);
                let x_new = x - custom!(self).leftclick_d.0;
                let y_new = y - custom!(self).leftclick_d.1;

                if (x_new - rect.0).abs() > 50
                || (y_new - rect.1).abs() > 50 {
                    custom!(self).leftclick_grab_window.do_map(self, (x_new, y_new, rect.2, rect.3));
                }
                return;
            }
        }
    }

}

