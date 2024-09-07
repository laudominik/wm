use crate::state;

pub struct Space<'a> {
    pub tag: &'a str,
    pub windows: Vec<Tile>
}

pub struct Tile {
    pub coords: (i32, i32),
    pub size: (u32, u32)
}

pub fn add_window(state: &mut state::State){
    let active_workspace: &mut Space<'_> = &mut state.workspaces[state.active_workspace];

    active_workspace.windows.push(
        Tile {
            coords: (0, 0),
            size: (0, 0)
        }
    );   

    println!("window added")
}

impl Tile {
    pub fn new(state: &mut state::State, boundaries: (i32, i32)) -> Tile {
        return Tile {
            coords: (0, 0),
            size: (1000, 1000)
        }
    }
}

