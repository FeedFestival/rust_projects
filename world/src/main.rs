mod continent_builder;
mod image_builder;
mod voronoi_builder;

use std::{env, time::SystemTime};

use gamescript::{
    bin_read_write, file_read_write, json_read_write,
    models::{
        continent::{Planet, PlanetSettings, Realm, Region},
        point::Size16,
    },
};
use world::LIB_NAME;

fn main() {
    let time_now = std::time::SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let dir_name: Option<String> = file_read_write::dir_name(LIB_NAME);
    let dist_folder: &str = &format!("{}{}", dir_name.unwrap(), "__dist");
    println!("dist_folder: {}", dist_folder);

    let planet: Planet;

    let load_and_draw = args.contains(&String::from("load-and-draw"));
    // let load_and_draw = true;

    if load_and_draw {
        let planet: Planet =
            bin_read_write::deserialize_bin(&format!("{}\\{}", dist_folder, "planet.bin"));
        let path: &String = &format!("{}\\{}", dist_folder, "planet_settings.json");
        let planet_settings: PlanetSettings = json_read_write::deserialize_json(path);
        image_builder::build_planet_image(
            &planet,
            &planet_settings,
            &format!("{}\\{}", dist_folder, "4__continets.png"),
        );
    } else if args.len() == 1
        || args.contains(&String::from("build-and-draw"))
        || args.contains(&String::from("build"))
    {
        // (768, 384);
        let planet_settings = create_planet_settings(1536, 768, 512, 128, 64, 12);

        planet = build_planet(&planet_settings, dist_folder, time_now);

        if args.contains(&String::from("build-and-draw")) {
            image_builder::build_planet_image(
                &planet,
                &planet_settings,
                &format!("{}\\{}", dist_folder, "4__continets.png"),
            );
        }
    }
}

fn build_planet(
    planet_settings: &PlanetSettings,
    dist_folder: &str,
    time_now: SystemTime,
) -> Planet {
    println!("{:?}", planet_settings);

    // make regions

    let mut regions: Vec<Region>;

    let build_regions = true;
    if build_regions {
        let region_sites_len = ((planet_settings.region_grid_size.width / 2)
            * (planet_settings.region_grid_size.height / 2))
            as usize;
        let sites =
            voronoi_builder::generate_scattered_sites(&planet_settings.img_size, region_sites_len);
        regions = continent_builder::build_regions_and_assign_sites(sites);
        voronoi_builder::build_voronoi_and_apply_site_pixels_and_corners(
            &planet_settings.img_size,
            &mut regions,
        );

        let path = &format!("{}\\{}", dist_folder, "regions.bin");
        gamescript::bin_read_write::write(&regions, path);
    } else {
        regions = gamescript::bin_read_write::deserialize_bin(&format!(
            "{}\\{}",
            dist_folder, "regions.bin"
        ));
    }
    println!("Finished regions -> {}", get_elapsed_time(&time_now));

    image_builder::build_regions_image(
        &planet_settings.img_size,
        &regions,
        &format!("{}\\{}", dist_folder, "1__regions.png"),
    );

    // make provinces
    let mut provinces = continent_builder::build_provinces_and_generate_sites(&planet_settings);
    continent_builder::assign_regions_to_provinces(regions, &mut provinces, &planet_settings);
    println!("Finished provinces -> {}", get_elapsed_time(&time_now));
    image_builder::build_provinces_image(
        &planet_settings.img_size,
        &provinces,
        &format!("{}\\{}", dist_folder, "2__provinces.png"),
    );

    // make realms
    let mut realms: Vec<Realm> =
        continent_builder::build_realms_and_generate_sites(&planet_settings);
    continent_builder::assign_provinces_to_realms(provinces, &mut realms, &planet_settings);
    println!("Finished realms -> {}", get_elapsed_time(&time_now));
    image_builder::build_realms_image(
        &planet_settings.img_size,
        &realms,
        &format!("{}\\{}", dist_folder, "3__realms.png"),
    );

    // make continents and apply realm to them based off of distance
    let mut continents = continent_builder::build_continents_with_site(&planet_settings);
    continent_builder::assign_realms_to_continents(
        realms,
        &mut continents,
        &planet_settings,
    );
    println!(
        "Finished creating continents -> {}",
        get_elapsed_time(&time_now)
    );

    //--------

    let continents_tuple = continent_builder::merge_continents(&mut continents, &planet_settings);
    // save planet for futher use
    let mut planet = Planet {
        edge_continents: continents_tuple.0,
        continents: continents_tuple.1,
    };
    println!("Merge continents -> {}", get_elapsed_time(&time_now));

    image_builder::debug_planet_image(
        &planet,
        &planet_settings,
        &format!("{}\\{}", dist_folder, "debug__continets.png"),
        false
    );

    continent_builder::assign_continent_gradient_to_pixels(&mut planet, &planet_settings);
    println!("Finished planet -> {}", get_elapsed_time(&time_now));

    continent_builder::move_continents_pixels_towards_edge(
        &mut planet,
        &planet_settings,
    );
    println!("Move continents pixels -> {}", get_elapsed_time(&time_now));

    image_builder::debug_planet_image(
        &planet,
        &planet_settings,
        &format!("{}\\{}", dist_folder, "debug_moved__continets.png"),
        true
    );

    //-----------------

    

    let path = &format!("{}\\{}", dist_folder, "planet.bin");
    gamescript::bin_read_write::write(&planet, path);
    let path = &format!("{}\\{}", dist_folder, "planet_settings.json");
    gamescript::json_read_write::write(&planet_settings, path);

    println!("{}", get_elapsed_time(&time_now));

    planet
}

fn create_planet_settings(
    width: u16,
    height: u16,
    region_pref_width: u16,
    province_pref_width: u16, // 256
    realm_pref_width: u16,
    continent_pref_width: u16, // 16
) -> PlanetSettings {
    let final_img_size = Size16::new(width, height);
    let continent_divider = final_img_size.width / continent_pref_width;
    let final_continent_grid_size = Size16::new(
        final_img_size.width / continent_divider,
        final_img_size.height / continent_divider,
    );
    let continent_cell_size = Size16 {
        width: final_img_size.width / final_continent_grid_size.width,
        height: final_img_size.height / final_continent_grid_size.height,
    };
    //
    let continent_multiplier = 2;

    let img_size = Size16::new(
        width + (continent_cell_size.width * continent_multiplier),
        height + (continent_cell_size.height * continent_multiplier),
    );
    let continent_grid_size = Size16::new(
        final_continent_grid_size.width + continent_multiplier,
        final_continent_grid_size.height + continent_multiplier,
    );

    let region_divider = img_size.width / region_pref_width;
    let province_divider = img_size.width / province_pref_width;
    let realm_divider = img_size.width / realm_pref_width;

    let region_grid_size = Size16::new(
        img_size.width / region_divider,
        img_size.height / region_divider,
    );
    let province_grid_size = Size16::new(
        img_size.width / province_divider,
        img_size.height / province_divider,
    );
    let realm_grid_size = Size16::new(
        img_size.width / realm_divider,
        img_size.height / realm_divider,
    );
    let province_cell_size = Size16 {
        width: img_size.width / province_grid_size.width,
        height: img_size.height / province_grid_size.height,
    };
    let realm_cell_size = Size16 {
        width: img_size.width / realm_grid_size.width,
        height: img_size.height / realm_grid_size.height,
    };

    PlanetSettings {
        final_img_size: final_img_size,
        final_continent_grid_size: final_continent_grid_size,
        //
        img_size,
        region_pref_width,
        province_pref_width,
        realm_pref_width,
        continent_pref_width,
        region_grid_size,
        province_grid_size,
        realm_grid_size,
        continent_grid_size,
        province_cell_size: province_cell_size,
        realm_cell_size: realm_cell_size,
        continent_cell_size: continent_cell_size,
    }
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
