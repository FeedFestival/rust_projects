pub struct Size16 {
    pub width: u16,
    pub height: u16,
}

impl Size16 {
    // pub fn default() -> Size16 {
    //     PointU16 { x: 0, y: 0 }
    // }
    pub fn new(width: u16, height: u16) -> Size16 {
        Size16 { width, height }
    }
}

pub struct Point16 {
    pub x: u16,
    pub y: u16,
}

impl Point16 {
    pub fn default() -> Point16 {
        Point16 { x: 0, y: 0 }
    }
    pub fn new(x: u16, y: u16) -> Point16 {
        Point16 { x, y }
    }
}