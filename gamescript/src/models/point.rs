use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Size16 {
    pub width: u16,
    pub height: u16,
}

impl Size16 {
    pub fn new(width: u16, height: u16) -> Size16 {
        Size16 { width, height }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
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

pub fn center_of_two_points(a: &Point16, b: &Point16) -> Point16 {
    return Point16 {
        // x: ((b.x - a.x) as f32 / 2.0) as u16 + a.x,
        // y: ((b.y - a.y) as f32 / 2.0) as u16 + a.y,
        x: (a.x + b.x) / 2,
        y: (a.y + b.y) / 2,
    };
}

pub fn calculate_distance(a: &Point16, b: &Point16) -> f32 {
    let x_diff = b.x as f32 - a.x as f32;
    let y_diff = b.y as f32 - a.y as f32;
    let distance = (x_diff.powi(2) + y_diff.powi(2)).sqrt();
    distance
}

pub fn calculate_pixel_pos(gradient_pos: &Point16, per_pixel_size: &(f64, f64)) -> Point16 {
    let mut pixel_pos = Point16 {
        x: (gradient_pos.x as f64 * per_pixel_size.0).floor() as u16,
        y: (gradient_pos.y as f64 * per_pixel_size.1).floor() as u16,
    };
    if pixel_pos.x > 127 {
        pixel_pos.x = 127;
    }
    if pixel_pos.y > 127 {
        pixel_pos.y = 127;
    }
    pixel_pos
}

// TODO: USAGE
// let mut hsv = to_hsv(pixel_color);
// hsv.set_value(hsv.value() * cp.1.elevation as f64);
// let color = Rgb::from_color(&hsv);
// ...
// *pixel = image::Rgb([
//     denormalize(color.red()),
//     denormalize(color.green()),
//     denormalize(color.blue())
// ]);

// fn to_hsv(pixel_color: image::Rgba<u8>) -> Hsv<f64> {
//     let rgb1 = Rgb::new(
//         normalize(pixel_color.0[0] as f64, 255 as f64),
//         normalize(pixel_color.0[1] as f64, 255 as f64),
//         normalize(pixel_color.0[2] as f64, 255 as f64)
//     );
//     Hsv::from_color(&rgb1)
// }

pub fn normalize_u8(value: f64) -> f64 {
    value / 255.0
}

pub fn denormalize_u8(normalized_value: f64) -> u8 {
    (normalized_value * 255.0) as u8
}
