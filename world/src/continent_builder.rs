use rand::Rng;
use std::collections::HashMap;
use voronoice::Point;
use world::{
    image_gradient,
    models::{
        continent::{Continent, Region},
        point::PointU16,
    },
};

pub fn build_regions(pixels_vec: &mut Vec<Vec<(u16, u16)>>) -> Vec<Region> {
    let mut regions = Vec::with_capacity(pixels_vec.len());

    for i in 0..pixels_vec.len() {
        let pixels = std::mem::take(&mut pixels_vec[i]);
        regions.push(Region { pixels });
    }

    regions
}

pub fn build(cell_size: PointU16, grid_size: &PointU16) -> HashMap<(u16, u16), Continent> {
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
                grid_position: PointU16 { x, y },
                site,
                plate_movement_direction: image_gradient::get_random_degrees_index(),
                elevation: get_random_tectonic_elevation(),
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
                        x: value.site.x as f64,
                        y: value.site.y as f64,
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

pub fn add_pixels_to_continents(
    continent_points: &mut HashMap<(u16, u16), Continent>,
    site_pixels: Vec<Vec<(u16, u16)>>,
) {
    for cp in continent_points {
        let pixels = site_pixels[cp.1.index as usize].clone();
        cp.1.pixels = Some(pixels);
    }
}
