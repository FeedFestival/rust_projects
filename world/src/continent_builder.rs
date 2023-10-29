use rand::Rng;
use std::collections::HashMap;
use voronoice::Point;
use world::{
    image_gradient,
    models::{
        continent::{Continent, Province, Realm, Region},
        point::{Point16, Size16, try_map_points_min_max_points_by_points},
    },
};

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

pub fn build_provinces_and_generate_sites(grid_size: &Size16, cell_size: &Size16) -> Vec<Province> {
    let mut provinces: Vec<Province> =
        Vec::with_capacity((grid_size.width * grid_size.height) as usize);

    for x in 0..grid_size.width {
        for y in 0..grid_size.height {
            let random_x = rand::thread_rng().gen_range(0..cell_size.width);
            let random_y = rand::thread_rng().gen_range(0..cell_size.height);
            let site_point = Point16 {
                x: ((x * cell_size.width) + random_x),
                y: ((y * cell_size.height) + random_y),
            };

            provinces.push(Province::new(Point16 { x, y }, site_point));
        }
    }

    provinces
}

pub fn build_realms_and_generate_sites(grid_size: &Size16, cell_size: &Size16) -> Vec<Realm> {
    let mut realms: Vec<Realm> = Vec::with_capacity((grid_size.width * grid_size.height) as usize);

    for x in 0..grid_size.width {
        for y in 0..grid_size.height {
            let random_x = rand::thread_rng().gen_range(0..cell_size.width);
            let random_y = rand::thread_rng().gen_range(0..cell_size.height);
            let site_point = Point16 {
                x: ((x * cell_size.width) + random_x),
                y: ((y * cell_size.height) + random_y),
            };

            realms.push(Realm::new(Point16 { x, y }, site_point));
        }
    }

    realms
}

pub fn build_continents_with_site(
    cell_size: &Size16,
    grid_size: &Size16,
) -> HashMap<(u16, u16), Continent> {
    let mut continents: HashMap<(u16, u16), Continent> = HashMap::new();

    for x in 0..grid_size.width {
        for y in 0..grid_size.height {
            let random_x = rand::thread_rng().gen_range(0..cell_size.width);
            let random_y = rand::thread_rng().gen_range(0..cell_size.height);
            let site = Point16 {
                x: (x * cell_size.width) + random_x,
                y: (y * cell_size.height) + random_y,
            };

            let continent_point = Continent::new(
                Point16 { x, y },
                site,
                image_gradient::get_random_degrees_index(),
                get_random_tectonic_elevation()
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
    province_grid_size: &Size16,
    province_cell_size: &Size16,
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
        let p_x = (region.site_point.x as f32 / province_cell_size.width as f32).floor() as u16;
        let p_y = (region.site_point.y as f32 / province_cell_size.height as f32).floor() as u16;

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
                    || bx >= province_grid_size.width as i32
                    || by >= province_grid_size.height as i32
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
                pixels: region.pixels,
            };

            // assign new corners if we found new min or max
            try_map_points_min_max_points_by_points(
                &mut province.bottom_left, &mut province.top_right,
                &rg.bottom_left, &rg.top_right
            );

            province.regions.push(rg);
        }


    }
}

pub fn assign_provinces_to_realms(
    provinces: Vec<Province>,
    realms: &mut Vec<Realm>,
    realm_grid_size: &Size16,
    realm_cell_size: &Size16,
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
        let p_x = (province.site_point.x as f32 / realm_cell_size.width as f32).floor() as u16;
        let p_y = (province.site_point.y as f32 / realm_cell_size.height as f32).floor() as u16;

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
                    || bx >= realm_grid_size.width as i32
                    || by >= realm_grid_size.height as i32
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
                regions: province.regions,
            };

            // assign new corners if we found new min or max
            try_map_points_min_max_points_by_points(
                &mut realm.bottom_left, &mut realm.top_right,
                &pv.bottom_left, &pv.top_right
            );

            realm.provinces.push(pv);
        }
    }
}

pub fn assign_realms_to_continents(
    realms: Vec<Realm>,
    continents: &mut HashMap<(u16, u16), Continent>,
    continent_grid_size: Size16,
    continent_cell_size: Size16,
) {
    // iterate over realms
    for realm in realms {
        let p_x = (realm.site_point.x as f32 / continent_cell_size.width as f32).floor() as u16;
        let p_y = (realm.site_point.y as f32 / continent_cell_size.height as f32).floor() as u16;

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
                    || bx >= continent_grid_size.width as i32
                    || by >= continent_grid_size.height as i32
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
                    provinces: realm.provinces,
                };

                // assign new corners if we found new min or max
                try_map_points_min_max_points_by_points(
                    &mut continent.bottom_left, &mut continent.top_right,
                    &rlm.bottom_left, &rlm.top_right
                );

                continent.realms.push(rlm);
            });
    }
}

fn get_random_tectonic_elevation() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.2..0.7)
}

fn calculate_distance(a: &Point16, b: &Point16) -> f32 {
    let x_diff = b.x as f32 - a.x as f32;
    let y_diff = b.y as f32 - a.y as f32;
    let distance = (x_diff.powi(2) + y_diff.powi(2)).sqrt();
    distance
}
