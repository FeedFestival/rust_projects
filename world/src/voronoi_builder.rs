extern crate image;
use rand::Rng;
use voronoice::{BoundingBox, Point, Voronoi, VoronoiBuilder};
use world::models::{continent::Region, point::Size16};

pub fn generate_scattered_sites(img_size: &Size16, len: usize) -> Vec<Point> {
    let mut rng = rand::thread_rng();
    let x_range = rand::distributions::Uniform::new(0, img_size.width);
    let y_range = rand::distributions::Uniform::new(0, img_size.height);

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

pub fn build_voronoi_and_apply_site_pixels(img_size: &Size16, regions: &mut Vec<Region>) {
    let sites: Vec<Point> = regions
        .iter()
        .map(|r| Point {
            x: r.site_point.x as f64,
            y: r.site_point.y as f64,
        })
        .collect();
    let voronoi = build(img_size, sites);

    let mut last_site_index = 0;

    for x in 0..img_size.width - 1 {
        for y in 0..img_size.height - 1 {
            let site_index = get_cell_index(&voronoi, last_site_index, x, y);
            last_site_index = site_index;
            regions[site_index as usize].pixels.push((x, y));
        }
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
