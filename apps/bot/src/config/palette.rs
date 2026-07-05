/// A color.
pub struct Color {
    pub int: u32,
}

impl Color {
    pub const fn new(int: u32) -> Self {
        Self { int }
    }
}

/// The color palette.
pub struct Palette;

#[allow(dead_code)]
impl Palette {
    pub const PRIMARY: Color = Color::new(0x5496ff);
    pub const RED: Color = Color::new(0xff3838);
    pub const ORANGE: Color = Color::new(0xffa700);
    pub const GREEN: Color = Color::new(0x0fcc45);
}
