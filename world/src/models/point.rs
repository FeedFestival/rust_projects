pub struct Size16 {
    pub width: u16,
    pub height: u16,
}

impl Size16 {
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
    pub fn substract(this: &Point16, other: &Point16) -> Point16 {
        Point16 {
            x: this.x - other.x,
            y: this.y - other.y
        }
    }
    pub fn multiply(this: &Point16, other: &Point16) -> Point16 {
        Point16 {
            x: this.x * other.x,
            y: this.y * other.y
        }
    }
    pub fn multiply_by_size(this: &Point16, other: &Size16) -> Point16 {
        Point16 {
            x: this.x * other.width,
            y: this.y * other.height
        }
    }
}

pub fn try_map_min_max_points(
    bottom_left_x: &mut u16, bottom_left_y: &mut u16, top_right_x: &mut u16, top_right_y: &mut u16,
    x: u16, y: u16
) {
    if x < *bottom_left_x {
        *bottom_left_x = x;
    }
    if x > *top_right_x {
        *top_right_x = x;
    }
    if y < *bottom_left_y {
        *bottom_left_y = y;
    }
    if y > *top_right_y {
        *top_right_y = y;
    }
}

pub fn try_map_min_max_points_by_points(
    bottom_left_x: &mut u16, bottom_left_y: &mut u16, top_right_x: &mut u16, top_right_y: &mut u16,
    rg_bottom_left: &Point16, rg_top_right: &Point16
) {
    if rg_bottom_left.x < *bottom_left_x {
        *bottom_left_x = rg_bottom_left.x;
    }
    if rg_top_right.x > *top_right_x {
        *top_right_x = rg_top_right.x;
    }
    if rg_bottom_left.y < *bottom_left_y {
        *bottom_left_y = rg_bottom_left.y;
    }
    if rg_top_right.y > *top_right_y {
        *top_right_y = rg_top_right.y;
    }
}

pub fn try_map_points_min_max_points_by_points(
    pv_bottom_left: &mut Point16, pv_top_right: &mut Point16,
    rg_bottom_left: &Point16, rg_top_right: &Point16
) {
    if rg_bottom_left.x < pv_bottom_left.x {
        pv_bottom_left.x = rg_bottom_left.x;
    }
    if rg_top_right.x > pv_top_right.x {
        pv_top_right.x = rg_top_right.x;
    }
    if rg_bottom_left.y < pv_bottom_left.y {
        pv_bottom_left.y = rg_bottom_left.y;
    }
    if rg_top_right.y > pv_top_right.y {
        pv_top_right.y = rg_top_right.y;
    }
}
