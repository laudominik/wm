use std::process::Command;
use std::sync::Arc;

use x11::keysym::{XK_Return, XK_space};
use x11::xlib::{Mod1Mask, Mod3Mask, Mod4Mask, ShiftMask};

use crate::state::{self, Keybinding, KEYBINDINGS};
use crate::style::{ColorScheme, ColorSchemes, Style};
use crate::wm;

macro_rules! set_spaces {
    ($state:expr, [ $($tag:expr),* ]) => {{
        $(    
            $state.workspaces.push(wm::Space {
                tag: $tag,
                windows: Vec::new()
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


/* your private config goes here */
pub fn make(state: &mut state::State){
    set_spaces!(state, ["一", "二", "三", "四"]);

    set_keybinding!(
        modkey: MODKEY,
        callback: |_| {println!("test: pressed")}, 
        key: XK_space
    );

    set_keybinding!(
        modkey: MODKEY,
        callback: |_| {spawn_with_shell!("alacritty");}, 
        key: XK_Return
    );

    spawn_with_shell!("nitrogen", ["--restore"]);
    spawn_with_shell!("picom");
}

impl state::State<'_> {
    pub fn retile(&mut self){
        /* configurable tiling logic */
        self.cascade_autotiling();
    }
}
