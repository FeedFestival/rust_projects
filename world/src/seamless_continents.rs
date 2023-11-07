use std::collections::HashMap;

use gamescript::models::{
    continent::{Continent, Planet, PlanetSettings, Realm},
    point::{calculate_pixel_pos, denormalize_u8, normalize_u8, Point16, Size16},
};
use image::GrayImage;
use world::image_gradient;

use crate::{cache::builder_settings, pixels_utils};

pub fn assign_continent_gradient_to_pixels(planet: &mut Planet, planet_settings: &PlanetSettings) {
    // load gradient images so we can calculate the pixel value of the region
    let gradient_images = load_gradient_images(planet);

    // iterate over regions and assign continent gradient color to pixels
    // center
    for x in 1..planet_settings.final_continent_grid_size.width {
        for y in 1..planet_settings.final_continent_grid_size.height {
            planet.continents.get_mut(&(x, y)).map(|continent| {
                assign_gradient_color_to_regions(continent, &gradient_images);
            });
        }
    }

    // top / bottom
    for x in 1..planet_settings.final_continent_grid_size.width {
        let y = 0;
        planet.edge_continents.get_mut(&(x, y)).map(|continent| {
            assign_gradient_color_to_regions(continent, &gradient_images);
        });
    }

    // left / right
    for y in 0..planet_settings.final_continent_grid_size.height {
        let x = 0;
        planet.edge_continents.get_mut(&(x, y)).map(|continent| {
            assign_gradient_color_to_regions(continent, &gradient_images);
        });
    }
}

fn assign_gradient_color_to_regions(
    continent: &mut Continent,
    gradient_images: &HashMap<u8, GrayImage>,
) {
    let square_size = Size16::new(
        continent.top_right.x - continent.bottom_left.x,
        continent.top_right.y - continent.bottom_left.y,
    );

    if let Some(gradient_texture) = gradient_images.get(&continent.plate_movement_direction) {
        let per_pixel_size = (
            (gradient_texture.width() as f64 / square_size.width as f64) as f64,
            (gradient_texture.height() as f64 / square_size.height as f64) as f64,
        );

        for rlm in &mut continent.realms {
            let mut realms_agerage: Vec<u8> = Vec::with_capacity(rlm.provinces.len());

            for pv in &mut rlm.provinces {
                let mut provinces_agerage: Vec<u8> = Vec::with_capacity(pv.regions.len());

                for rg in &mut pv.regions {
                    let gradient_pos = Point16::substract(&rg.site_point, &continent.bottom_left);
                    let pixel_pos = calculate_pixel_pos(&gradient_pos, &per_pixel_size);
                    let pixel_color =
                        gradient_texture.get_pixel(pixel_pos.x as u32, pixel_pos.y as u32);

                    let color_value = pixel_color.0[0];
                    let new_value = normalize_u8(color_value as f64) * continent.elevation as f64;
                    rg.grey_value = denormalize_u8(new_value);

                    provinces_agerage.push(rg.grey_value);
                }

                let sum: u32 = provinces_agerage.iter().map(|&x| x as u32).sum();
                pv.average_grey_value = ((sum as f64) / (provinces_agerage.len() as f64)) as u8;

                realms_agerage.push(pv.average_grey_value);
            }

            let sum: u32 = realms_agerage.iter().map(|&x| x as u32).sum();
            rlm.average_grey_value = ((sum as f64) / (realms_agerage.len() as f64)) as u8;
        }
    }
}

pub fn move_continents_pixels_towards_edge(planet: &mut Planet, planet_settings: &PlanetSettings) {
    let x_move = planet_settings.continent_cell_size.width * 2;
    let y_move = planet_settings.continent_cell_size.height * 2;

    // center
    for x in 1..planet_settings.final_continent_grid_size.width {
        for y in 1..planet_settings.final_continent_grid_size.height {
            let continent_opt = planet.continents.get_mut(&(x, y));
            move_pixels(continent_opt, x_move, y_move);
        }
    }

    // top / bottom
    for x in 1..planet_settings.final_continent_grid_size.width {
        let y = 0;
        let continent_opt = planet.edge_continents.get_mut(&(x, y));
        move_pixels(continent_opt, x_move, y_move);
    }

    // left / right
    for y in 0..planet_settings.final_continent_grid_size.height {
        let x = 0;
        let continent_opt = planet.edge_continents.get_mut(&(x, y));
        move_pixels(continent_opt, x_move, y_move);
    }
}

pub fn merge_centered_continents(planet: &mut Planet, planet_settings: &PlanetSettings) {
    let from_y = planet_settings.final_continent_grid_size.height - 1;
    let from_x = planet_settings.final_continent_grid_size.width - 1;

    // merge the opposite edges realms into 00
    merge_opposite_bottom_edges_into_00(planet, from_y);
    merge_opposite_right_edges_into_00(planet, from_x);

    let pixels_at_00 = get_pixels_at_00(&mut planet.edge_continents);
    // println!("pixels_at_00.len(): {}\n", pixels_at_00.len());

    planet.edge_continents.remove(&(0, from_y));
    planet.edge_continents.remove(&(from_x, 0));

    let new_width = planet_settings.final_continent_grid_size.width - 1;
    cleanup_top_edge_pixels(planet, new_width, &pixels_at_00);

    let new_height = planet_settings.final_continent_grid_size.height - 1;
    cleanup_left_edge_pixels(planet, new_height, &pixels_at_00);

    // centered bottom
    let to_y = planet_settings.final_continent_grid_size.height - 2;
    for x in 1..planet_settings.final_continent_grid_size.width {
        move_realms_to_continent(&mut planet.continents, x, x, from_y, to_y);
    }
    // centered right
    let to_x = planet_settings.final_continent_grid_size.width - 2;
    for y in 1..planet_settings.final_continent_grid_size.height {
        move_realms_to_continent(&mut planet.continents, from_x, to_x, y, y);
    }
    for x in 1..planet_settings.final_continent_grid_size.width {
        planet.continents.remove(&(x, from_y));
    }
    for y in 1..planet_settings.final_continent_grid_size.width {
        planet.continents.remove(&(from_x, y));
    }

    let mut pixels_on_edge = get_all_pixels_on_edge(
        &mut planet.edge_continents,
        pixels_at_00,
        new_width,
        new_height,
    );
    cleanup_centered_right_pixels(
        &mut planet.continents,
        &mut pixels_on_edge,
        new_width,
        new_height,
    );
}

fn merge_opposite_bottom_edges_into_00(planet: &mut Planet, from_y: u16) {
    let bottom_continent = planet.edge_continents.get_mut(&(0, from_y)).unwrap();
    let mut removed_realms: Vec<Realm> = Vec::with_capacity(bottom_continent.realms.len());

    let length = bottom_continent.realms.len();
    for i in (0..length).rev() {
        let realm = bottom_continent.realms.remove(i);
        removed_realms.push(realm);
    }

    let continent_at_00 = planet.edge_continents.get_mut(&(0, 0)).unwrap();

    continent_at_00.realms.append(&mut removed_realms);
}

fn merge_opposite_right_edges_into_00(planet: &mut Planet, from_x: u16) {
    let right_continent = planet.edge_continents.get_mut(&(from_x, 0)).unwrap();
    let mut removed_realms: Vec<Realm> = Vec::with_capacity(right_continent.realms.len());

    let length = right_continent.realms.len();
    for i in (0..length).rev() {
        let realm = right_continent.realms.remove(i);
        removed_realms.push(realm);
    }

    let continent_at_00 = planet.edge_continents.get_mut(&(0, 0)).unwrap();

    continent_at_00.realms.append(&mut removed_realms);
}

fn get_pixels_at_00(edge_continents: &mut HashMap<(u16, u16), Continent>) -> Vec<(u16, u16)> {
    let continent_at_00 = edge_continents.get_mut(&(0, 0)).unwrap();

    clean_continent(continent_at_00);

    let mut pixels: HashMap<(u16, u16), bool> = HashMap::new();
    pixels_utils::remove_pixels_from_continent(continent_at_00, &mut pixels, true);

    let mut pixels_at_00: Vec<(u16, u16)> = Vec::new();

    for px in pixels {
        pixels_at_00.push(px.0);
    }

    pixels_at_00
}

fn clean_continent(continent_at_00: &mut Continent) {
    // println!("\n\n\n START clean_continent");

    let mut to_remove_realms: Vec<u16> = Vec::new();
    let mut rlm_i = 0;

    for rlm in &mut continent_at_00.realms {
        if rlm.provinces.len() == 0 {
            // println!("---- found an empty realm ----- {}", rlm_i);
            to_remove_realms.push(rlm_i);
        } else {
            let mut to_remove_provinces: Vec<u16> = Vec::new();
            let mut pv_i = 0;

            for pv in &mut rlm.provinces {
                if pv.regions.len() == 0 {
                    // println!("---- found an empty province ----- {}", pv_i);
                    to_remove_provinces.push(pv_i);
                } else {
                    let mut to_remove_regions: Vec<u16> = Vec::new();
                    let mut r_i = 0;

                    for rg in &mut pv.regions {
                        if rg.pixels.len() == 0 {
                            // println!("---- found an empty region -----");
                            to_remove_regions.push(r_i);
                        }

                        r_i += 1;
                    }

                    let r_length = to_remove_regions.len();
                    if r_length > 0 {
                        to_remove_regions.sort();
                        // println!("------ removing {} regions --- ", r_length);
                        for i in (0..r_length).rev() {
                            pv.regions
                                .remove(*to_remove_regions.get(i).unwrap() as usize);
                        }
                    }
                }

                pv_i += 1;
            }
        }

        rlm_i += 1;
    }

    let rlm_length = to_remove_realms.len();
    if rlm_length > 0 {
        to_remove_realms.sort();
        // println!("------ removing {} realms --- ", rlm_length);
        for i in (0..rlm_length).rev() {
            continent_at_00
                .realms
                .remove(*to_remove_realms.get(i).unwrap() as usize);
        }
    }

    // println!("\n FINISHED clean_continent\n\n\n");
}

fn cleanup_top_edge_pixels(planet: &mut Planet, new_width: u16, pixels_at_00: &Vec<(u16, u16)>) {
    let mut top_edge_pixels = get_top_edge_pixels(&planet.edge_continents, new_width);
    // println!("top_edge_pixels.len(): {}", top_edge_pixels.len());

    mark_overlaping_pixels(&pixels_at_00, &mut top_edge_pixels);

    cleanup_edge_pixels_for_top(planet, new_width, &top_edge_pixels);
}

fn cleanup_left_edge_pixels(planet: &mut Planet, new_height: u16, pixels_at_00: &Vec<(u16, u16)>) {
    let mut left_edge_pixels = get_end_left_edge_pixels(&planet.edge_continents, new_height);
    // println!("left_edge_pixels.len(): {}", left_edge_pixels.len());

    mark_overlaping_pixels(&pixels_at_00, &mut left_edge_pixels);

    cleanup_edge_pixels_for_left(planet, new_height, &left_edge_pixels);
}

fn get_top_edge_pixels(
    edge_continents: &HashMap<(u16, u16), Continent>,
    new_width: u16,
) -> HashMap<(u16, u16), bool> {
    let mut top_edge_pixels: HashMap<(u16, u16), bool> = HashMap::new();
    for x in (new_width - 1)..new_width {
        let y = 0;
        let edge_continent = edge_continents.get(&(x, y)).unwrap();

        for rlm in &edge_continent.realms {
            for pv in &rlm.provinces {
                for rg in &pv.regions {
                    for px in &rg.pixels {
                        top_edge_pixels.insert(*px, false);
                    }
                }
            }
        }
    }
    top_edge_pixels
}

fn get_end_left_edge_pixels(
    edge_continents: &HashMap<(u16, u16), Continent>,
    new_height: u16,
) -> HashMap<(u16, u16), bool> {
    let mut left_edge_pixels: HashMap<(u16, u16), bool> = HashMap::new();
    for y in (new_height - 1)..new_height {
        let x = 0;
        let edge_continent = edge_continents.get(&(x, y)).unwrap();

        for rlm in &edge_continent.realms {
            for pv in &rlm.provinces {
                for rg in &pv.regions {
                    for px in &rg.pixels {
                        left_edge_pixels.insert(*px, false);
                    }
                }
            }
        }
    }
    left_edge_pixels
}

fn mark_overlaping_pixels(
    pixels_at_00: &Vec<(u16, u16)>,
    unmarked_pixels: &mut HashMap<(u16, u16), bool>,
) {
    for i in 0..pixels_at_00.len() {
        let top_left_px = *pixels_at_00.get(i).unwrap();
        let overlap = unmarked_pixels.contains_key(&top_left_px);
        if overlap {
            unmarked_pixels.insert(top_left_px, true);
        }
    }
}

fn cleanup_edge_pixels_for_top(
    planet: &mut Planet,
    new_width: u16,
    top_edge_pixels: &HashMap<(u16, u16), bool>,
) {
    for x in (new_width - 1)..new_width {
        let y = 0;
        let edge_continent: &mut Continent = planet.edge_continents.get_mut(&(x, y)).unwrap();
        remove_overlaping_pixels_from_regions(edge_continent, top_edge_pixels);
    }
}

fn cleanup_edge_pixels_for_left(
    planet: &mut Planet,
    new_height: u16,
    left_edge_pixels: &HashMap<(u16, u16), bool>,
) {
    for y in (new_height - 1)..new_height {
        let x = 0;
        let edge_continent: &mut Continent = planet.edge_continents.get_mut(&(x, y)).unwrap();
        remove_overlaping_pixels_from_regions(edge_continent, left_edge_pixels);
    }
}

fn remove_overlaping_pixels_from_regions(
    edge_continent: &mut Continent,
    edge_pixels: &HashMap<(u16, u16), bool>,
) {
    for rlm in &mut edge_continent.realms {
        for pv in &mut rlm.provinces {
            for rg in &mut pv.regions {
                let mut to_remove_edge_pixels: Vec<u16> = Vec::new();
                let mut ipx: u16 = 0;

                for px in &rg.pixels {
                    let is_same_pixel = edge_pixels.contains_key(px);
                    if is_same_pixel {
                        let overlaping = *edge_pixels.get(px).unwrap();
                        if overlaping {
                            to_remove_edge_pixels.push(ipx);
                        }
                    }
                    ipx += 1;
                }

                for i in (0..to_remove_edge_pixels.len()).rev() {
                    rg.pixels
                        .remove(*to_remove_edge_pixels.get(i).unwrap() as usize);
                }
            }
        }
    }
}

fn move_realms_to_continent(
    continents: &mut HashMap<(u16, u16), Continent>,
    from_x: u16,
    to_x: u16,
    from_y: u16,
    to_y: u16,
) {
    let continent = continents.get_mut(&(from_x, from_y)).unwrap();
    let mut removed_realms: Vec<Realm> = Vec::with_capacity(continent.realms.len());
    // println!(
    //     "continent ({:?}, {:?}), continent.realms.len(): {:?}\n",
    //     from_x,
    //     from_y,
    //     continent.realms.len()
    // );

    let length = continent.realms.len();

    for i in (0..length).rev() {
        // // println!("- remove realm at {}", i);
        let realm = continent.realms.remove(i);
        removed_realms.push(realm);
    }

    // // println!("\ncontinent.realms.len(): {:?}, removed_realms(): {:?}", continent.realms.len(), removed_realms.len());

    let top_continent = continents.get_mut(&(to_x, to_y)).unwrap();
    // // println!("\ntop_continent.realms: {:?}", top_continent.realms.len());

    top_continent.realms.append(&mut removed_realms);

    // // println!(
    //     "\n - after add, top_continent.realms: {:?}",
    //     top_continent.realms.len()
    // );
}

fn move_pixels(continent_opt: Option<&mut Continent>, x_move: u16, y_move: u16) {
    let settings = builder_settings();

    match continent_opt {
        Some(continent) => {
            for rlm in &mut continent.realms {
                for pv in &mut rlm.provinces {
                    for rg in &mut pv.regions {
                        let mut moved_pixeles: Vec<(u16, u16)> = Vec::new();

                        for px in &rg.pixels {
                            let mut new_x = px.0 as i16 - x_move as i16;
                            if new_x < 0 {
                                new_x =
                                    (((settings.img_size.width) as i16) + new_x) - x_move as i16;
                            }
                            let mut new_y = px.1 as i16 - y_move as i16;
                            if new_y < 0 {
                                new_y =
                                    (((settings.img_size.height) as i16) + new_y) - y_move as i16;
                            }

                            moved_pixeles.push((new_x as u16, new_y as u16));
                        }

                        rg.pixels = moved_pixeles;
                    }
                }
            }
        }
        None => {
            // // println!("Continent not found at ({}, {})", x, y);
        }
    }
}

fn load_gradient_images(planet: &Planet) -> HashMap<u8, GrayImage> {
    let mut gradient_images: HashMap<u8, GrayImage> = HashMap::new();
    for continent in planet.edge_continents.values() {
        let exists = gradient_images.contains_key(&continent.plate_movement_direction);
        if exists == false {
            let degrees = image_gradient::get_degrees_by_index(continent.plate_movement_direction);
            let gradient_image: GrayImage = image_gradient::load_gradient(degrees);
            gradient_images.insert(continent.plate_movement_direction, gradient_image);
        }
    }
    for continent in planet.continents.values() {
        let exists = gradient_images.contains_key(&continent.plate_movement_direction);
        if exists == false {
            let degrees = image_gradient::get_degrees_by_index(continent.plate_movement_direction);
            let gradient_image: GrayImage = image_gradient::load_gradient(degrees);
            gradient_images.insert(continent.plate_movement_direction, gradient_image);
        }
    }

    gradient_images
}

fn get_all_pixels_on_edge(
    edge_continents: &mut HashMap<(u16, u16), Continent>,
    pixels_at_00: Vec<(u16, u16)>,
    width: u16,
    height: u16,
) -> HashMap<(u16, u16), bool> {
    let mut pixels_on_edge: HashMap<(u16, u16), bool> = HashMap::new();

    for px_00 in pixels_at_00 {
        pixels_on_edge.insert(px_00, false);
    }

    for x in 1..width {
        let y = 0;
        let edge_continent = edge_continents.get(&(x, y)).unwrap();
        for rlm in &edge_continent.realms {
            for pv in &rlm.provinces {
                for rg in &pv.regions {
                    for px in &rg.pixels {
                        pixels_on_edge.insert(*px, false);
                    }
                }
            }
        }
    }

    for y in 1..height {
        let x = 0;
        let edge_continent = edge_continents.get(&(x, y)).unwrap();
        for rlm in &edge_continent.realms {
            for pv in &rlm.provinces {
                for rg in &pv.regions {
                    for px in &rg.pixels {
                        pixels_on_edge.insert(*px, false);
                    }
                }
            }
        }
    }

    pixels_on_edge
}

fn cleanup_centered_right_pixels(
    continents: &mut HashMap<(u16, u16), Continent>,
    pixels_on_edge: &mut HashMap<(u16, u16), bool>,
    width: u16,
    height: u16,
) {
    println!("\n\n\n START cleanup_centered_right_pixels \n");

    for x in 1..width {
        let y = height - 1;
        let continent = continents.get_mut(&(x, y)).unwrap();
        clean_continent(continent);
        pixels_utils::remove_pixels_from_continent(continent, pixels_on_edge, false);
    }

    for y in 1..height {
        let x = width - 1;
        let continent = continents.get_mut(&(x, y)).unwrap();
        clean_continent(continent);
        pixels_utils::remove_pixels_from_continent(continent, pixels_on_edge, false);
    }

    println!("\n FINISHED cleanup_centered_right_pixels \n\n\n");
}

