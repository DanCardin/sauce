use ansi_term::Colour;
use ansi_term::Colour::RGB;
use comfy_table::Color;

pub const BLUE: Colour = RGB(70, 130, 180);
pub const YELLOW: Colour = RGB(250, 189, 47);
pub const RED: Colour = RGB(251, 73, 52);

pub const TABLE_BLUE: Color = Color::Rgb {
    r: 70,
    g: 130,
    b: 180,
};
pub const TABLE_YELLOW: Color = Color::Rgb {
    r: 250,
    g: 180,
    b: 47,
};
