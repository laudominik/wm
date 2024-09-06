use x11::xlib;

pub type WmAtoms = WmAtoms_<xlib::Atom>; 
pub type NetAtoms = NetAtoms_<xlib::Atom>;

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

pub struct State {
    pub wmatom: WmAtoms,
    pub netatom: NetAtoms
}
