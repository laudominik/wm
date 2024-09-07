use crate::state;

pub struct Style {
    pub colors: ColorSchemes,
    pub border_thickness: i32
}

pub struct ColorSchemes {
    pub normal: ColorScheme,
    pub selected: ColorScheme
}

pub struct ColorScheme {
    pub fg: &'static str,
    pub bg: &'static str,
    pub border: &'static str
}

pub static STYLE: Style = Style {
    colors: ColorSchemes {
       normal:  ColorScheme {
            fg: "#222",
            bg: "#222",
            border: "#222"
       },
       selected: ColorScheme {
            fg: "#222",
            bg: "#222",
            border: "#222"
       }
    },
    border_thickness: 5
};

