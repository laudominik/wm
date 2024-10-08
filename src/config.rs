use std::collections::HashSet;
use std::process::Command;
use std::sync::Arc;
use std::time::Duration;

use x11::keysym;
use x11::xlib;

use crate::add_widget;
use crate::state::WIDGETS;
use crate::state::{self, Keybinding, Mousemotion, KEYBINDINGS, MOUSEMOTIONS};
use crate::style::Paddings;
use crate::style::{ColorScheme, ColorSchemes, Style};
use crate::widgets::Ctx;
use crate::widgets::Stats;
use crate::widgets::{TopBar, Widget};
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
    useless_gap: 5,
    paddings: Paddings {
        top: 20,
        bottom: 0,
        left: 0,
        right: 0
    }
};

pub static WIDGET_REFRESH: Duration = Duration::from_secs(15);

const MODKEY: u32 = xlib::Mod4Mask;
const MODKEY_SHIFT: u32 = MODKEY |  xlib::ShiftMask;
const MODKEY_CTRL: u32 = MODKEY |  xlib::ControlMask;

/* your private config goes here */
pub fn make(state: &mut state::State){
    /* widgets */
    {
        add_widget!(state, TopBar, "Noto Sans CJK JP-12");
        add_widget!(state, Stats, "Noto Sans-12");
    }

    /* mouse motion */
    {
        set_mousemotion!( on_press, modkey: MODKEY, callback: |state, pt, _| { state.rightclick_grab(pt)}, mousebutton: 3);
        set_mousemotion!( on_release, modkey: MODKEY, callback: |state, pt, _| { state.rightclick_release(pt)}, mousebutton: 3 );
        set_mousemotion!( on_press, modkey: MODKEY, callback: |state, pt, _| { state.leftclick_grab(pt)}, mousebutton: 1 );
        set_mousemotion!( on_release, modkey: MODKEY, callback: |state, pt, _| { state.leftclick_release(pt)}, mousebutton: 1 );
        set_mousemotion!( on_move, modkey: MODKEY, callback: |state, pt, _| { state.mouse_move(pt)}, nobutton );
        set_mousemotion!( on_cross, modkey: MODKEY, callback: |state, _, window| { state.mouse_cross(window)}, nobutton );
    }
    
    /* keybindings */
    {
        set_keybinding!( modkey: MODKEY, callback: |_| {spawn_with_shell!("alacritty");}, key: keysym::XK_Return );
        set_keybinding!( modkey: MODKEY, callback: |_| {spawn_with_shell!("dmenu_run");}, key: keysym::XK_r );
        set_keybinding!( modkey: MODKEY, callback: |state| {state.focus_next();}, key: keysym::XK_j );
        set_keybinding!( modkey: MODKEY, callback: |state| {state.focus_previous();}, key: keysym::XK_k );
        set_keybinding!( modkey: MODKEY, callback: |state| {toggle_active_window_prop!(state, fullscreen_windows);}, key: keysym::XK_f );
        set_keybinding!( modkey: MODKEY, callback: |state| {state.separator_modify(40)}, key: keysym::XK_l );
        set_keybinding!( modkey: MODKEY, callback: |state| {state.separator_modify(-40)}, key: keysym::XK_h );
        set_keybinding!( modkey: MODKEY, callback: |state| { state.next_workspace(); }, key: keysym::XK_Right );
        set_keybinding!( modkey: MODKEY, callback: |state| { state.prev_workspace(); }, key: keysym::XK_Left );
        set_keybinding!( modkey: MODKEY, callback: |state| { state.goto_workspace(0); }, key: keysym::XK_1 );
        set_keybinding!( modkey: MODKEY, callback: |state| { state.goto_workspace(1); }, key: keysym::XK_2 );
        set_keybinding!( modkey: MODKEY, callback: |state| { state.goto_workspace(2); }, key: keysym::XK_3 );
        set_keybinding!( modkey: MODKEY, callback: |state| { state.goto_workspace(3); }, key: keysym::XK_4 );
        set_keybinding!( modkey: MODKEY_SHIFT, callback: |state| { state.send_active_window_to_workspace(0); }, key: keysym::XK_1 );
        set_keybinding!( modkey: MODKEY_SHIFT, callback: |state| { state.send_active_window_to_workspace(1); }, key: keysym::XK_2 );
        set_keybinding!( modkey: MODKEY_SHIFT, callback: |state| { state.send_active_window_to_workspace(2); }, key: keysym::XK_3 );
        set_keybinding!( modkey: MODKEY_SHIFT, callback: |state| { state.send_active_window_to_workspace(3); }, key: keysym::XK_4 );
        set_keybinding!( modkey: MODKEY_SHIFT, callback: |state| {state.close_active();}, key: keysym::XK_c );
        set_keybinding!( modkey: MODKEY_SHIFT, callback: |state| {state.active_floating_move(0, 40);}, key: keysym::XK_Down );
        set_keybinding!( modkey: MODKEY_SHIFT, callback: |state| {state.active_floating_move(0, -40);}, key: keysym::XK_Up );
        set_keybinding!( modkey: MODKEY_SHIFT, callback: |state| {state.active_floating_move(-40, 0);}, key: keysym::XK_Left );
        set_keybinding!( modkey: MODKEY_SHIFT, callback: |state| {state.active_floating_move(40, 0);}, key: keysym::XK_Right );
        set_keybinding!( modkey: MODKEY_SHIFT, callback: |state| {state.active_floating_resize(40, 40);}, key: keysym::XK_plus );
        set_keybinding!( modkey: MODKEY_SHIFT, callback: |state| {state.active_floating_resize(-40, -40);}, key: keysym::XK_minus );
        set_keybinding!( modkey: MODKEY_CTRL, callback: |state| {toggle_active_window_prop!(state, floating_windows);}, key: keysym::XK_space );
    }

    /* startup apps */
    {
        spawn_with_shell!("nitrogen", ["--restore"]);
        spawn_with_shell!("picom",  ["--opacity-rule", "100:x=0", "--fade-exclude", "x=0"]);
        //spawn_with_shell!("./target/debug/xwidgetrefresher");
    }

    /* default workspaces config */
    {
        set_spaces!(state, ["一", "二", "三", "四"]);
        //set_spaces!(state, ["1", "2", "3", "4"]);
        let screen_width = unsafe{xlib::XDisplayWidth(state.dpy, state.screen) as u32};

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
    pub fullscreen_windows: HashSet<xlib::Window>,
    pub floating_windows: HashSet<xlib::Window>,
    pub rightclick_grab_origin: (i32, i32),
    pub rightclick_grab_window: xlib::Window,
    pub rightclick_grabbing: bool,
    pub leftclick_d: (i32, i32),
    pub leftclick_grab_window: xlib::Window,
    pub leftclick_grabbing: bool
}

macro_rules! is_floating {
    ($state: expr, $window: expr) => {
        active_workspace!($state).custom.as_ref().unwrap().floating_windows.contains($window)
    };
}

macro_rules! is_fullscreen {
    ($state: expr, $window: expr) => {
        active_workspace!($state).custom.as_ref().unwrap().fullscreen_windows.contains($window)
    };
}

macro_rules! custom {
    ($state: expr) => {
        active_workspace!($state).custom.as_mut().unwrap()
    };
}

impl state::State<'_> {
    pub fn retile(&mut self){
        self.draw_widgets(Ctx::Retile);

        /* configurable grouping logic */
        let mut tiled_windows: Vec<xlib::Window> = Vec::new();
        let mut fullscreen_windows: Vec<xlib::Window> = Vec::new();
        let mut floating_windows: Vec<xlib::Window> = Vec::new();

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
    
    fn draw_fullscreen_windows(&mut self, windows: &Vec<xlib::Window>){
        let screen_width: u32 = unsafe{xlib::XDisplayWidth(self.dpy, self.screen) as u32};
        let screen_height = unsafe{xlib::XDisplayHeight(self.dpy, self.screen) as u32};
    
        for window in windows {
            unsafe { xlib::XRaiseWindow(self.dpy, *window) };       
            window.do_map(self, (0, 0, screen_width, screen_height));
        }
    }

    fn draw_floating_windows(&mut self, windows: &Vec<xlib::Window>){
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

    fn leftclick_release(&mut self, _: (i32, i32)){
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

    fn rightclick_release(&mut self, _: (i32, i32)){
        if let Some(custom) = &mut active_workspace!(self).custom {
            custom.rightclick_grab_origin = (0,0);
            custom.rightclick_grabbing = false;
            self.active.focus_locked = false;
        }
    }
    
    fn mouse_move(&mut self, pt: (i32, i32)){
        if active_workspace!(self).custom.is_none() { return; }
        if custom!(self).rightclick_grabbing { self.mouse_move_rightclick(pt) };
        if custom!(self).leftclick_grabbing { self.mouse_move_leftclick(pt) };
    }

    fn mouse_move_rightclick(&mut self, (x, y): (i32, i32)) {
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

    fn mouse_move_leftclick(&mut self, (x, y): (i32, i32)) {
        if !is_floating!(self, &active_workspace!(self).custom.as_ref().unwrap().leftclick_grab_window) { return; }
        let rect: (i32, i32, u32, u32) = custom!(self).leftclick_grab_window.get_rect(self);
        let x_new = x - custom!(self).leftclick_d.0;
        let y_new = y - custom!(self).leftclick_d.1;

        if (x_new - rect.0).abs() > 50
        || (y_new - rect.1).abs() > 50 {
            custom!(self).leftclick_grab_window.do_map(self, (x_new, y_new, rect.2, rect.3));
        }
    }

    fn mouse_cross(&mut self, window: xlib::Window) {
        if active_workspace!(self).custom.is_none() { return };
        if !custom!(self).leftclick_grabbing { return };
        if is_floating!(self, &active_workspace!(self).custom.as_ref().unwrap().leftclick_grab_window) { return };
        let leftclick_grab_window = custom!(self).leftclick_grab_window;
        let maybe_ix1: Option<usize> = active_workspace_wins!(self).iter().position(|x| *x == window);
        let maybe_ix2: Option<usize> = active_workspace_wins!(self).iter().position(|x| *x == leftclick_grab_window);
        if maybe_ix1.is_none() { return }
        if maybe_ix2.is_none() { return }
        if is_floating!(self, &window) || is_fullscreen!(self, &window) { return }
        active_workspace_wins!(self).swap(maybe_ix1.unwrap(), maybe_ix2.unwrap());
        self.retile();
    }
}

