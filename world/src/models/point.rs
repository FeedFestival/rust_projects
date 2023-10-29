pub struct PointU16 {
    pub x: u16,
    pub y: u16,
}

impl PointU16 {
    pub fn default() -> PointU16 {
        PointU16 { x: 0, y: 0 }
    }
    pub fn new(x: u16, y: u16) -> PointU16 {
        PointU16 { x, y }
    }
}