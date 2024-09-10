use std::process::Command;
use std::sync::Arc;

use x11::keysym::{XK_Return, XK_c, XK_h, XK_j, XK_k, XK_l, XK_space, XK_r};
use x11::xlib::{ControlMask, Mod1Mask, Mod3Mask, Mod4Mask, ShiftMask, XDisplayHeight, XDisplayWidth};

use crate::state::{self, Keybinding, KEYBINDINGS};
use crate::style::{ColorScheme, ColorSchemes, Style};
use crate::{active_workspace, active_workspace_wins, wm};

macro_rules! set_spaces {
    ($state:expr, [ $($tag:expr),* ]) => {{
        $(    
            $state.workspaces.push(wm::Space {
                tag: $tag,
                windows: Vec::new(),
                custom: None
            });
        )*
    }};
}


macro_rules! set_keybinding {
    (modkey: $mdky: expr, callback: $cb:expr, key: $key:expr) => {
        {
            unsafe {
                KEYBINDINGS.push(Keybinding {
                    mdky: $mdky,
                    key: $key, 
                    callback: Arc::new($cb)
                });
            }
        }
    }
}

macro_rules! spawn_with_shell {
    ($command:expr, [ $($arg:expr),* ]) => {{
            Command::new($command)
            .env("DISPLAY", ":1")
            $(    
                .arg($arg)
            )*.spawn().expect("Failed to execute command")
    }};

    ($command:expr) => {
        {
            Command::new($command)
            .env("DISPLAY", ":1")
            .spawn().expect("Failed to execute command");
        }
    }
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
            callback: |state| {separator_modify(state, 10)},
            key: XK_l
        );

        set_keybinding!(
            modkey: MODKEY,
            callback: |state| {separator_modify(state, -10)},
            key: XK_h
        );
    
        set_keybinding!(
            modkey: MODKEY_SHIFT,
            callback: |state| {state.close_active();},
            key: XK_c
        );

        set_keybinding!(
            modkey: MODKEY_CTRL,
            callback: |state| {/* make active window floating */},
            key: XK_space
        )
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
                separator: screen_width/2
            });
        }
    }
}

pub struct CustomData {
    pub separator: u32 /* used by cascade_autotiling */
}

impl state::State<'_> {
    pub fn retile(&mut self){
        /* configurable tiling logic */
        self.cascade_autotiling();
    }
}

fn separator_modify(state: &mut state::State, modifier: i32) {
    if let Some(custom ) = &mut active_workspace!(state).custom {
        custom.separator = (custom.separator as i32 + modifier).clamp(10, 1760) as u32;
        state.retile();
    }
}
