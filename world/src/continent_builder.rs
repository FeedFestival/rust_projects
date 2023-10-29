use rand::Rng;
use std::{
    cmp::{max, min},
    collections::HashMap,
};
use voronoice::Point;
use world::{
    image_gradient,
    models::{
        continent::{Continent, Region, RegionSite},
        point::PointU16,
    },
};

pub fn build_regions(region_sites: &mut Vec<RegionSite>) -> Vec<Region> {
    let mut regions = Vec::with_capacity(region_sites.len());

    for i in 0..region_sites.len() {
        regions.push(Region {
            x: std::mem::take(&mut region_sites[i].x),
            y: std::mem::take(&mut region_sites[i].y),
            grid_position: PointU16 { x: region_sites[i].grid_position.x as u16, y: region_sites[i].grid_position.y as u16 },
            site_point: PointU16 { x: region_sites[i].point.x as u16, y: region_sites[i].point.y as u16 },
            pixels: std::mem::take(&mut region_sites[i].pixels),
        });
    }

    regions
}

pub fn build_continents_with_site(
    cell_size: &PointU16,
    grid_size: &PointU16,
) -> HashMap<(u16, u16), Continent> {
    let mut continent_points: HashMap<(u16, u16), Continent> = HashMap::new();

    // TODO: refactor and initialize vectors with size
    // let size = grid_size.x * grid_size.y;
    // let mut sites: Vec<Point> = Vec::with_capacity(size as usize);

    let mut i: u16 = 0;
    for x in 0..grid_size.x {
        for y in 0..grid_size.y {
            let random_x = rand::thread_rng().gen_range(0..cell_size.x);
            let random_y = rand::thread_rng().gen_range(0..cell_size.y);
            let site = PointU16 {
                x: (x * cell_size.x) + random_x,
                y: (y * cell_size.y) + random_y,
            };

            let continent_point = Continent {
                index: i,
                grid_coord: PointU16 { x, y },
                grid_position: PointU16 { x: x * cell_size.x, y: y * cell_size.y },
                site_point: site,
                plate_movement_direction: image_gradient::get_random_degrees_index(),
                elevation: get_random_tectonic_elevation(),
                regions: Vec::new(),
                //
                pixels_size: None,
                pixels: None,
            };

            continent_points.insert((x, y), continent_point);
            i += 1;
        }
    }

    // return sites;
    continent_points
}

pub fn assign_regions_to_continents(
    regions: Vec<Region>,
    continents: &mut HashMap<(u16, u16), Continent>,
    continent_grid_size: PointU16,
    continent_cell_size: PointU16,

) {
    // iterate over regions
    for region in regions {
        let p_x = (region.site_point.x as f32 / continent_cell_size.x as f32).floor() as u16;
        let p_y = (region.site_point.y as f32 / continent_cell_size.y as f32).floor() as u16;

        let mut nearest_distance = f32::INFINITY;
        let mut nearest_point = PointU16::new(0, 0);

        let fromx: i32 = p_x as i32 - 1;
        let tox: i32 = p_x as i32 + 1;
        for bx in fromx..tox {
            let fromy = p_y as i32 - 1;
            let toy = p_y as i32 + 1;
            for by in fromy..toy {
                // Skip if the neighbor cell is out of the grid bounds.
                if bx < 0 || by < 0 || bx >= continent_grid_size.x as i32 || by >= continent_grid_size.y as i32 {
                    continue;
                }

                // Calculate the distance between the current pixel and the point in the neighboring cell.
                let distance = calculate_distance(&region.site_point, &continents[&(bx as u16, by as u16)].site_point);
                // If the calculated distance is less than the current minimum distance.
                if distance < nearest_distance {
                    // Update the minimum distance.
                    nearest_distance = distance;
                    // Update the nearest point.
                    nearest_point = PointU16 {
                        x: continents[&(bx as u16, by as u16)].grid_coord.x,
                        y: continents[&(bx as u16, by as u16)].grid_coord.y,
                    }
                }
            }
        }

        continents
            .get_mut(&(nearest_point.x, nearest_point.y))
            .map(|continent| {
                let region = Region {
                    x: region.x,
                    y: region.y,
                    grid_position: region.grid_position,
                    site_point: region.site_point,
                    pixels: region.pixels
                };
                continent.regions.push(region);
            });
    }
}

pub fn get_sites_from_continents(
    continent_points: &HashMap<(u16, u16), Continent>,
    continent_grid_size: PointU16,
) -> Vec<Point> {
    let mut sites: Vec<Point> = Vec::new();

    for x in 0..continent_grid_size.x {
        for y in 0..continent_grid_size.y {
            match continent_points.get(&(x, y)) {
                Some(value) => {
                    sites.push(Point {
                        x: value.site_point.x as f64,
                        y: value.site_point.y as f64,
                    });
                }
                None => {
                    println!("Key not found");
                }
            }
        }
    }

    sites
}

fn get_random_tectonic_elevation() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.2..0.7)
}

fn calculate_distance(a: &PointU16, b: &PointU16) -> f32 {
    let x_diff = b.x as f32 - a.x as f32;
    let y_diff = b.y as f32 - a.y as f32;
    let distance = (x_diff.powi(2) + y_diff.powi(2)).sqrt();
    distance
}

pub fn add_pixels_to_continents(
    continent_points: &mut HashMap<(u16, u16), Continent>,
    site_pixels: Vec<Vec<(u16, u16)>>,
) {
    for cp in continent_points {
        let pixels = site_pixels[cp.1.index as usize].clone();
        cp.1.pixels = Some(pixels);
    }
}
