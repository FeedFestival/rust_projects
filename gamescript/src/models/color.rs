pub const GREY_VALUE: u8 = 255 / 2;
pub const GREY_COLOR: Color8 = Color8 {
    r: GREY_VALUE,
    g: GREY_VALUE,
    b: GREY_VALUE,
};
pub const WHITE_COLOR: Color8 = Color8 {
    r: 255,
    g: 255,
    b: 255,
};
pub struct Color8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl Color8 {
    pub fn new(r: u8, g: u8, b: u8) -> Color8 {
        return Color8 { r, g, b };
    }
}
pub fn new_color(r: u8, g: u8, b: u8) -> Color8 {
    return Color8 { r, g, b };
}
