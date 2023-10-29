mod continent_builder;
mod image_builder;
mod voronoi_builder;

use voronoice::Point;
use world::models::point::PointU16;

fn main() {
    let img_size: PointU16 = PointU16 { x: 768, y: 384 };
    let continent_grid_size: PointU16 = PointU16 { x: 12, y: 6 };

    let mut continents = continent_builder::build(&img_size, &continent_grid_size);

    let sites: Vec<Point> =
        continent_builder::get_sites_from_continents(&continents, continent_grid_size);
    let site_pixels: Vec<Vec<(u16, u16)>> =
        voronoi_builder::build_and_get_site_pixels(&img_size, sites);
    continent_builder::add_pixels_to_continents(&mut continents, site_pixels);

    image_builder::build(img_size, &continents, "continets.png");

    // let pointsLength = imgx / 6;
    // let sites = build_voronoi::generate_sites(imgx, imgy, pointsLength);
}
