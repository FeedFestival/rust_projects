use gamescript::models::{
    continent::{Continent, Planet, PlanetSettings, Province, Realm, Region},
    point::{
        calculate_distance, calculate_pixel_pos, center_of_two_points, denormalize_u8,
        normalize_u8, try_map_points_min_max_points_by_points, Point16, Size16,
    },
};
use image::GrayImage;
use rand::Rng;
use std::collections::HashMap;
use voronoice::Point;
use world::image_gradient;

pub fn build_regions_and_assign_sites(sites: Vec<Point>) -> Vec<Region> {
    let mut regions = Vec::with_capacity(sites.len());

    for i in 0..sites.len() {
        regions.push(Region::new(Point16 {
            x: sites[i].x as u16,
            y: sites[i].y as u16,
        }));
    }

    regions
}

pub fn build_provinces_and_generate_sites(planet_settings: &PlanetSettings) -> Vec<Province> {
    let mut provinces: Vec<Province> = Vec::with_capacity(
        (planet_settings.province_grid_size.width * planet_settings.province_grid_size.height)
            as usize,
    );

    for x in 0..planet_settings.province_grid_size.width {
        for y in 0..planet_settings.province_grid_size.height {
            let random_x =
                rand::thread_rng().gen_range(0..planet_settings.province_cell_size.width);
            let random_y =
                rand::thread_rng().gen_range(0..planet_settings.province_cell_size.height);
            let site_point = Point16 {
                x: ((x * planet_settings.province_cell_size.width) + random_x),
                y: ((y * planet_settings.province_cell_size.height) + random_y),
            };

            provinces.push(Province::new(Point16 { x, y }, site_point));
        }
    }

    provinces
}

pub fn build_realms_and_generate_sites(planet_setting: &PlanetSettings) -> Vec<Realm> {
    let mut realms: Vec<Realm> = Vec::with_capacity(
        (planet_setting.realm_grid_size.width * planet_setting.realm_cell_size.height) as usize,
    );

    for x in 0..planet_setting.realm_grid_size.width {
        for y in 0..planet_setting.realm_grid_size.height {
            let random_x = rand::thread_rng().gen_range(0..planet_setting.realm_cell_size.width);
            let random_y = rand::thread_rng().gen_range(0..planet_setting.realm_cell_size.height);
            let site_point = Point16 {
                x: ((x * planet_setting.realm_cell_size.width) + random_x),
                y: ((y * planet_setting.realm_cell_size.height) + random_y),
            };

            realms.push(Realm::new(Point16 { x, y }, site_point));
        }
    }

    realms
}

pub fn build_continents_with_site(
    planet_settings: &PlanetSettings,
) -> HashMap<(u16, u16), Continent> {
    let mut continents: HashMap<(u16, u16), Continent> = HashMap::new();

    for x in 0..planet_settings.continent_grid_size.width {
        for y in 0..planet_settings.continent_grid_size.height {
            let random_x =
                rand::thread_rng().gen_range(0..planet_settings.continent_cell_size.width);
            let random_y =
                rand::thread_rng().gen_range(0..planet_settings.continent_cell_size.height);
            let site = Point16 {
                x: (x * planet_settings.continent_cell_size.width) + random_x,
                y: (y * planet_settings.continent_cell_size.height) + random_y,
            };

            let continent_point = Continent::new(
                Point16 { x, y },
                site,
                image_gradient::get_random_degrees_index(),
                get_random_tectonic_elevation(),
            );

            continents.insert((x, y), continent_point);
        }
    }

    // return sites;
    continents
}

pub fn assign_regions_to_provinces(
    regions: Vec<Region>,
    provinces: &mut Vec<Province>,
    planet_settings: &PlanetSettings,
) {
    // create realms hashmap
    let mut provinces_hmap: HashMap<(u16, u16), (u16, u16, u16, u16, usize)> = HashMap::new();
    let mut i: usize = 0;
    for province in &mut *provinces {
        let realm_tuple = (
            province.grid_coord.x,
            province.grid_coord.y,
            province.site_point.x,
            province.site_point.y,
            i,
        );

        provinces_hmap.insert((province.grid_coord.x, province.grid_coord.y), realm_tuple);
        i += 1;
    }

    // iterate over realms and assing provinces to realms hashmap
    for region in regions {
        let p_x = (region.site_point.x as f32 / planet_settings.province_cell_size.width as f32)
            .floor() as u16;
        let p_y = (region.site_point.y as f32 / planet_settings.province_cell_size.height as f32)
            .floor() as u16;

        let mut nearest_distance = f32::INFINITY;
        let mut nearest_point = Point16::new(0, 0);

        let fromx: i32 = p_x as i32 - 1;
        let tox: i32 = p_x as i32 + 1;
        for bx in fromx..tox {
            let fromy = p_y as i32 - 1;
            let toy = p_y as i32 + 1;
            for by in fromy..toy {
                // Skip if the neighbor cell is out of the grid bounds.
                if bx < 0
                    || by < 0
                    || bx >= planet_settings.province_grid_size.width as i32
                    || by >= planet_settings.province_grid_size.height as i32
                {
                    continue;
                }

                // Calculate the distance between the current pixel and the point in the neighboring cell.
                let spx = provinces_hmap[&(bx as u16, by as u16)].2;
                let spy = provinces_hmap[&(bx as u16, by as u16)].3;
                let realm_site_point = Point16::new(spx, spy);
                let distance = calculate_distance(&region.site_point, &realm_site_point);
                // If the calculated distance is less than the current minimum distance.
                if distance < nearest_distance {
                    // Update the minimum distance.
                    nearest_distance = distance;
                    // Update the nearest point.
                    nearest_point = Point16 {
                        x: provinces_hmap[&(bx as u16, by as u16)].0,
                        y: provinces_hmap[&(bx as u16, by as u16)].1,
                    }
                }
            }
        }

        let province_index: usize = provinces_hmap[&(nearest_point.x, nearest_point.y)].4 as usize;
        if let Some(province) = provinces.get_mut(province_index) {
            let rg = Region {
                site_point: region.site_point,
                bottom_left: region.bottom_left,
                top_right: region.top_right,
                grey_value: region.grey_value,
                pixels: region.pixels,
            };

            // assign new corners if we found new min or max
            try_map_points_min_max_points_by_points(
                &mut province.bottom_left,
                &mut province.top_right,
                &rg.bottom_left,
                &rg.top_right,
            );

            province.regions.push(rg);
        }
    }
}

pub fn assign_provinces_to_realms(
    provinces: Vec<Province>,
    realms: &mut Vec<Realm>,
    planet_settings: &PlanetSettings,
) {
    // create realms hashmap
    let mut realms_hmap: HashMap<(u16, u16), (u16, u16, u16, u16, usize)> = HashMap::new();
    let mut i: usize = 0;
    for realm in &mut *realms {
        let realm_tuple = (
            realm.grid_coord.x,
            realm.grid_coord.y,
            realm.site_point.x,
            realm.site_point.y,
            i,
        );

        realms_hmap.insert((realm.grid_coord.x, realm.grid_coord.y), realm_tuple);
        i += 1;
    }

    // iterate over realms and assing provinces to realms hashmap
    for province in provinces {
        let p_x = (province.site_point.x as f32 / planet_settings.realm_cell_size.width as f32)
            .floor() as u16;
        let p_y = (province.site_point.y as f32 / planet_settings.realm_cell_size.height as f32)
            .floor() as u16;

        let mut nearest_distance = f32::INFINITY;
        let mut nearest_point = Point16::new(0, 0);

        let fromx: i32 = p_x as i32 - 1;
        let tox: i32 = p_x as i32 + 1;
        for bx in fromx..tox {
            let fromy = p_y as i32 - 1;
            let toy = p_y as i32 + 1;
            for by in fromy..toy {
                // Skip if the neighbor cell is out of the grid bounds.
                if bx < 0
                    || by < 0
                    || bx >= planet_settings.realm_grid_size.width as i32
                    || by >= planet_settings.realm_grid_size.height as i32
                {
                    continue;
                }

                // Calculate the distance between the current pixel and the point in the neighboring cell.
                let spx = realms_hmap[&(bx as u16, by as u16)].2;
                let spy = realms_hmap[&(bx as u16, by as u16)].3;
                let realm_site_point = Point16::new(spx, spy);
                let distance = calculate_distance(&province.site_point, &realm_site_point);
                // If the calculated distance is less than the current minimum distance.
                if distance < nearest_distance {
                    // Update the minimum distance.
                    nearest_distance = distance;
                    // Update the nearest point.
                    nearest_point = Point16 {
                        x: realms_hmap[&(bx as u16, by as u16)].0,
                        y: realms_hmap[&(bx as u16, by as u16)].1,
                    }
                }
            }
        }

        let realm_index: usize = realms_hmap[&(nearest_point.x, nearest_point.y)].4 as usize;
        if let Some(realm) = realms.get_mut(realm_index) {
            let pv = Province {
                grid_coord: province.grid_coord,
                site_point: province.site_point,
                top_right: province.top_right,
                bottom_left: province.bottom_left,
                average_grey_value: province.average_grey_value,
                regions: province.regions,
            };

            // assign new corners if we found new min or max
            try_map_points_min_max_points_by_points(
                &mut realm.bottom_left,
                &mut realm.top_right,
                &pv.bottom_left,
                &pv.top_right,
            );

            realm.provinces.push(pv);
        }
    }
}

pub fn assign_realms_to_continents(
    realms: Vec<Realm>,
    continents: &mut HashMap<(u16, u16), Continent>,
    planet_settings: &PlanetSettings,
) {
    // iterate over realms
    for realm in realms {
        let p_x = (realm.site_point.x as f32 / planet_settings.continent_cell_size.width as f32)
            .floor() as u16;
        let p_y = (realm.site_point.y as f32 / planet_settings.continent_cell_size.height as f32)
            .floor() as u16;

        let mut nearest_distance = f32::INFINITY;
        let mut nearest_point = Point16::new(0, 0);

        let fromx: i32 = p_x as i32 - 1;
        let tox: i32 = p_x as i32 + 1;
        for bx in fromx..tox {
            let fromy = p_y as i32 - 1;
            let toy = p_y as i32 + 1;
            for by in fromy..toy {
                // Skip if the neighbor cell is out of the grid bounds.
                if bx < 0
                    || by < 0
                    || bx >= planet_settings.continent_grid_size.width as i32
                    || by >= planet_settings.continent_grid_size.height as i32
                {
                    continue;
                }

                // Calculate the distance between the current pixel and the point in the neighboring cell.
                let distance = calculate_distance(
                    &realm.site_point,
                    &continents[&(bx as u16, by as u16)].site_point,
                );
                // If the calculated distance is less than the current minimum distance.
                if distance < nearest_distance {
                    // Update the minimum distance.
                    nearest_distance = distance;
                    // Update the nearest point.
                    nearest_point = Point16 {
                        x: continents[&(bx as u16, by as u16)].grid_coord.x,
                        y: continents[&(bx as u16, by as u16)].grid_coord.y,
                    }
                }
            }
        }

        continents
            .get_mut(&(nearest_point.x, nearest_point.y))
            .map(|continent| {
                let rlm = Realm {
                    grid_coord: realm.grid_coord,
                    site_point: realm.site_point,
                    top_right: realm.top_right,
                    bottom_left: realm.bottom_left,
                    average_grey_value: realm.average_grey_value,
                    provinces: realm.provinces,
                };

                // assign new corners if we found new min or max
                try_map_points_min_max_points_by_points(
                    &mut continent.bottom_left,
                    &mut continent.top_right,
                    &rlm.bottom_left,
                    &rlm.top_right,
                );

                continent.realms.push(rlm);
            });
    }
}

pub fn merge_continents(
    continents: &mut HashMap<(u16, u16), Continent>,
    planet_settings: &PlanetSettings,
) -> (
    HashMap<(u16, u16), Continent>,
    HashMap<(u16, u16), Continent>,
) {
    let mut edge_continents: HashMap<(u16, u16), Continent> = HashMap::new();
    let mut new_continents: HashMap<(u16, u16), Continent> = HashMap::new();

    println!("{:?}", continents.len());

    // remove first column
    println!(" \n- START remove first column -> \n \n ");
    for y in 0..planet_settings.continent_grid_size.height {
        let x = 0;
        continents.remove(&(x, y));
    }
    println!(" \n \nFINISHED continents.len(): {:?}", continents.len());

    // remove first row
    println!(" \n- START remove first row -> \n \n ");
    for x in 1..planet_settings.continent_grid_size.width {
        let y = 0;
        continents.remove(&(x, y));
    }
    println!(" \n \nFINISHED continents.len(): {:?}", continents.len());

    // create the 4 cubed
    println!(" \n- START create the 4 sized -> \n \n ");
    let insert_at = (0, 0);
    let moved_coord = &(2, 2);
    let mut continent = continents.remove(&moved_coord).unwrap();
    let top_c = continents.remove(&(2, 1)).unwrap();
    try_map_points_min_max_points_by_points(
        &mut continent.bottom_left,
        &mut continent.top_right,
        &top_c.bottom_left,
        &top_c.top_right,
    );
    for realm in top_c.realms {
        continent.realms.push(realm);
    }
    let left_c = continents.remove(&(1, 2)).unwrap();
    try_map_points_min_max_points_by_points(
        &mut continent.bottom_left,
        &mut continent.top_right,
        &left_c.bottom_left,
        &left_c.top_right,
    );
    for realm in left_c.realms {
        continent.realms.push(realm);
    }
    let top_left_c = continents.remove(&(1, 1)).unwrap();
    try_map_points_min_max_points_by_points(
        &mut continent.bottom_left,
        &mut continent.top_right,
        &top_left_c.bottom_left,
        &top_left_c.top_right,
    );
    for realm in top_left_c.realms {
        continent.realms.push(realm);
    }

    continent.grid_coord = Point16::new(insert_at.0, insert_at.1);
    continent.site_point = center_of_two_points(&continent.site_point, &top_left_c.site_point);

    println!("into: {:?} <- INSERT {:?}", insert_at, moved_coord);
    println!("into: {:?} <- INSERT {:?} + realms", insert_at, (2, 1));
    println!("into: {:?} <- INSERT {:?} + realms", insert_at, (1, 2));
    println!("into: {:?} <- INSERT {:?} + realms", insert_at, (2, 2));
    edge_continents.insert(insert_at, continent);

    println!(
        " \n \nFINISHED continents.len(): {:?}, edge_continents.len(): {:?}",
        continents.len(),
        edge_continents.len()
    );

    // create the 2 2 vertical
    println!(" \n- START create the 2 2 vertical -> \n \n ");
    for y in 3..planet_settings.continent_grid_size.height {
        let x = 2;

        let insert_at = (x - 2, y - 2);
        let moved_coord = (x, y);
        let to_remove = &(x - 1, y);
        let mut continent = continents.remove(&moved_coord).unwrap();
        let left_c = continents.remove(to_remove).unwrap();

        try_map_points_min_max_points_by_points(
            &mut continent.bottom_left,
            &mut continent.top_right,
            &left_c.bottom_left,
            &left_c.top_right,
        );
        continent.grid_coord = Point16::new(insert_at.0, insert_at.1);
        continent.site_point = center_of_two_points(&continent.site_point, &left_c.site_point);

        for realm in left_c.realms {
            continent.realms.push(realm);
        }

        println!(
            "into: {:?} <- INSERT {:?} + realms of {:?}",
            insert_at, moved_coord, to_remove
        );
        edge_continents.insert(insert_at, continent);
    }

    println!(
        " \n \nFINISHED continents.len(): {:?}, new_continents.len(): {:?}",
        continents.len(),
        new_continents.len()
    );

    // create the 2 2 horizontal
    println!(" \n- START create the 2 2 horizontal -> \n \n ");
    for x in 3..planet_settings.continent_grid_size.width {
        let y = 2;

        let insert_at = (x - 2, y - 2);
        let moved_coord = (x, y);
        let to_remove = &(x, y - 1);
        let mut continent = continents.remove(&moved_coord).unwrap();
        let top_c = continents.remove(to_remove).unwrap();
        try_map_points_min_max_points_by_points(
            &mut continent.bottom_left,
            &mut continent.top_right,
            &top_c.bottom_left,
            &top_c.top_right,
        );
        continent.grid_coord = Point16::new(insert_at.0, insert_at.1);
        continent.site_point = center_of_two_points(&continent.site_point, &top_c.site_point);

        for realm in top_c.realms {
            continent.realms.push(realm);
        }

        println!(
            "into: {:?} <- INSERT {:?} + realms of {:?}",
            insert_at, moved_coord, to_remove
        );
        edge_continents.insert(insert_at, continent);
    }

    println!(
        " \n \nFINISHED continents.len(): {:?}, new_continents.len(): {:?}",
        continents.len(),
        new_continents.len()
    );

    // move the rest one index less
    for x in 3..planet_settings.continent_grid_size.width {
        for y in 3..planet_settings.continent_grid_size.height {
            let insert_at = (x - 2, y - 2);
            let to_remove = &(x, y);
            let mut continent = continents.remove(to_remove).unwrap();

            continent.grid_coord = Point16::new(insert_at.0, insert_at.1);

            println!("into: {:?} <- INSERT {:?}", insert_at, to_remove);
            new_continents.insert(insert_at, continent);
        }
    }

    println!("\n\n FINISHED move the rest one index less -> continents.len(): {:?}, new_continents.len(): {:?} \n\n", continents.len(), new_continents.len());

    (edge_continents, new_continents)
}

pub fn move_continents_pixels_towards_edge(planet: &mut Planet, planet_settings: &PlanetSettings) {
    let x_move = planet_settings.continent_cell_size.width * 2;
    let y_move = planet_settings.continent_cell_size.height * 2;

    // center
    for x in 1..planet_settings.final_continent_grid_size.width {
        for y in 1..planet_settings.final_continent_grid_size.height {
            let continent_opt = planet.continents.get_mut(&(x, y));
            move_pixels(continent_opt, planet_settings, x_move, y_move);
        }
    }

    // top / bottom
    for x in 1..planet_settings.final_continent_grid_size.width {
        let y = 0;
        let continent_opt = planet.edge_continents.get_mut(&(x, y));
        move_pixels(continent_opt, planet_settings, x_move, y_move);
    }

    // left / right
    for y in 0..planet_settings.final_continent_grid_size.height {
        let x = 0;
        let continent_opt = planet.edge_continents.get_mut(&(x, y));
        move_pixels(continent_opt, planet_settings, x_move, y_move);
    }
}

fn move_pixels(
    continent_opt: Option<&mut Continent>,
    planet_settings: &PlanetSettings,
    x_move: u16,
    y_move: u16,
) {
    match continent_opt {
        Some(continent) => {
            for rlm in &mut continent.realms {
                for pv in &mut rlm.provinces {
                    for rg in &mut pv.regions {
                        let mut moved_pixeles: Vec<(u16, u16)> = Vec::new();

                        for px in &rg.pixels {
                            let mut new_x = px.0 as i16 - x_move as i16;
                            if new_x < 0 {
                                new_x = (((planet_settings.img_size.width) as i16) + new_x)
                                    - x_move as i16;
                            }
                            let mut new_y = px.1 as i16 - y_move as i16;
                            if new_y < 0 {
                                new_y = (((planet_settings.img_size.height) as i16) + new_y)
                                    - y_move as i16;
                            }

                            moved_pixeles.push((new_x as u16, new_y as u16));
                        }

                        rg.pixels = moved_pixeles;
                    }
                }
            }
        }
        None => {
            // println!("Continent not found at ({}, {})", x, y);
        }
    }
}

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

fn get_random_tectonic_elevation() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.2..0.7)
}
