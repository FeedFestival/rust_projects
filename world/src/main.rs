mod build_voronoi;
mod build_image;

use voronoice::{Voronoi};

fn main() {
    let imgx = 768;
    let imgy = 384;
    let pointsLength = imgx / 6;
    let sites = build_voronoi::generate_sites(imgx, imgy, pointsLength);
    let voronoi: Voronoi = build_voronoi::build(imgx, imgy, sites);
    build_image::make_image(imgx as u32, imgy as u32, voronoi, "continets.png");

    let imgx = 768;
    let imgy = 384;
    let pointsLength = imgx / 2;
    let sites = build_voronoi::generate_sites(imgx, imgy, pointsLength);
    let voronoi: Voronoi = build_voronoi::build(imgx, imgy, sites);
    build_image::make_image(imgx as u32, imgy as u32, voronoi, "countries.png");
}

