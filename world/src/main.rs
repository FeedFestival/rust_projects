mod continent_builder;
mod image_builder;
mod voronoi_builder;

use voronoice::Point;
use world::models::point::PointU16;

fn main() {
    let img_size: PointU16 = PointU16 { x: 768, y: 384 };
    let continent_grid_size: PointU16 = PointU16 { x: 12, y: 6 };

    // make regions
    let region_sites_length = ((48 * 24) / 2) as usize;
    let region_sites = voronoi_builder::generate_sites(&img_size, region_sites_length);
    let mut region_pixels = voronoi_builder::build_and_get_site_pixels(&img_size, region_sites);
    let regions = continent_builder::build_regions(&mut region_pixels);
    image_builder::build_regions_image(&img_size, &regions, "regions.png");

    let continent_cell_size = PointU16 {
        x: img_size.x / continent_grid_size.x,
        y: img_size.y / continent_grid_size.y,
    };
    // make continents and apply region to them based off of distance
    let mut continents = continent_builder::build(continent_cell_size, &continent_grid_size);

    let sites: Vec<Point> =
        continent_builder::get_sites_from_continents(&continents, continent_grid_size);
    let site_pixels: Vec<Vec<(u16, u16)>> =
        voronoi_builder::build_and_get_site_pixels(&img_size, sites);
    continent_builder::add_pixels_to_continents(&mut continents, site_pixels);

    image_builder::build(img_size, &continents, "continets.png");
}
