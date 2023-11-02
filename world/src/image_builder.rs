use std::collections::HashMap;

use gamescript::models::{
    color::Color8,
    continent::{Planet, Province, Realm, Region, PlanetSettings, Continent},
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

pub fn build_planet_image(planet: &Planet, planet_settings: &PlanetSettings, image_name: &str) {
    let mut imgbuf = ImageBuffer::new(planet.img_size.width as u32, planet.img_size.height as u32);
    for x in 0..planet_settings.continent_grid_size.width {
        for y in 0..planet_settings.continent_grid_size.height {
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

pub fn debug_planet_image(continents: &HashMap<(u16, u16), Continent>, planet_settings: &PlanetSettings, image_name: &str) {
    let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(
        planet_settings.img_size.width as u32,
        planet_settings.img_size.height as u32
    );
    for x in 0..planet_settings.continent_grid_size.width {
        for y in 0..planet_settings.continent_grid_size.height {

            let continent_opt = continents.get(&(x as u16, y));

            match continent_opt {
                Some(continent) => {
                    let mut rng = rand::thread_rng();
                    let mut color = Color8 {
                        r: rng.gen_range(60..=190),
                        g: 220,
                        b: rng.gen_range(60..=190),
                    };

                    if x == 1 && y == 1 {
                        color = Color8 { r: 255, g: 255, b: 255 };
                    } else if x == 1 && y > 1 {
                        color = Color8::new(240, rng.gen_range(60..=190), rng.gen_range(60..=190));
                    } else if y == 1 && x > 1 {
                        color = Color8::new(rng.gen_range(60..=190), rng.gen_range(60..=190), 240);
                    } else if y >= 2 && x >= 2 {
                        let value = rng.gen_range(60..=190);
                        color = Color8::new(value, value, value);
                    }

                    for rlm in &continent.realms {
                        for pv in &rlm.provinces {
                            for rg in &pv.regions {
                                for px in &rg.pixels {
                                    let pixel = imgbuf.get_pixel_mut(px.0 as u32, px.1 as u32);
                                    *pixel = Rgb([color.r, color.g, color.b]);
                                }
                            }
                        }
                    }
                },
                None => {

                },
            }
        }
    }

    // create the actual image
    imgbuf.save(image_name).unwrap();
}

