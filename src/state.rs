use std::sync::Arc;

use x11::{xft::XftDraw, xlib::{self, Window}};

use crate::{style::ColorSchemesXft, widgets, wm};

pub type Cursor = Cursor_<xlib::Cursor>;

pub struct Cursor_<T> {
    pub normal: T,
    pub resize: T,
    pub mov: T
}

pub struct State<'a> {
    pub screen: i32,
    pub root: xlib::Window,
    pub draw: xlib::Window,
    pub xft_draw: *mut XftDraw,
    pub cursor: Cursor,
    pub dpy: &'a mut xlib::Display,
    pub workspaces: Vec<wm::Space<'a>>,
    pub colors : ColorSchemesXft,
    pub active: Active,
}

pub struct Active {
    pub workspace: usize,
    pub window: Window,
    pub focus_locked: bool,
}

pub static mut WIDGETS: Vec<Box<widgets::Widget>> = Vec::new();
pub static mut KEYBINDINGS : Vec<Keybinding> = Vec::new();

macro_rules! mousemotion_type_decl {
    ([$($name:ident),*]) => {
        pub struct MousemotionsType {
            $(
                pub $name: Vec<Mousemotion>,
            )*
        }
        pub static mut MOUSEMOTIONS: MousemotionsType = MousemotionsType{
            $(
                $name: Vec::new(),
            )*
        };
    };
} mousemotion_type_decl!([on_press, on_release, on_move, on_cross]);

pub struct Keybinding {
    pub mdky: u32,
    pub key: u32,
    pub callback: Arc<dyn Fn(&mut State) + Send + Sync>
}

pub struct Mousemotion {
    pub mdky: u32,
    pub button: u32,
    pub callback: Arc<dyn Fn(&mut State, (i32, i32), Window) + Send + Sync>
}
