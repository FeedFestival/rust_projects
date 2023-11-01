mod continent_builder;
mod image_builder;
mod voronoi_builder;

use std::{env, time::SystemTime};

use gamescript::models::{
    continent::{Planet, Realm},
    point::Size16,
};

const DIST_FOLDER: &str = "dist/";

fn main() {
    let time_now = std::time::SystemTime::now();
    let args: Vec<String> = env::args().collect();

    let img_size: Size16;
    let planet: Planet;

    if args.contains(&String::from("load-and-draw")) {
        planet = load_planet();
        image_builder::build_planet_image(&planet, &format!("{}{}", DIST_FOLDER, "continets.png"));
    } else if args.len() == 1
        || args.contains(&String::from("build-and-draw"))
        || args.contains(&String::from("build"))
    {
        // (768, 384);
        img_size = Size16::new(1536, 768);
        let region_pref_width = 512;
        let province_pref_width = 128; // 256
        let realm_pref_width = 64;
        let continent_pref_width = 12; // 16

        planet = build_planet(
            &img_size,
            region_pref_width,
            province_pref_width,
            realm_pref_width,
            continent_pref_width,
            time_now,
        );

        if args.contains(&String::from("build-and-draw")) {
            image_builder::build_planet_image(
                &planet,
                &format!("{}{}", DIST_FOLDER, "continets.png"),
            );
        }
    }
}

fn load_planet() -> Planet {
    let bin_path = "data.bin";
    let planet = gamescript::bin_read_write::deserialize_bin(bin_path);
    planet
}

fn build_planet(
    img_size: &Size16,
    region_pref_width: u16,
    province_pref_width: u16,
    realm_pref_width: u16,
    continent_pref_width: u16,
    time_now: SystemTime,
) -> Planet {
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
    voronoi_builder::build_voronoi_and_apply_site_pixels_and_corners(&img_size, &mut regions);
    println!("Finished regions -> {}", get_elapsed_time(&time_now));
    
    image_builder::build_regions_image(
        &img_size,
        &regions,
        &format!("{}{}", DIST_FOLDER, "regions.png"),
    );

    // make provinces
    let province_cell_size = Size16 {
        width: img_size.width / province_grid_size.width,
        height: img_size.height / province_grid_size.height,
    };
    let mut provinces = continent_builder::build_provinces_and_generate_sites(
        &province_grid_size,
        &province_cell_size,
    );
    continent_builder::assign_regions_to_provinces(
        regions,
        &mut provinces,
        &province_grid_size,
        &province_cell_size,
    );
    println!("Finished provinces -> {}", get_elapsed_time(&time_now));
    image_builder::build_provinces_image(
        &img_size,
        &provinces,
        &format!("{}{}", DIST_FOLDER, "provinces.png"),
    );

    // make realms
    let realm_cell_size = Size16 {
        width: img_size.width / realm_grid_size.width,
        height: img_size.height / realm_grid_size.height,
    };
    let mut realms: Vec<Realm> =
        continent_builder::build_realms_and_generate_sites(&realm_grid_size, &realm_cell_size);
    continent_builder::assign_provinces_to_realms(
        provinces,
        &mut realms,
        &realm_grid_size,
        &realm_cell_size,
    );
    println!("Finished realms -> {}", get_elapsed_time(&time_now));
    image_builder::build_realms_image(
        &img_size,
        &realms,
        &format!("{}{}", DIST_FOLDER, "realms.png"),
    );

    // make continents and apply realm to them based off of distance
    let continent_cell_size = Size16 {
        width: img_size.width / continent_grid_size.width,
        height: img_size.height / continent_grid_size.height,
    };
    let mut continents =
        continent_builder::build_continents_with_site(&continent_cell_size, &continent_grid_size);
    // assign realms to continents
    continent_builder::assign_realms_to_continents_and_calculate_region_color(
        realms,
        &mut continents,
        &continent_grid_size,
        continent_cell_size,
    );
    println!("Finished continents -> {}", get_elapsed_time(&time_now));

    let planet = Planet {
        img_size: Size16 {
            width: img_size.width,
            height: img_size.height,
        },
        grid_size: continent_grid_size,
        continents,
    };
    let bin_path = "data.bin";
    gamescript::bin_read_write::write(&planet, bin_path);

    println!("{}", get_elapsed_time(&time_now));

    planet
}

fn get_elapsed_time(time_now: &SystemTime) -> String {
    match time_now.elapsed() {
        Ok(ellapsed) => {
            return format!(
                "s: {}, ms: {}, ns: {}",
                ellapsed.as_secs(),
                ellapsed.as_millis(),
                ellapsed.as_nanos()
            )
        }
        Err(_) => return "".to_string(),
    }
}
