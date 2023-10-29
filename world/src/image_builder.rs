use image::{DynamicImage, GenericImageView, GrayImage};
use prisma::{FromColor, FromTuple, Hsv, Rgb};
use rand::Rng;
use std::collections::HashMap;
use world::{
    image_gradient,
    models::{
        color::Color8,
        continent::{Continent, Province, Realm, Region},
        point::{try_map_min_max_points, try_map_min_max_points_by_points, Point16, Size16},
    },
};

pub fn build_regions_image(img_size: &Size16, regions: &Vec<Region>, image_name: &str) {
    let mut imgbuf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        image::ImageBuffer::new(img_size.width as u32, img_size.height as u32);

    for rg in regions {
        let mut rng = rand::thread_rng();
        let color = Color8 {
            r: rng.gen_range(0..=255),
            g: rng.gen_range(0..=255),
            b: rng.gen_range(0..=255),
        };

        for px in &rg.pixels {
            let pixel = imgbuf.get_pixel_mut(px.0 as u32, px.1 as u32);
            *pixel = image::Rgb([color.r, color.g, color.b]);
        }
    }

    // create the actual image
    imgbuf.save(image_name).unwrap();
}

pub fn build_provinces_image(img_size: &Size16, provinces: &Vec<Province>, image_name: &str) {
    let mut imgbuf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        image::ImageBuffer::new(img_size.width as u32, img_size.height as u32);

    for pv in provinces {
        let mut rng = rand::thread_rng();
        let color = Color8 {
            r: rng.gen_range(0..=255),
            g: rng.gen_range(0..=255),
            b: rng.gen_range(0..=255),
        };

        for rg in &pv.regions {
            for px in &rg.pixels {
                let pixel = imgbuf.get_pixel_mut(px.0 as u32, px.1 as u32);
                *pixel = image::Rgb([color.r, color.g, color.b]);
            }
        }
    }

    // create the actual image
    imgbuf.save(image_name).unwrap();
}

pub fn build_realms_image(img_size: &Size16, realms: &Vec<Realm>, image_name: &str) {
    let mut imgbuf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        image::ImageBuffer::new(img_size.width as u32, img_size.height as u32);

    for rlm in realms {
        let mut rng = rand::thread_rng();
        let color = Color8 {
            r: rng.gen_range(0..=255),
            g: rng.gen_range(0..=255),
            b: rng.gen_range(0..=255),
        };

        for pv in &rlm.provinces {
            for rg in &pv.regions {
                for px in &rg.pixels {
                    let pixel = imgbuf.get_pixel_mut(px.0 as u32, px.1 as u32);
                    *pixel = image::Rgb([color.r, color.g, color.b]);
                }
            }
        }
    }

    // create the actual image
    imgbuf.save(image_name).unwrap();
}

pub fn build(img_size: Size16, continents: &HashMap<(u16, u16), Continent>, image_name: &str) {
    let mut imgbuf: image::ImageBuffer<image::Luma<u8>, Vec<u8>> =
        image::ImageBuffer::new(img_size.width as u32, img_size.height as u32);

    // load gradient images
    let mut gradient_images: HashMap<u8, GrayImage> = HashMap::new();
    for continent in continents.values() {
        let exists = gradient_images.contains_key(&continent.plate_movement_direction);
        if exists == false {
            let degrees = image_gradient::get_degrees_by_index(continent.plate_movement_direction);
            let gradient_image: GrayImage = image_gradient::load_gradient(degrees);
            gradient_images.insert(continent.plate_movement_direction, gradient_image);
        }
    }

    for cp in continents {
        let square_size = Size16::new(
            cp.1.top_right.x - cp.1.bottom_left.x,
            cp.1.top_right.y - cp.1.bottom_left.y,
        );

        if let Some(gradient_texture) = gradient_images.get(&cp.1.plate_movement_direction) {
            let per_pixel_size = (
                (gradient_texture.width() as f64 / square_size.width as f64) as f64,
                (gradient_texture.height() as f64 / square_size.height as f64) as f64,
            );

            for rlm in &cp.1.realms {
                for pv in &rlm.provinces {
                    for rg in &pv.regions {
                        let gradient_pos = Point16::substract(&rg.site_point, &cp.1.bottom_left);
                        let pixel_pos = calculate_pixel_pos(&gradient_pos, &per_pixel_size);
                        let pixel_color =
                            gradient_texture.get_pixel(pixel_pos.x as u32, pixel_pos.y as u32);

                        let mut color_value = pixel_color.0[0];
                        let new_value =
                            normalize(color_value as f64, 255.0) * cp.1.elevation as f64;
                        color_value = denormalize(new_value);

                        for px in &rg.pixels {
                            let pixel = imgbuf.get_pixel_mut(px.0 as u32, px.1 as u32);
                            *pixel = image::Luma([color_value]);
                        }
                    }
                }
            }
        }
    }

    // create the actual image
    imgbuf.save(image_name).unwrap();
}

fn calculate_pixel_pos(gradient_pos: &Point16, per_pixel_size: &(f64, f64)) -> Point16 {
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

fn normalize(value: f64, max: f64) -> f64 {
    if max == 0.0 {
        return 0.0; // Avoid division by zero
    }
    value / max
}

fn denormalize(normalized_value: f64) -> u8 {
    (normalized_value * 255.0) as u8
}
