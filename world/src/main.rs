mod continent_builder;
mod image_builder;
mod voronoi_builder;

use world::models::point::PointU16;

fn main() {
    let img_size: PointU16 = PointU16 { x: 768, y: 384 };

    // make provinces
    let province_grid_size: PointU16 = PointU16 { x: 192, y: 96 };

    // make regions
    let region_grid_size: PointU16 = PointU16 { x: 48, y: 24 };
    let region_cell_size = PointU16 {
        x: img_size.x / region_grid_size.x,
        y: img_size.y / region_grid_size.y,
    };
    let mut region_sites = voronoi_builder::generate_sites_by_cell_size(&region_grid_size, &region_cell_size);
    voronoi_builder::build_voronoi_and_apply_site_pixels(&img_size, &mut region_sites);
    let regions = continent_builder::build_regions(&mut region_sites);
    image_builder::build_regions_image(&img_size, &regions, "regions.png");

    // make continents and apply region to them based off of distance
    let continent_grid_size: PointU16 = PointU16 { x: 12, y: 6 };
    let continent_cell_size = PointU16 {
        x: img_size.x / continent_grid_size.x,
        y: img_size.y / continent_grid_size.y,
    };
    let mut continents
        = continent_builder::build_continents_with_site(&continent_cell_size, &continent_grid_size);

    // assign regions to continents
    continent_builder::assign_regions_to_continents(regions, &mut continents, continent_grid_size, continent_cell_size);

    image_builder::build(img_size, &continents, "continets.png");
}
