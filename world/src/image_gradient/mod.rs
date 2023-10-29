use rand::Rng;
use image::{DynamicImage};
extern crate image;
use std::path::Path;

pub fn get_degrees_by_index(index: u8) -> u16 {
    match index {
        0 => return 0,
        1 => return 22,
        2 => return 45,
        3 => return 67,
        4 => return 90,
        5 => return 112,
        6 => return 135,
        7 => return 157,
        8 => return 180,
        9 => return 202,
        10 => return 225,
        11 => return 247,
        12 => return 270,
        13 => return 292,
        14 => return 315,
        15 => return 337,
        16 => return 337,
        _ => return 337,
    }
}

pub fn get_random_degrees_index() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..16)
}

pub fn load_gradient(nr: u16) -> DynamicImage {
    // let image_path = "C:/__WORK__/Projects/rust-projects/world/src/image_gradient/0.jpg";
    let image_path = format!("src/image_gradient/{}.jpg", nr);
    let path = Path::new(&image_path);
    // println!("{}", &path.display());
    let img: DynamicImage = image::open(&path).unwrap();

    // let (width, height) = img.dimensions();
    // for x in 0..width {
    //     for y in 0..height {
    //         // Get the color of the pixel at coordinates (x, y)
    //         let pixel = img.get_pixel(x, y);
    //         let red = pixel[0];
    //         let green = pixel[1];
    //         let blue = pixel[2];
    //         println!("Pixel at ({}, {}): R={}, G={}, B={}", x, y, red, green, blue);
    //     }
    // }

    img
}