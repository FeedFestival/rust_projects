use std::collections::HashMap;

use gamescript::models::{
    continent::{Planet, Realm, Region},
    point::{calculate_distance, center_of_two_points, denormalize_u8, normalize_u8},
};
use image::{ImageBuffer, Rgb};

const REALM_PX_RANGE_1_1: f32 = 35.0;
const PROVINCE_PX_RANGE_1_1: f32 = 15.0;
const PROVINCE_PX_RANGE_1_2: f32 = 25.0;
const MULTIPLIER_1: f32 = 0.30;
const REALM_PX_RANGE_2: f32 = 55.0;
const PROVINCE_PX_RANGE_2_1: f32 = 35.0;
const PROVINCE_PX_RANGE_2_2: f32 = 45.0;
const MULTIPLIER_2: f32 = 0.20;
const REALM_PX_RANGE_3: f32 = 100.0;
const MULTIPLIER_3: f32 = 0.10;

pub fn equalize_light_realms(planet: &Planet, light_min_value: u8, dark_min_value: u8) -> (HashMap<(u16, u16), u8>, ImageBuffer<Rgb<u8>, Vec<u8>>) {

    let mut light_realms: Vec<&Realm> = Vec::new();
    let mut dark_realms: Vec<&Realm> = Vec::new();

    for x in 0..planet.grid_size.width {
        for y in 0..planet.grid_size.height {
            let continent = planet.continents.get(&(x, y)).unwrap();

            for rlm in &continent.realms {
                if rlm.average_grey_value > light_min_value {
                    light_realms.push(rlm);
                }

                if rlm.average_grey_value < dark_min_value {
                    dark_realms.push(rlm);
                }
            }
        }
    }

    let mut unprocessed_img_buf: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::new(planet.img_size.width as u32, planet.img_size.height as u32);
    let mut img_buf: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::new(planet.img_size.width as u32, planet.img_size.height as u32);
    for x in 0..planet.img_size.width {
        for y in 0..planet.img_size.height {
            let pixel = unprocessed_img_buf.get_pixel_mut(x as u32, y as u32);
            *pixel = Rgb([255, 0, 102]);
            let pixel = img_buf.get_pixel_mut(x as u32, y as u32);
            *pixel = Rgb([255, 0, 102]);
        }
    }

    let mut modified_pixels: HashMap<(u16, u16), u8> = HashMap::new();

    for rlm in light_realms {
        let rlm_normalized = normalize_u8(rlm.average_grey_value as f64);
        let rlm_center = center_of_two_points(&rlm.bottom_left, &rlm.top_right);

        for dkrlm in &dark_realms {
            // check distance between
            let dkrlm_center = center_of_two_points(&dkrlm.bottom_left, &dkrlm.top_right);
            let distance = calculate_distance(&rlm_center, &dkrlm_center);
            if distance < REALM_PX_RANGE_1_1 {
                let dkrlm_normalized = normalize_u8(dkrlm.average_grey_value as f64);
                let multiplied_diff = (rlm_normalized - dkrlm_normalized) * MULTIPLIER_1 as f64;

                for pv in &dkrlm.provinces {
                    for rg in &pv.regions {
                        let rg_distance = calculate_distance(&rlm_center, &rg.site_point);

                        if rg_distance < PROVINCE_PX_RANGE_1_1 {
                            increase_the_value_of_regions_pixels(
                                rg,
                                multiplied_diff,
                                2.0,
                                &mut modified_pixels,
                                &mut unprocessed_img_buf,
                                &mut img_buf,
                            );
                        } else if rg_distance < PROVINCE_PX_RANGE_1_2 {
                            increase_the_value_of_regions_pixels(
                                rg,
                                multiplied_diff,
                                1.5,
                                &mut modified_pixels,
                                &mut unprocessed_img_buf,
                                &mut img_buf,
                            );
                        } else {
                            increase_the_value_of_regions_pixels(
                                rg,
                                multiplied_diff,
                                1.0,
                                &mut modified_pixels,
                                &mut unprocessed_img_buf,
                                &mut img_buf,
                            );
                        }
                    }
                }
            } else if distance < REALM_PX_RANGE_2 {
                let dkrlm_normalized = normalize_u8(dkrlm.average_grey_value as f64);
                let multiplied_diff = (rlm_normalized - dkrlm_normalized) * MULTIPLIER_2 as f64;

                for pv in &dkrlm.provinces {
                    for rg in &pv.regions {
                        let rg_distance = calculate_distance(&rlm_center, &rg.site_point);

                        if rg_distance < PROVINCE_PX_RANGE_2_1 {
                            increase_the_value_of_regions_pixels(
                                rg,
                                multiplied_diff,
                                2.0,
                                &mut modified_pixels,
                                &mut unprocessed_img_buf,
                                &mut img_buf,
                            );
                        } else if rg_distance < PROVINCE_PX_RANGE_2_2 {
                            increase_the_value_of_regions_pixels(
                                rg,
                                multiplied_diff,
                                1.5,
                                &mut modified_pixels,
                                &mut unprocessed_img_buf,
                                &mut img_buf,
                            );
                        } else {
                            increase_the_value_of_regions_pixels(
                                rg,
                                multiplied_diff,
                                1.0,
                                &mut modified_pixels,
                                &mut unprocessed_img_buf,
                                &mut img_buf,
                            );
                        }
                    }
                }
            } else if distance < REALM_PX_RANGE_3 {
                let dkrlm_normalized = normalize_u8(dkrlm.average_grey_value as f64);
                let multiplied_diff = (rlm_normalized - dkrlm_normalized) * MULTIPLIER_3 as f64;

                for pv in &dkrlm.provinces {
                    for rg in &pv.regions {
                        increase_the_value_of_regions_pixels(
                            rg,
                            multiplied_diff,
                            1.0,
                            &mut modified_pixels,
                            &mut unprocessed_img_buf,
                            &mut img_buf,
                        );
                    }
                }
            }
        }

        for pv in &rlm.provinces {
            for rg in &pv.regions {
                for px in &rg.pixels {
                    let pixel = unprocessed_img_buf.get_pixel_mut(px.0 as u32, px.1 as u32);
                    *pixel = Rgb([rg.grey_value, rg.grey_value, rg.grey_value]);
                    let pixel = img_buf.get_pixel_mut(px.0 as u32, px.1 as u32);
                    *pixel = Rgb([rg.grey_value, rg.grey_value, rg.grey_value]);
                }
            }
        }
    }

    unprocessed_img_buf.save("unprocessed.png").unwrap();
    img_buf.save("processed.png").unwrap();

    (modified_pixels, img_buf)
}

fn increase_the_value_of_regions_pixels(
    rg: &Region,
    multiplied_diff: f64,
    multiplier: f64,
    modified_pixels: &mut HashMap<(u16, u16), u8>,
    unprocessed_img_buf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    img_buf: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
) {
    let normalized_value = normalize_u8(rg.grey_value as f64);
    let new_value = normalized_value + (multiplied_diff * multiplier);
    let value = denormalize_u8(new_value);

    for px in &rg.pixels {
        // TODO: refactor so we store the coords on the Regions, Provinces and Realms
        let exists = modified_pixels.contains_key(&(px.0, px.1));
        if exists {
            let old_value = modified_pixels.get(&(px.0, px.1)).unwrap();
            if value < *old_value {
                break;
            } else {
                let pixel = modified_pixels.get_mut(&(px.0, px.1)).unwrap();
                *pixel = value;
            }
        } else {
            modified_pixels.insert((px.0, px.1), value);
        }

        let unprocessed_pixel = unprocessed_img_buf.get_pixel_mut(px.0 as u32, px.1 as u32);
        *unprocessed_pixel = Rgb([rg.grey_value, rg.grey_value, rg.grey_value]);
        let pixel = img_buf.get_pixel_mut(px.0 as u32, px.1 as u32);
        *pixel = Rgb([value, value, value]);
    }
}
