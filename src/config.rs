use crate::state::{self, Keybinding};
use crate::style::{ColorScheme, ColorSchemes, Style};
use crate::wm;


pub static STYLE: Style = Style {
    colors: ColorSchemes {
       normal:  ColorScheme {
            fg: "#222",
            bg: "#222",
            border: "#ff0000"
       },
       selected: ColorScheme {
            fg: "#222",
            bg: "#222",
            border: "#222"
       }
    },
    border_thickness: 5,
    useless_gap: 5
};

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
    ($state:expr, callback: $cb:expr, keys: [$($key:expr), *]) => {
        {
            $state.keybindings.push(Keybinding {});
        }
    }
}

/* your private config goes here */
pub fn make(state: &mut state::State){
    set_spaces!(state, ["一", "二", "三", "四"]);
    set_keybinding!(
        state,
        callback: || {}, 
        keys:[Mod1Mask, XK_p]
    )
}

impl state::State<'_> {
    pub fn retile(&mut self){
        self.cascade_autotiling();
    }
}
