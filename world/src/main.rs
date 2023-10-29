mod continent_builder;
mod image_builder;
mod voronoi_builder;

use world::models::point::{Point16, Size16};

fn main() {
    let img_size: Size16 = Size16::new(768, 384);

    // make provinces
    let province_grid_size: Size16 = Size16::new(192, 96);

    // make regions
    let region_grid_size = Size16::new(48, 24);
    let region_cell_size = Size16 {
        width: img_size.width / region_grid_size.width,
        height: img_size.height / region_grid_size.height
    };
    let mut regions = voronoi_builder::build_regions_and_generate_sites(&region_grid_size, &region_cell_size);
    voronoi_builder::build_voronoi_and_apply_site_pixels(&img_size, &mut regions);
    image_builder::build_regions_image(&img_size, &regions, "regions.png");

    // make continents and apply region to them based off of distance
    let continent_grid_size = Size16::new(12, 6);
    let continent_cell_size = Size16 {
        width: img_size.width / continent_grid_size.width,
        height: img_size.height / continent_grid_size.height,
    };
    let mut continents
        = continent_builder::build_continents_with_site(&continent_cell_size, &continent_grid_size);

    // assign regions to continents
    continent_builder::assign_regions_to_continents(regions, &mut continents, continent_grid_size, continent_cell_size);

    image_builder::build(img_size, &continents, "continets.png");
}
