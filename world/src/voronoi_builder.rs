extern crate image;
use rand::Rng;
use voronoice::{BoundingBox, Point, Voronoi, VoronoiBuilder};
use world::models::{point::{Point16, Size16}, continent::Region};

pub fn generate_scattered_sites(img_size: &Point16, size: usize) -> Vec<Point> {
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

pub fn build_regions_and_generate_sites(grid_size: &Size16, cell_size: &Size16) -> Vec<Region> {
    let mut sites: Vec<Region> = Vec::with_capacity((grid_size.width * grid_size.height) as usize);

    for x in 0..grid_size.width {
        for y in 0..grid_size.height {
            let random_x = rand::thread_rng().gen_range(0..cell_size.width);
            let random_y = rand::thread_rng().gen_range(0..cell_size.height);
            let site_point = Point16 {
                x: ((x * cell_size.width) + random_x),
                y: ((y * cell_size.height) + random_y)
            };

            sites.push(Region::new(x, y, site_point));
        }
    }

    sites
}

pub fn build_voronoi_and_apply_site_pixels(img_size: &Size16, region_sites: &mut Vec<Region>) {
    
    let sites: Vec<Point> = region_sites.iter().map(|r| Point { x: r.site_point.x as f64, y: r.site_point.y as f64 }).collect();
    let voronoi = build(img_size, sites);

    let mut last_site_index = 0;
    // let mut ret_val: Vec<Vec<(u16, u16)>> = vec![vec![]; voronoi.cells().len()];

    for x in 0..img_size.width - 1 {
        for y in 0..img_size.height - 1 {
            let site_index = get_cell_index(&voronoi, last_site_index, x, y);
            last_site_index = site_index;

            // ret_val[site_index as usize].push((x, y));

            region_sites[site_index as usize].pixels.push((x, y));
        }
    }

    // ret_val
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
