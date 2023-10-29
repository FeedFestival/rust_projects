use rand::Rng;
use std::{
    collections::HashMap,
};
use world::{
    image_gradient,
    models::{
        continent::{Continent, Region},
        point::{Point16, Size16},
    },
};

// pub fn build_regions(region_sites: &mut Vec<Region>) -> Vec<Region> {
//     let mut regions = Vec::with_capacity(region_sites.len());

//     for i in 0..region_sites.len() {
//         regions.push(Region {
//             x: std::mem::take(&mut region_sites[i].x),
//             y: std::mem::take(&mut region_sites[i].y),
//             site_point: PointU16 {
//                 x: region_sites[i].site_point.x as u16,
//                 y: region_sites[i].site_point.y as u16,
//             },
//             pixels: std::mem::take(&mut region_sites[i].pixels),
//         });
//     }

//     regions
// }

pub fn build_continents_with_site(
    cell_size: &Size16,
    grid_size: &Size16,
) -> HashMap<(u16, u16), Continent> {
    let mut continent_points: HashMap<(u16, u16), Continent> = HashMap::new();

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

            continent_points.insert((x, y), continent_point);
        }
    }

    // return sites;
    continent_points
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
                let region = Region {
                    x: region.x,
                    y: region.y,
                    site_point: region.site_point,
                    pixels: region.pixels,
                };
                continent.regions.push(region);
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
