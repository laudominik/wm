use std::collections::HashSet;
use std::f32::INFINITY;
use std::process::Command;
use std::sync::Arc;
use std::mem;

use x11::keysym::{XK_Down, XK_Left, XK_Return, XK_Right, XK_Up, XK_c, XK_downarrow, XK_f, XK_h, XK_j, XK_k, XK_l, XK_minus, XK_plus, XK_r, XK_space, XK_uparrow, XK_w};
use x11::xlib::{ControlMask, Mod1Mask, Mod3Mask, Mod4Mask, ShiftMask, Window, XDisplayHeight, XDisplayWidth, XGetWindowAttributes, XWindowAttributes};

use crate::state::{self, Keybinding, State, KEYBINDINGS};
use crate::style::{ColorScheme, ColorSchemes, Style};
use crate::wm::WindowExt;
use crate::{active_workspace, active_workspace_wins, set_keybinding, set_spaces, spawn_with_shell, wm};

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
    border_thickness: 3,
    useless_gap: 5
};

const MODKEY: u32 = Mod4Mask;
const MODKEY_SHIFT: u32 = MODKEY | ShiftMask;
const MODKEY_CTRL: u32 = MODKEY | ControlMask;

/* your private config goes here */
pub fn make(state: &mut state::State){

    /* keybindings */
    {
        set_keybinding!(
            modkey: MODKEY,
            callback: |_| {spawn_with_shell!("alacritty");}, 
            key: XK_Return
        );

        set_keybinding!(
            modkey: MODKEY,
            callback: |_| {spawn_with_shell!("dmenu_run");},
            key: XK_r
        );

        set_keybinding!(
            modkey: MODKEY,
            callback: |state| {state.focus_next();},
            key: XK_j
        );

        set_keybinding!(
            modkey: MODKEY,
            callback: |state| {state.focus_previous();},
            key: XK_k
        );

        set_keybinding!(
            modkey: MODKEY,
            callback: |state| {toggle_active_window_prop!(state, fullscreen_windows);},
            key: XK_f
        );

        set_keybinding!(
            modkey: MODKEY,
            callback: |state| {state.separator_modify(40)},
            key: XK_l
        );

        set_keybinding!(
            modkey: MODKEY,
            callback: |state| {state.separator_modify(-40)},
            key: XK_h
        );
    
        set_keybinding!(
            modkey: MODKEY_SHIFT,
            callback: |state| {state.close_active();},
            key: XK_c
        );

        set_keybinding!(
            modkey: MODKEY_SHIFT,
            callback: |state| {state.active_floating_move(0, 40);},
            key: XK_Down
        );

        set_keybinding!(
            modkey: MODKEY_SHIFT,
            callback: |state| {state.active_floating_move(0, -40);},
            key: XK_Up
        );

        set_keybinding!(
            modkey: MODKEY_SHIFT,
            callback: |state| {state.active_floating_move(-40, 0);},
            key: XK_Left
        );

        set_keybinding!(
            modkey: MODKEY_SHIFT,
            callback: |state| {state.active_floating_move(40, 0);},
            key: XK_Right
        );

        set_keybinding!(
            modkey: MODKEY_SHIFT,
            callback: |state| {state.active_floating_resize(40, 40);},
            key: XK_plus
        );
        
        set_keybinding!(
            modkey: MODKEY_SHIFT,
            callback: |state| {state.active_floating_resize(-40, -40);},
            key: XK_minus
        );

        set_keybinding!(
            modkey: MODKEY_CTRL,
            callback: |state| {toggle_active_window_prop!(state, floating_windows);},
            key: XK_space
        );
   
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
                floating_windows: HashSet::new()
            });
        }
    }
}

pub struct CustomData {
    pub separator: u32 /* used by cascade_autotiling */,
    pub fullscreen_windows: HashSet<Window>,
    pub floating_windows: HashSet<Window>
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
                } else if active_workspace!(self).custom.as_ref().unwrap().floating_windows.contains(window) {
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
        self.draw_floating_windows(floating_windows);
        self.draw_fullscreen_windows(fullscreen_windows);
    }

    fn separator_modify(&mut self, modifier: i32) {
        if let Some(custom ) = &mut active_workspace!(self).custom {
            custom.separator = (custom.separator as i32 + modifier).clamp(100, 1760) as u32;
            self.retile();
        }
    }
    
    fn draw_fullscreen_windows(&mut self, windows: Vec<Window>){
        let screen_width: u32 = unsafe{XDisplayWidth(self.dpy, self.screen) as u32};
        let screen_height = unsafe{XDisplayHeight(self.dpy, self.screen) as u32};
    
        for window in windows {
            window.do_map(self, (0, 0, screen_width, screen_height));
        }
    }

    fn draw_floating_windows(&mut self, windows: Vec<Window>){
        for window in windows {
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
}

