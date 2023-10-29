use image::DynamicImage;
use rand::Rng;
use std::collections::HashMap;
use world::{
    image_gradient,
    models::{
        color::Color8,
        continent::{Continent, Region},
        point::PointU16,
    },
};

pub fn build_regions_image(img_size: &PointU16, regions: &Vec<Region>, image_name: &str) {
    let mut imgbuf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        image::ImageBuffer::new(img_size.x as u32, img_size.y as u32);

    for r in regions {
        let mut rng = rand::thread_rng();
        let color = Color8 {
            r: rng.gen_range(0..=255),
            g: rng.gen_range(0..=255),
            b: rng.gen_range(0..=255),
        };

        for px in &r.pixels {
            let pixel = imgbuf.get_pixel_mut(px.0 as u32, px.1 as u32);
            *pixel = image::Rgb([color.r, color.g, color.b]);
        }
    }

    // create the actual image
    imgbuf.save(image_name).unwrap();
}

pub fn build(img_size: PointU16, continents: &HashMap<(u16, u16), Continent>, image_name: &str) {
    let mut imgbuf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        image::ImageBuffer::new(img_size.x as u32, img_size.y as u32);

    // load gradient images
    let mut gradient_images: HashMap<u8, DynamicImage> = HashMap::new();
    for continent_point in continents.values() {
        let exists = gradient_images.contains_key(&continent_point.plate_movement_direction);
        if exists == true {
            let degrees =
                image_gradient::get_degrees_by_index(continent_point.plate_movement_direction);
            let gradient_image: DynamicImage = image_gradient::load_gradient(degrees);
            gradient_images.insert(continent_point.plate_movement_direction, gradient_image);
        }
    }

    for cp in continents {
        let mut rng = rand::thread_rng();
        let color = Color8 {
            r: rng.gen_range(0..=255),
            g: rng.gen_range(0..=255),
            b: rng.gen_range(0..=255),
        };

        if let Some(pixels) = &cp.1.pixels {
            for px in pixels {
                let pixel = imgbuf.get_pixel_mut(px.0 as u32, px.1 as u32);
                *pixel = image::Rgb([color.r, color.g, color.b]);
            }
        }
    }

    // create the actual image
    imgbuf.save(image_name).unwrap();
}
