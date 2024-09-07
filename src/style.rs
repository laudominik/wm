use std::ffi::CString;

use x11::{xft::{XftColor, XftColorAllocName}, xlib::{XDefaultColormap, XDefaultVisual}, xrender::XRenderColor};

use crate::state;


pub type ColorSchemes = ColorSchemes_<&'static str>;
pub type ColorScheme = ColorScheme_<&'static str>;
pub type ColorSchemesXft = ColorSchemes_<XftColor>;
pub type ColorSchemeXft = ColorScheme_<XftColor>;

pub struct Style {
    pub colors: ColorSchemes,
    pub border_thickness: u32,
    pub useless_gap: u32
}

pub struct ColorSchemes_<T> {
    pub normal: ColorScheme_<T>,
    pub selected: ColorScheme_<T>
}

pub struct ColorScheme_<T> {
    pub fg: T,
    pub bg: T,
    pub border: T
}

trait XftColorExt {
    fn from_str(state: &mut state::State, color: &str) ->XftColor;
}

impl XftColorExt for XftColor {
    fn from_str(state: &mut state::State, color: &str) -> XftColor {
        let mut x_color = XftColor {
            pixel: 0,
            color: XRenderColor { red: 0, green: 0, blue: 0, alpha: 0 }
        };
        let ccolor = CString::new(color).unwrap();
    
        unsafe {
            XftColorAllocName(
                state.dpy, 
                XDefaultVisual(state.dpy, state.screen),
                XDefaultColormap(state.dpy, state.screen),
                ccolor.as_ptr(), &mut x_color);
        }
        x_color
    }
}

impl ColorScheme_<&'static str> {
    fn to_xft(&self, state: &mut state::State) -> ColorSchemeXft {
        ColorScheme_ {
            fg: XftColor::from_str(state, self.fg),
            bg: XftColor::from_str(state, self.bg),
            border: XftColor::from_str(state, self.border)
        }
    }
}

impl ColorSchemes_<&'static str> {
    pub fn to_xft(&self, state: &mut state::State) -> ColorSchemes_<XftColor> {
        ColorSchemes_ {
            normal: self.normal.to_xft(state),
            selected: self.selected.to_xft(state),
        }
    }
}
