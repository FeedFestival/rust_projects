extern crate image;
use std::collections::HashMap;

use rand::Rng;
use voronoice::{BoundingBox, Point, Voronoi, VoronoiBuilder};
use gamescript::models::{continent::{Region, PlanetSettings, Province, Realm, Continent}, point::{Size16, Point16, try_map_min_max_points}};

use gamescript::models::point::{
        calculate_distance, try_map_points_min_max_points_by_points,
    };
use world::image_gradient;

use crate::cache::builder_settings;

pub fn generate_scattered_sites() -> Vec<Point> {
    let settings = builder_settings();
    let len: usize = ((settings.region_grid_size.width / 2)
        * (settings.region_grid_size.height / 2))
        as usize;
    let mut rng = rand::thread_rng();
    let x_range = rand::distributions::Uniform::new(0, settings.img_size.width);
    let y_range = rand::distributions::Uniform::new(0, settings.img_size.height);

    let mut sites: Vec<Point> = Vec::with_capacity(len); // Use a Vec to store the sites

    while sites.len() < len {
        let x = rng.sample(x_range) as f64;
        let y = rng.sample(y_range) as f64;
        let new_site = Point { x, y };

        // Check if the new site is too close to existing sites
        let is_coincident = sites
            .iter()
            .any(|site| (new_site.x - site.x).hypot(new_site.y - site.y) < 1.0);

        if !is_coincident {
            sites.push(new_site);
        }
    }

    sites
}

pub fn build_voronoi_and_apply_site_pixels_and_corners(regions: &mut Vec<Region>) {
    let settings = builder_settings();

    let sites: Vec<Point> = regions
        .iter()
        .map(|r| Point {
            x: r.site_point.x as f64,
            y: r.site_point.y as f64,
        })
        .collect();
    let voronoi = build(&settings.img_size, sites);

    let mut last_site_index = 0;

    for x in 0..settings.img_size.width - 1 {
        for y in 0..settings.img_size.height - 1 {
            let site_index = get_cell_index(&voronoi, last_site_index, x, y);
            last_site_index = site_index;
            regions[site_index as usize].pixels.push((x, y));
        }
    }

    for i in 0..regions.len() {
        let mut bottom_left_x: u16 = u16::MAX;
        let mut bottom_left_y: u16 = u16::MAX;
        let mut top_right_x: u16 = u16::MIN;
        let mut top_right_y: u16 = u16::MIN;

        for j in 0..regions[i].pixels.len() {
            try_map_min_max_points(
                &mut bottom_left_x, &mut bottom_left_y, &mut top_right_x, &mut top_right_y,
                regions[i].pixels[j].0, regions[i].pixels[j].1
            );
        }

        let top_right = Point16::new(top_right_x, top_right_y);
        let bottom_left = Point16::new(bottom_left_x, bottom_left_y);
        
        regions[i].top_right = top_right;
        regions[i].bottom_left = bottom_left;
    }

}


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

pub fn build_provinces_and_generate_sites() -> Vec<Province> {
    let settings = builder_settings();
    let mut provinces: Vec<Province> = Vec::with_capacity(
        (settings.province_grid_size.width * settings.province_grid_size.height)
            as usize,
    );

    for x in 0..settings.province_grid_size.width {
        for y in 0..settings.province_grid_size.height {
            let random_x =
                rand::thread_rng().gen_range(0..settings.province_cell_size.width);
            let random_y =
                rand::thread_rng().gen_range(0..settings.province_cell_size.height);
            let site_point = Point16 {
                x: ((x * settings.province_cell_size.width) + random_x),
                y: ((y * settings.province_cell_size.height) + random_y),
            };

            provinces.push(Province::new(Point16 { x, y }, site_point));
        }
    }

    provinces
}

pub fn build_realms_and_generate_sites(planet_setting: &PlanetSettings) -> Vec<Realm> {
    let settings = builder_settings();
    let mut realms: Vec<Realm> = Vec::with_capacity(
        (settings.realm_grid_size.width * settings.realm_cell_size.height) as usize,
    );

    for x in 0..settings.realm_grid_size.width {
        for y in 0..settings.realm_grid_size.height {
            let random_x = rand::thread_rng().gen_range(0..settings.realm_cell_size.width);
            let random_y = rand::thread_rng().gen_range(0..settings.realm_cell_size.height);
            let site_point = Point16 {
                x: ((x * settings.realm_cell_size.width) + random_x),
                y: ((y * settings.realm_cell_size.height) + random_y),
            };

            realms.push(Realm::new(Point16 { x, y }, site_point));
        }
    }

    realms
}

pub fn build_continents_with_site(
    planet_settings: &PlanetSettings,
) -> HashMap<(u16, u16), Continent> {
    let settings = builder_settings();
    let mut continents: HashMap<(u16, u16), Continent> = HashMap::new();

    for x in 0..settings.continent_grid_size.width {
        for y in 0..settings.continent_grid_size.height {
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
    provinces: &mut Vec<Province>
) {
    let settings = builder_settings();
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
        let p_x = (region.site_point.x as f32 / settings.province_cell_size.width as f32)
            .floor() as u16;
        let p_y = (region.site_point.y as f32 / settings.province_cell_size.height as f32)
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
                    || bx >= settings.province_grid_size.width as i32
                    || by >= settings.province_grid_size.height as i32
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
    realms: &mut Vec<Realm>
) {
    let settings = builder_settings();
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
        let p_x = (province.site_point.x as f32 / settings.realm_cell_size.width as f32)
            .floor() as u16;
        let p_y = (province.site_point.y as f32 / settings.realm_cell_size.height as f32)
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
                    || bx >= settings.realm_grid_size.width as i32
                    || by >= settings.realm_grid_size.height as i32
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
    let settings = builder_settings();
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
                    || bx >= settings.continent_grid_size.width as i32
                    || by >= settings.continent_grid_size.height as i32
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

fn build(img_size: &Size16, sites: Vec<Point>) -> Voronoi {
    let center: Point = Point {
        x: img_size.width as f64 / 2.0,
        y: img_size.height as f64 / 2.0,
    };

    let voronoi: Voronoi = VoronoiBuilder::default()
        .set_sites(sites)
        .set_clip_behavior(voronoice::ClipBehavior::None)
        // image origin is top left corner, center is width/2,height/2
        .set_bounding_box(BoundingBox::new(
            center,
            img_size.width as f64,
            img_size.height as f64,
        ))
        .build()
        .unwrap();

    voronoi
}

fn get_cell_index(voronoi: &Voronoi, current_site: u16, x: u16, y: u16) -> u16 {
    let p = Point {
        x: x as f64,
        y: y as f64,
    };
    voronoi
        .cell(current_site as usize)
        .iter_path(p)
        .last()
        .expect("Expected to find site that contains point") as u16
}

fn get_random_tectonic_elevation() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.2..0.7)
}
