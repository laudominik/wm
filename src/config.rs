use crate::state::{self};
use crate::style::{ColorScheme, ColorSchemes, Style};
use crate::wm;


pub static STYLE: Style = Style {
    colors: ColorSchemes {
       normal:  ColorScheme {
            fg: "#222",
            bg: "#222",
            border: "#222"
       },
       selected: ColorScheme {
            fg: "#222",
            bg: "#222",
            border: "#222"
       }
    },
    border_thickness: 5
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

/* your private config goes here */
pub fn make(state: &mut state::State){
    set_spaces!(state, ["一", "二", "三", "四"]);
}
