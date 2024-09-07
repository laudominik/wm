use crate::state;

pub fn add_window(state: &mut state::State){
    let active_workspace: &mut state::Wspace<'_> = &mut state.workspaces[state.active_workspace];
    active_workspace.windows.push(
        state::Wwin {
            coords: (0, 0),
            size: (0, 0)
        }
    );   


    println!("window added")

    // TODO: window management logic
}