use x11::xlib;

pub type WmAtoms = WmAtoms_<xlib::Atom>; 
pub type NetAtoms = NetAtoms_<xlib::Atom>;
pub type Cursor = Cursor_<xlib::Cursor>;

pub struct NetAtoms_<T> {
    pub active_window: T,
    pub supported: T,
    pub state: T,
    pub check: T,
    pub fullscreen: T,
    pub wtype: T
}

pub struct WmAtoms_<T> {
    pub protocols: T,
    pub delete: T,
    pub state: T,
    pub take_focus: T
}

pub struct Cursor_<T> {
    pub normal: T,
    pub resize: T,
    pub mov: T
}

pub struct State<'a> {
    pub screen: i32,
    pub root: xlib::Window,
    pub wmatom: WmAtoms,
    pub netatom: NetAtoms,
    pub cursor: Cursor,
    pub dpy: &'a mut xlib::Display,
    pub workspaces: Vec<Wspace<'a>>,
    pub active_workspace: usize
}

pub struct Wspace<'a> {
    pub tag: &'a str,
    pub windows: Vec<Wwin>
}

pub struct Wwin {
    pub coords: (i32, i32),
    pub size: (i32, i32)
}

pub struct Style {
    pub border_color: (i32, i32, i32, i32),
    pub border_thickness: i32

}