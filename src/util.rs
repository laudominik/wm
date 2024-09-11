#[macro_export]
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

#[macro_export]
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

#[macro_export]
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

#[macro_export]
macro_rules! set_mousemotion {
    (modkey: $mdky: expr, callback: $cb:expr, mousebutton: $msby:expr, onpress) => {
        {
            unsafe {
                MOUSEMOTIONS.on_press.push(Mousemotion {
                    mdky: $mdky, 
                    callback: Arc::new($cb),
                    button: $msby
                });
            }
        }
    };

    (modkey: $mdky: expr, callback: $cb:expr, mousebutton: $msby:expr, onrelease) => {
        {
            unsafe {
                MOUSEMOTIONS.on_release.push(Mousemotion {
                    mdky: $mdky, 
                    callback: Arc::new($cb),
                    button: $msby
                });
            }
        }
    };

    (modkey: $mdky: expr, callback: $cb:expr, onmove) => {
        {
            unsafe {
                MOUSEMOTIONS.on_move.push(Mousemotion {
                    mdky: $mdky, 
                    callback: Arc::new($cb),
                    button: 0
                });
            }
        }
    };
}