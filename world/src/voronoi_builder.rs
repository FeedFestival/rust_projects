extern crate image;
use rand::Rng;
use voronoice::{BoundingBox, Point, Voronoi, VoronoiBuilder};
use world::models::{point::PointU16, continent::RegionSite};

pub fn generate_scattered_sites(img_size: &PointU16, size: usize) -> Vec<Point> {
    let mut rng = rand::thread_rng();
    let x_range = rand::distributions::Uniform::new(0, img_size.x);
    let y_range = rand::distributions::Uniform::new(0, img_size.y);

    let mut sites: Vec<Point> = Vec::with_capacity(size); // Use a Vec to store the sites

    while sites.len() < size {
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

pub fn generate_sites_by_cell_size(grid_size: &PointU16, cell_size: &PointU16) -> Vec<RegionSite> {
    let mut sites: Vec<RegionSite> = Vec::with_capacity((grid_size.x * grid_size.y) as usize);

    for x in 0..grid_size.x {
        for y in 0..grid_size.y {
            let random_x = rand::thread_rng().gen_range(0..cell_size.x);
            let random_y = rand::thread_rng().gen_range(0..cell_size.y);
            let point = Point {
                x: ((x * cell_size.x) + random_x) as f64,
                y: ((y * cell_size.y) + random_y) as f64,
            };

            sites.push(RegionSite::new(x, y, point));
        }
    }

    sites
}

pub fn build_voronoi_and_apply_site_pixels(img_size: &PointU16, region_sites: &mut Vec<RegionSite>) {
    
    let sites: Vec<Point> = region_sites.iter().map(|r| r.site_point.clone()).collect();
    let voronoi = build(img_size, sites);

    let mut last_site_index = 0;
    // let mut ret_val: Vec<Vec<(u16, u16)>> = vec![vec![]; voronoi.cells().len()];

    for x in 0..img_size.x - 1 {
        for y in 0..img_size.y - 1 {
            let site_index = get_cell_index(&voronoi, last_site_index, x, y);
            last_site_index = site_index;

            // ret_val[site_index as usize].push((x, y));

            region_sites[site_index as usize].pixels.push((x, y));
        }
    }

    // ret_val
}

fn build(img_size: &PointU16, sites: Vec<Point>) -> Voronoi {
    let center: Point = Point {
        x: img_size.x as f64 / 2.0,
        y: img_size.y as f64 / 2.0,
    };

    let voronoi: Voronoi = VoronoiBuilder::default()
        .set_sites(sites)
        .set_clip_behavior(voronoice::ClipBehavior::None)
        // image origin is top left corner, center is width/2,height/2
        .set_bounding_box(BoundingBox::new(
            center,
            img_size.x as f64,
            img_size.y as f64,
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
