extern crate image;
use rand::Rng;
use std::{env, u8};
use voronoice::{BoundingBox, Point, Voronoi, VoronoiBuilder};

pub fn generate_sites(width: usize, height: usize, size: usize) -> Vec<Point>  {
    let mut rng = rand::thread_rng();
    let x_range = rand::distributions::Uniform::new(0, width);
    let y_range = rand::distributions::Uniform::new(0, height);

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

pub fn build(width: usize, height: usize, sites: Vec<Point>) -> Voronoi {
    let center: Point = Point {
        x: width as f64 / 2.0,
        y: height as f64 / 2.0,
    };

    let voronoi: Voronoi = VoronoiBuilder::default()
        .set_sites(sites)
        .set_clip_behavior(voronoice::ClipBehavior::None)
        // image origin is top left corner, center is width/2,height/2
        .set_bounding_box(BoundingBox::new(center, width as f64, height as f64))
        .build()
        .unwrap();

    voronoi
}
