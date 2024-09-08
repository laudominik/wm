use x11::xlib::{self, Window};

use crate::{style::{ColorSchemesXft}, wm};

pub type Cursor = Cursor_<xlib::Cursor>;

pub struct Cursor_<T> {
    pub normal: T,
    pub resize: T,
    pub mov: T
}

pub struct State<'a> {
    pub screen: i32,
    pub root: xlib::Window,
    pub cursor: Cursor,
    pub dpy: &'a mut xlib::Display,
    pub workspaces: Vec<wm::Space<'a>>,
    pub colors : ColorSchemesXft,
    pub keybindings: Vec<Keybinding>,
    pub active: Active
}

pub struct Active {
    pub workspace: usize,
    pub window: Window
}

pub struct Keybinding {
    pub mdky: u32,
    pub key: u32,
    pub callback: Box<dyn Fn()>
}
