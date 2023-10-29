use image::{DynamicImage, GenericImageView };
use prisma::{Hsv, FromTuple, Rgb, FromColor};
use rand::Rng;
use std::collections::HashMap;
use world::{
    image_gradient,
    models::{
        color::Color8,
        continent::{Continent, Province, Realm, Region},
        point::{Point16, Size16},
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
    let mut imgbuf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        image::ImageBuffer::new(img_size.width as u32, img_size.height as u32);

    // load gradient images
    let mut gradient_images: HashMap<u8, DynamicImage> = HashMap::new();
    for continent in continents.values() {
        // println!("continent ({}, {}).plate_movement_direction: {}", continent.grid_coord.x, continent.grid_coord.y, continent.plate_movement_direction);
        let exists = gradient_images.contains_key(&continent.plate_movement_direction);
        if exists == false {
            let degrees =
                image_gradient::get_degrees_by_index(continent.plate_movement_direction);
            let gradient_image: DynamicImage = image_gradient::load_gradient(degrees);
            gradient_images.insert(continent.plate_movement_direction, gradient_image);
        }
    }

    for cp in continents {
        // figure out gradient square pixel coords
        let square_coord = get_gradient_square_pixel_coords(&cp.1);
        let bottom_left = square_coord.0;
        // println!("bottom_left: ({}, {})", bottom_left.x, bottom_left.y);
        let top_right = square_coord.1;
        // println!("top_right: ({}, {})", top_right.x, top_right.y);
        let square_size = Size16::new(top_right.x - bottom_left.x, top_right.y - bottom_left.y);

        // println!("cp.1.plate_movement_direction: {}", cp.1.plate_movement_direction);

        if let Some(gradient_texture) = gradient_images.get(&cp.1.plate_movement_direction) {
            let per_pixel_size = (
                (gradient_texture.width() as f64 / square_size.width as f64) as f64,
                (gradient_texture.height() as f64 / square_size.height as f64) as f64,
            );

            // println!("gradient_texture: ({}, {})", gradient_texture.width(), gradient_texture.height());
            // println!("per_pixel_size: ({}, {})", per_pixel_size.0, per_pixel_size.0);

            for rlm in &cp.1.realms {
                for pv in &rlm.provinces {
                    for rg in &pv.regions {

                        // println!("rg.site_point: ({}, {}); bottom_left: ({}, {})", rg.site_point.x, rg.site_point.y, bottom_left.x, bottom_left.y);
                        let gradient_pos = Point16::substract(&rg.site_point, &bottom_left);
                        // println!("gradient_pos: ({}, {})", gradient_pos.x, gradient_pos.y);
                        let pixel_pos = calculate_pixel_pos(&gradient_pos, &per_pixel_size);
                        // println!("pixel_pos: ({}, {})", pixel_pos.x, pixel_pos.y);
                        let pixel_color: image::Rgba<u8> = gradient_texture.get_pixel(pixel_pos.x as u32, pixel_pos.y as u32);
                        // println!("pixel_color: Rgb({}, {}, {})", pixel_color.0[0], pixel_color.0[1], pixel_color.0[2]);
                        let mut hsv = to_hsv(pixel_color);
                        hsv.set_value(hsv.value() * cp.1.elevation as f64);
                        let color = Rgb::from_color(&hsv);
                        // println!("RGB: {}, {}, {}", color.red(), color.green(), color.blue());

                        for px in &rg.pixels {
                            let pixel = imgbuf.get_pixel_mut(px.0 as u32, px.1 as u32);
                            // *pixel = image::Rgb([color.r, color.g, color.b]);
                            *pixel = image::Rgb([
                                denormalize(color.red()),
                                denormalize(color.green()),
                                denormalize(color.blue())
                            ]);
                        }

                        // println!("----");
                    }
                }
            }
        }
    }

    // create the actual image
    imgbuf.save(image_name).unwrap();
}

fn calculate_pixel_pos(gradient_pos: &Point16, per_pixel_size: &(f64, f64)) -> Point16 {
    let mut pixel_pos = Point16{
        x: (gradient_pos.x as f64 * per_pixel_size.0).floor() as u16,
        y: (gradient_pos.y as f64 * per_pixel_size.1).floor() as u16
    }; 
    if pixel_pos.x > 127 {
        pixel_pos.x = 127;
    }
    if pixel_pos.y > 127 {
        pixel_pos.y = 127;
    }
    pixel_pos
}

fn get_gradient_square_pixel_coords(continent: &Continent) -> (Point16, Point16) {
    let mut sx: u16 = u16::MAX;
    let mut sy: u16 = u16::MAX;
    let mut bx: u16 = u16::MIN;
    let mut by: u16 = u16::MIN;

    for rlm in &continent.realms {
        for pv in &rlm.provinces {
            for rg in &pv.regions {
                for px in &rg.pixels {
                    // TODO: can't we do this when we apply pixels to regions ??
                    // or when we apply realms to continents ??

                    let x = px.0;
                    if x < sx {
                        sx = x;
                    }
                    if x > bx {
                        bx = x;
                    }

                    let y = px.1;
                    if y < sy {
                        sy = y;
                    }
                    if y > by {
                        by = y;
                    }
                }
            }
        }
    }

    (Point16::new(sx, sy), Point16::new(bx, by))
}

fn to_hsv(pixel_color: image::Rgba<u8>) -> Hsv<f64> {
    let rgb1 = Rgb::new(
        normalize(pixel_color.0[0] as f64, 255 as f64),
        normalize(pixel_color.0[1] as f64, 255 as f64),
        normalize(pixel_color.0[2] as f64, 255 as f64)
    );
    Hsv::from_color(&rgb1)
}

fn normalize(value: f64, max: f64) -> f64 {
    if max == 0.0 {
        return 0.0; // Avoid division by zero
    }
    value / max
}

fn denormalize(normalized_value: f64) -> u8 {
    (normalized_value * 255.0) as u8
}
