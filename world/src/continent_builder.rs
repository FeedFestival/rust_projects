use rand::Rng;
use std::collections::HashMap;
use voronoice::Point;
use world::{
    image_gradient,
    models::{
        continent::{Continent, Province, Region},
        point::{Point16, Size16},
    },
};

pub fn build_provinces_and_assign_sites(
    // grid_size: Size16,
    // cell_size: Size16,
    sites: Vec<Point>,
) -> Vec<Province> {
    let mut provinces = Vec::with_capacity(sites.len());

    for i in 0..sites.len() {
        provinces.push(Province {
            site_point: Point16 {
                x: sites[i].x as u16,
                y: sites[i].y as u16,
            },
            pixels: Vec::new(),
        });
    }

    provinces
}

pub fn build_regions_and_generate_sites(grid_size: &Size16, cell_size: &Size16) -> Vec<Region> {
    let mut sites: Vec<Region> = Vec::with_capacity((grid_size.width * grid_size.height) as usize);

    for x in 0..grid_size.width {
        for y in 0..grid_size.height {
            let random_x = rand::thread_rng().gen_range(0..cell_size.width);
            let random_y = rand::thread_rng().gen_range(0..cell_size.height);
            let site_point = Point16 {
                x: ((x * cell_size.width) + random_x),
                y: ((y * cell_size.height) + random_y),
            };

            sites.push(Region::new(Point16 { x, y }, site_point));
        }
    }

    sites
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

            let continent_point = Continent {
                grid_coord: Point16 { x, y },
                site_point: site,
                plate_movement_direction: image_gradient::get_random_degrees_index(),
                elevation: get_random_tectonic_elevation(),
                regions: Vec::new(),
            };

            continents.insert((x, y), continent_point);
        }
    }

    // return sites;
    continents
}

pub fn assign_provinces_to_regions(
    provinces: Vec<Province>,
    regions: &mut Vec<Region>,
    region_grid_size: &Size16,
    region_cell_size: &Size16,
) {
    // create regions hashmap
    let mut regions_hmap: HashMap<(u16, u16), (u16, u16, u16, u16, usize)> = HashMap::new();
    let mut i: usize = 0;
    for region in &mut *regions {
        let region_tuple = (
            region.grid_coord.x,
            region.grid_coord.y,
            region.site_point.x,
            region.site_point.y,
            i,
        );

        regions_hmap.insert((region.grid_coord.x, region.grid_coord.y), region_tuple);
        i += 1;
    }

    // iterate over regions and assing provinces to regions hashmap
    for province in provinces {
        let p_x = (province.site_point.x as f32 / region_cell_size.width as f32).floor() as u16;
        let p_y = (province.site_point.y as f32 / region_cell_size.height as f32).floor() as u16;

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
                    || bx >= region_grid_size.width as i32
                    || by >= region_grid_size.height as i32
                {
                    continue;
                }

                // Calculate the distance between the current pixel and the point in the neighboring cell.
                let spx = regions_hmap[&(bx as u16, by as u16)].2;
                let spy = regions_hmap[&(bx as u16, by as u16)].3;
                let region_site_point = Point16::new(spx, spy);
                let distance = calculate_distance(&province.site_point, &region_site_point);
                // If the calculated distance is less than the current minimum distance.
                if distance < nearest_distance {
                    // Update the minimum distance.
                    nearest_distance = distance;
                    // Update the nearest point.
                    nearest_point = Point16 {
                        x: regions_hmap[&(bx as u16, by as u16)].0,
                        y: regions_hmap[&(bx as u16, by as u16)].1,
                    }
                }
            }
        }

        let region_index: usize = regions_hmap[&(nearest_point.x, nearest_point.y)].4 as usize;
        if let Some(region) = regions.get_mut(region_index) {
            let pv = Province {
                site_point: province.site_point,
                pixels: province.pixels,
            };
            region.provinces.push(pv);
        }
    }
}

pub fn assign_regions_to_continents(
    regions: Vec<Region>,
    continents: &mut HashMap<(u16, u16), Continent>,
    continent_grid_size: Size16,
    continent_cell_size: Size16,
) {
    // iterate over regions
    for region in regions {
        let p_x = (region.site_point.x as f32 / continent_cell_size.width as f32).floor() as u16;
        let p_y = (region.site_point.y as f32 / continent_cell_size.height as f32).floor() as u16;

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
                    &region.site_point,
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
                let rg = Region {
                    grid_coord: region.grid_coord,
                    site_point: region.site_point,
                    provinces: region.provinces,
                };
                continent.regions.push(rg);
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
