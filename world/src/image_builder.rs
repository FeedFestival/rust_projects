use gamescript::models::{
    color::Color8,
    continent::{Planet, Province, Realm, Region},
    point::Size16,
};
use image::{Rgb, Luma, ImageBuffer};
use rand::Rng;

pub fn build_regions_image(img_size: &Size16, regions: &Vec<Region>, image_path: &str) {
    let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::new(img_size.width as u32, img_size.height as u32);

    for rg in regions {
        let mut rng = rand::thread_rng();
        let color = Color8 {
            r: rng.gen_range(0..=255),
            g: rng.gen_range(0..=255),
            b: rng.gen_range(0..=255),
        };

        for px in &rg.pixels {
            let pixel = imgbuf.get_pixel_mut(px.0 as u32, px.1 as u32);
            *pixel = Rgb([color.r, color.g, color.b]);
        }
    }

    // create the actual image
    imgbuf.save(image_path).unwrap();
}

pub fn build_provinces_image(img_size: &Size16, provinces: &Vec<Province>, image_name: &str) {
    let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::new(img_size.width as u32, img_size.height as u32);

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
                *pixel = Rgb([color.r, color.g, color.b]);
            }
        }
    }

    // create the actual image
    imgbuf.save(image_name).unwrap();
}

pub fn build_realms_image(img_size: &Size16, realms: &Vec<Realm>, image_name: &str) {
    let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::new(img_size.width as u32, img_size.height as u32);

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
                    *pixel = Rgb([color.r, color.g, color.b]);
                }
            }
        }
    }

    // create the actual image
    imgbuf.save(image_name).unwrap();
}

pub fn build_planet_image(planet: &Planet, image_name: &str) {
    let mut imgbuf = ImageBuffer::new(planet.img_size.width as u32, planet.img_size.height as u32);
    for x in 0..planet.grid_size.width {
        for y in 0..planet.grid_size.height {
            let continent = planet.continents.get(&(x, y)).unwrap();
            for rlm in &continent.realms {
                for pv in &rlm.provinces {
                    for rg in &pv.regions {
                        for px in &rg.pixels {
                            let pixel = imgbuf.get_pixel_mut(px.0 as u32, px.1 as u32);
                            *pixel = Luma([rg.grey_value]);
                        }
                    }
                }
            }
        }
    }

    // create the actual image
    imgbuf.save(image_name).unwrap();
}
