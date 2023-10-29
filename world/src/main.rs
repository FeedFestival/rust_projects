mod continent_builder;
mod image_builder;
mod voronoi_builder;

use world::models::point::Size16;

fn main() {
    // let img_size: Size16 = Size16::new(768, 384);
    let img_size: Size16 = Size16::new(1536, 768);

    // make provinces (256, 128) (192, 96)
    let province_grid_size: Size16 = Size16::new(img_size.width / 4, img_size.height / 4);
    println!("province_grid_size: {}, {}", province_grid_size.width, province_grid_size.height);
    let province_sites_len = ((province_grid_size.width / 2) * (province_grid_size.height / 2)) as usize;
    let sites = voronoi_builder::generate_scattered_sites(&img_size, province_sites_len);
    let mut provinces = continent_builder::build_provinces_and_assign_sites(sites);
    voronoi_builder::build_voronoi_and_apply_site_pixels(&img_size, &mut provinces);
    image_builder::build_provinces_image(&img_size, &provinces, "provinces.png");

    // make regions (64, 32)
    let region_grid_size = Size16::new(province_grid_size.width / 4, province_grid_size.height / 4);
    let region_cell_size = Size16 {
        width: img_size.width / region_grid_size.width,
        height: img_size.height / region_grid_size.height,
    };
    let mut regions: Vec<world::models::continent::Region> =
        continent_builder::build_regions_and_generate_sites(&region_grid_size, &region_cell_size);
    continent_builder::assign_provinces_to_regions(
        provinces,
        &mut regions,
        &region_grid_size,
        &region_cell_size,
    );
    image_builder::build_regions_image(&img_size, &regions, "regions.png");

    // make continents and apply region to them based off of distance (16, 8)
    let continent_grid_size = Size16::new(region_grid_size.width / 6, region_grid_size.height / 6);
    let continent_cell_size = Size16 {
        width: img_size.width / continent_grid_size.width,
        height: img_size.height / continent_grid_size.height,
    };
    let mut continents =
        continent_builder::build_continents_with_site(&continent_cell_size, &continent_grid_size);

    // assign regions to continents
    continent_builder::assign_regions_to_continents(
        regions,
        &mut continents,
        continent_grid_size,
        continent_cell_size,
    );
    image_builder::build(img_size, &continents, "continets.png");
}
