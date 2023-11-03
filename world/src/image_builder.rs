use std::collections::HashMap;

use gamescript::models::{
    color::Color8,
    continent::{Continent, Planet, PlanetSettings, Province, Realm, Region},
    point::Size16,
};
use image::{ImageBuffer, Luma, Rgb};
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
    let mut imgbuf: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(
        planet_settings.final_img_size.width as u32,
        planet_settings.final_img_size.height as u32,
    );

    // center
    for x in 1..planet_settings.final_continent_grid_size.width {
        for y in 1..planet_settings.final_continent_grid_size.height {
            let continent = planet.continents.get(&(x, y)).unwrap();
            iterate_realms_and_draw_regions(continent, &mut imgbuf);
        }
    }

    // top / bottom
    for x in 1..planet_settings.final_continent_grid_size.width {
        let y = 0;
        let continent = planet.edge_continents.get(&(x, y)).unwrap();
        iterate_realms_and_draw_regions(continent, &mut imgbuf);
    }

    // left / right
    for y in 0..planet_settings.final_continent_grid_size.height {
        let x = 0;
        let continent = planet.edge_continents.get(&(x, y)).unwrap();
        iterate_realms_and_draw_regions(continent, &mut imgbuf);
    }

    // create the actual image
    imgbuf.save(image_name).unwrap();
}

fn iterate_realms_and_draw_regions(
    continent: &Continent,
    imgbuf: &mut ImageBuffer<Luma<u8>, Vec<u8>>,
) {
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

pub fn debug_planet_image(
    planet: &Planet,
    planet_settings: &PlanetSettings,
    image_name: &str,
    final_s: bool,
) {
    let img_width = if final_s == true {
        planet_settings.final_img_size.width
    } else {
        planet_settings.img_size.width
    };
    let img_height = if final_s == true {
        planet_settings.final_img_size.height
    } else {
        planet_settings.img_size.height
    };

    let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::new(img_width as u32, img_height as u32);

    let width = if final_s == true {
        planet_settings.final_continent_grid_size.width
    } else {
        planet_settings.continent_grid_size.width
    };
    let height = if final_s == true {
        planet_settings.final_continent_grid_size.height
    } else {
        planet_settings.continent_grid_size.height
    };

    // center
    for x in 1..width {
        for y in 1..height {
            let continent_opt: Option<&Continent> = planet.continents.get(&(x as u16, y));
            draw_continent(continent_opt, &mut imgbuf, x, y, final_s);
        }
    }

    // top / bottom
    for x in 1..width {
        let y = 0;
        let continent_opt: Option<&Continent> = planet.edge_continents.get(&(x as u16, y));
        draw_continent(continent_opt, &mut imgbuf, x, y, final_s);
    }

    // left / right
    for y in 0..height {
        let x = 0;
        let continent_opt: Option<&Continent> = planet.edge_continents.get(&(x as u16, y));
        draw_continent(continent_opt, &mut imgbuf, x, y, final_s);
    }

    // create the actual image
    imgbuf.save(image_name).unwrap();
}

fn draw_continent(
    continent_opt: Option<&Continent>,
    imgbuf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    x: u16,
    y: u16,
    final_s: bool,
) {
    match continent_opt {
        Some(continent) => {
            // println!("Draw continent ({}, {})", x, y);
            let mut rng = rand::thread_rng();
            let mut color = Color8 {
                r: rng.gen_range(60..=190),
                g: 220,
                b: rng.gen_range(60..=190),
            };

            if final_s == false {
                if x == 0 && y == 0 {
                    color = Color8 {
                        r: 255,
                        g: 255,
                        b: 255,
                    };
                } else if x == 0 && y > 0 {
                    color = Color8::new(240, rng.gen_range(60..=190), rng.gen_range(60..=190));
                } else if y == 0 && x > 0 {
                    color = Color8::new(rng.gen_range(60..=190), rng.gen_range(60..=190), 240);
                } else if y >= 1 && x >= 1 {
                    let value = rng.gen_range(60..=190);
                    color = Color8::new(value, value, value);
                }
            }

            for rlm in &continent.realms {
                for pv in &rlm.provinces {
                    for rg in &pv.regions {
                        if final_s == true {
                            color = Color8::new(rg.grey_value, rg.grey_value, rg.grey_value);
                        }

                        for px in &rg.pixels {
                            let pixel = imgbuf.get_pixel_mut(px.0 as u32, px.1 as u32);
                            *pixel = Rgb([color.r, color.g, color.b]);
                        }
                    }
                }
            }
        }
        None => {
            // println!("Continent not found at ({}, {})", x, y);
        }
    }
}
