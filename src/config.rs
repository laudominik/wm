use crate::state;

macro_rules! set_workspaces {
    ($state:expr, [ $($tag:expr),* ]) => {{
        $(    
            $state.workspaces.push(state::Wspace {
                tag: $tag,
                windows: Vec::new()
            });
        )*
    }};
}

/* your private config goes here */
pub fn make(state: &mut state::State){
    set_workspaces!(state, ["一", "二", "三", "四"]);
}
