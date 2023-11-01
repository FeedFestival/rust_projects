use std::{collections::HashMap, env, path::Path};
use std::thread;
use image::{imageops, DynamicImage, ImageBuffer, Luma};

pub struct Color8 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

enum SurfaceDepths {
    DeepSea = 35,   // 21
    ShallowSea = 50,    // 42
    Coast = 60, // 63

    Plains = 84,
    GentleHill = 105,
    SteepHill = 126,

    RuggedHill = 147,
    LowerMountain = 168,
    Plateaus = 189,

    HighMountain = 210,
    MountainPlateaus = 231,
    Peaks = 252,
}

const DEEP_SEA: Color8 = Color8 { r: 28, g: 101, b: 142, };   // 1C658E  = rgb(28, 101, 142)
const SHALLOW_SEA: Color8 = Color8 { r: 47, g: 137, b: 187 };    // 2F89BB = rgb(47, 137, 187)
const COAST: Color8 = Color8 { r: 142, g: 182, b: 134 };    // 8EB686 = rgb(142, 182, 134)

const PLAINS: Color8 = Color8 { r: 121, g: 162, b: 92 };   // 79A25C = rgb(121, 162, 92)
const GENTLE_HILL: Color8 = Color8 { r: 138, g: 170, b: 96 };  // 8AAA60 = rgb(138, 170, 96)
const STEEP_HILL: Color8 = Color8 { r: 167, g: 184, b: 101, };    // A7B865 = rgb(167, 184, 101)

const RUGGED_HILL: Color8 = Color8 { r: 184, g: 192, b: 105 };  // B8C069 = rgb(184, 192, 105)
const LOWER_MOUNTAIN: Color8 = Color8 { r: 239, g: 225, b: 123 };   // EFE17B = rgb(239, 225, 123)
const PLATEAUS: Color8 = Color8 { r: 223, g: 195, b: 113 };    // DFC371 = rgb(223, 195, 113)

const HIGH_MOUNTAIN: Color8 = Color8 { r: 202, g: 158, b: 100 };   // CA9E64 = rgb(202, 158, 100)
const MOUNTAIN_PLATEAUS: Color8 = Color8 { r: 166, g: 135, b: 113 };   // A68771 = rgb(166, 135, 113)
const PEAKS: Color8 = Color8 { r: 255, g: 255, b: 255 };   // FFFFFF

fn main() {
    if let Ok(project_dir) = env::current_dir() {
        
        let mut project_dir = project_dir.to_string_lossy().to_string();
        project_dir = project_dir.replace("tectonic-process", "world-tweak");
        let image_name = "final.png";
        let image_path = format!("{}/{}", project_dir, image_name);
        
        let path = Path::new(&image_path);
        let loaded_img: DynamicImage = image::open(&path).unwrap();
        let grey_img = loaded_img.to_luma8();

        let sharpened_img = imageops::unsharpen(&grey_img, 35.0, 5);
        sharpened_img.save("sharpened_image.png").unwrap();

        // let lighten_image = imageops::brighten(&sharpened_img, 4);
        // lighten_image.save("light_image.png");

        gradually_increase_light_pixels(&sharpened_img);

        color_oceans_and_land(&sharpened_img);

        let blurred_img = imageops::blur(&sharpened_img, 1.2);
        blurred_img.save("blurred_image.png").unwrap();
    }
}

fn gradually_increase_light_pixels(img: &ImageBuffer<Luma<u8>, Vec<u8>>) {
    // find very light pixels
    const LIGHT_PIXEL: u8 = 172;
    let mut light_pixels: HashMap<(u32, u32), u8> = HashMap::new();
    for (x, y, pixel) in img.enumerate_pixels() {
        let px_value = pixel.0[0];
        // println!("px_value: {}", px_value);
        if px_value > LIGHT_PIXEL {
            light_pixels.insert((x, y), px_value);
        }
    }

    println!("{}", light_pixels.len());

    let mut img_buf: ImageBuffer<Luma<u8>, Vec<u8>> =
        ImageBuffer::new(img.dimensions().0 as u32, img.dimensions().1 as u32);
    for x in 0..img.dimensions().0 {
        for y in 0..img.dimensions().1 {
            let pixel = img_buf.get_pixel_mut(x, y);

            if light_pixels.contains_key(&(x, y)) {
                let value = *light_pixels.get(&(x, y)).unwrap();
                // println!("value: {}", value);
                *pixel = image::Luma([value]);
            } else {
                *pixel = image::Luma([0]);
            }
        }
    }

    img_buf.save("light_pixels.png");
}

fn color_oceans_and_land(img: &ImageBuffer<Luma<u8>, Vec<u8>>) {
    let width = img.dimensions().0;
    let height = img.dimensions().1;
    let mut img_buf: ImageBuffer<image::Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let img_pixel = img.get_pixel(x, y);
            let grey_value = img_pixel.0[0];

            let mut color: Color8 = get_pixel_color_by_depth(grey_value);

            let pixel = img_buf.get_pixel_mut(x, y);
            *pixel = image::Rgb([color.r, color.g, color.b]);
        }
    }

    img_buf.save("earth_pixels.png");
}

fn get_pixel_color_by_depth(grey_value: u8) -> Color8 {

    if grey_value < SurfaceDepths::DeepSea as u8 {
        return DEEP_SEA;
    } else if grey_value < SurfaceDepths::ShallowSea as u8 {
        return SHALLOW_SEA;
    } else if grey_value < SurfaceDepths::Coast as u8 {
        return COAST;

    } else if grey_value < SurfaceDepths::Plains as u8 {
        return PLAINS;
    } else if grey_value < SurfaceDepths::GentleHill as u8 {
        return GENTLE_HILL;
    } else if grey_value < SurfaceDepths::SteepHill as u8 {
        return STEEP_HILL;

    } else if grey_value < SurfaceDepths::RuggedHill as u8 {
        return RUGGED_HILL;
    } else if grey_value < SurfaceDepths::LowerMountain as u8 {
        return LOWER_MOUNTAIN;
    } else if grey_value < SurfaceDepths::Plateaus as u8 {
        return PLATEAUS;

    } else if grey_value < SurfaceDepths::HighMountain as u8 {
        return HIGH_MOUNTAIN;
    } else if grey_value < SurfaceDepths::MountainPlateaus as u8 {
        return MOUNTAIN_PLATEAUS;
    }
    return PEAKS;
}
