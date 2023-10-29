mod continent_builder;
mod image_builder;
mod voronoi_builder;

use world::models::point::Size16;

fn main() {
    // (768, 384);
    let img_size: Size16 = Size16::new(1536, 768);
    let region_pref_width = 512;
    let province_pref_width = 128;  // 256
    let realm_pref_width = 64;
    let continent_pref_width = 16;  // 16

    let region_divider = img_size.width / region_pref_width;
    let province_divider = img_size.width / province_pref_width;
    let realm_divider = img_size.width / realm_pref_width;
    let continent_divider = img_size.width / continent_pref_width;

    let region_grid_size: Size16 = Size16::new(
        img_size.width / region_divider,
        img_size.height / region_divider,
    );
    let province_grid_size: Size16 = Size16::new(
        img_size.width / province_divider,
        img_size.height / province_divider,
    );
    let realm_grid_size = Size16::new(
        img_size.width / realm_divider,
        img_size.height / realm_divider,
    );
    let continent_grid_size = Size16::new(
        img_size.width / continent_divider,
        img_size.height / continent_divider,
    );

    println!("Making map size ({}, {}); region size ({}, {}); province size ({}, {}); realm size ({}, {}); continent size ({}, {})"
        , img_size.width
        , img_size.height
        , region_grid_size.width
        , region_grid_size.height
        , province_grid_size.width
        , province_grid_size.height
        , realm_grid_size.width
        , realm_grid_size.height
        , continent_grid_size.width
        , continent_grid_size.height
    );




    // make regions (768, 384)
    let region_sites_len = ((region_grid_size.width / 2) * (region_grid_size.height / 2)) as usize;
    let sites = voronoi_builder::generate_scattered_sites(&img_size, region_sites_len);
    let mut regions = continent_builder::build_regions_and_assign_sites(sites);
    voronoi_builder::build_voronoi_and_apply_site_pixels(&img_size, &mut regions);
    image_builder::build_regions_image(&img_size, &regions, "regions.png");





    // make provinces
    let province_cell_size = Size16 {
        width: img_size.width / province_grid_size.width,
        height: img_size.height / province_grid_size.height,
    };
    let mut provinces =
        continent_builder::build_provinces_and_generate_sites(&province_grid_size, &province_cell_size);
    continent_builder::assign_regions_to_provinces(
        regions,
        &mut provinces,
        &province_grid_size,
        &province_cell_size,
    );
    image_builder::build_provinces_image(&img_size, &provinces, "provinces.png");

    // make realms
    let realm_cell_size = Size16 {
        width: img_size.width / realm_grid_size.width,
        height: img_size.height / realm_grid_size.height,
    };
    let mut realms: Vec<world::models::continent::Realm> =
        continent_builder::build_realms_and_generate_sites(&realm_grid_size, &realm_cell_size);
    continent_builder::assign_provinces_to_realms(
        provinces,
        &mut realms,
        &realm_grid_size,
        &realm_cell_size,
    );
    image_builder::build_realms_image(&img_size, &realms, "realms.png");

    // make continents and apply realm to them based off of distance
    let continent_cell_size = Size16 {
        width: img_size.width / continent_grid_size.width,
        height: img_size.height / continent_grid_size.height,
    };
    let mut continents =
        continent_builder::build_continents_with_site(&continent_cell_size, &continent_grid_size);

    // assign realms to continents
    continent_builder::assign_realms_to_continents(
        realms,
        &mut continents,
        continent_grid_size,
        continent_cell_size,
    );
    image_builder::build(img_size, &continents, "continets.png");
}
