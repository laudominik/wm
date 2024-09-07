use crate::state::{self};
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

/* your private config goes here */
pub fn make(state: &mut state::State){
    set_spaces!(state, ["一", "二", "三", "四"]);
}
