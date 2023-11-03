mod cache;
mod continent_builder;
mod image_builder;
mod voronoi_builder;

use crate::cache::{initialize_cache, ArgName};
use cache::{cache_contains_args, dist_folder};
use gamescript::{
    bin_read_write,
    models::{
        continent::{Continent, Planet, PlanetSettings, Province, Realm, Region},
        point::Size16,
    },
};
use std::{collections::HashMap, time::SystemTime};

/*------------------------------------------------- FUNCTION_NAME -----
|  Function FUNCTION_NAME
|
|  Purpose:  This builds the planet.
|
|  Args:
|   - build (just builds the planet, it's the same as not writing anything)
|   - load (load the previously generated planet)
|   - draw (loads the generated planet and draws the images)
|   - build draw (builds the continents and draws the full process)
|   - build load-rg draw (builds the planet, load regions and draws the full process)
|   - build load draw (builds the planet, load the generated continents and draws the full process)
|   -
|
|
*-------------------------------------------------------------------*/
fn main() {
    let time_now = std::time::SystemTime::now();

    // (768, 384);
    let planet_settings: PlanetSettings = create_planet_settings(1536, 768, 12);
    initialize_cache(&planet_settings, 512, 128, 64);

    if cache_contains_args(&ArgName::EMPTY) || cache_contains_args(&ArgName::BUILD) {
        build_planet(&planet_settings, time_now);
    }

    if cache_contains_args(&ArgName::DRAW) {
        let planet: Planet = bin_read_write::deserialize_bin(dist_folder("planet.bin").as_str());
        image_builder::build_planet_image(
            &planet,
            &planet_settings,
            dist_folder("4__continets.png").as_str(),
        );
    }
}

fn build_planet(planet_settings: &PlanetSettings, time_now: SystemTime) {
    println!("{:?}", planet_settings);

    let load_regions = cache_contains_args(&ArgName::LoadRegions);
    let load: bool = cache_contains_args(&ArgName::LOAD);
    let mut continents: HashMap<(u16, u16), Continent>;

    if load == false {
        // make regions
        let mut regions: Vec<Region>;
        if load_regions {
            regions = gamescript::bin_read_write::deserialize_bin(&dist_folder("regions.bin"));
        } else {
            let sites = voronoi_builder::generate_scattered_sites();
            regions = voronoi_builder::build_regions_and_assign_sites(sites);
            voronoi_builder::build_voronoi_and_apply_site_pixels_and_corners(&mut regions);

            gamescript::bin_read_write::write(&regions, &dist_folder("regions.bin"));
        }
        println!("1. Regions at {}", get_elapsed_time(&time_now));

        if cache_contains_args(&ArgName::DRAW) {
            image_builder::build_regions_image(&regions, &dist_folder("1__regions.png"));
        }

        // make provinces
        let mut provinces: Vec<Province> = voronoi_builder::build_provinces_and_generate_sites();
        voronoi_builder::assign_regions_to_provinces(regions, &mut provinces);

        println!("2. Provinces at {}", get_elapsed_time(&time_now));

        if cache_contains_args(&ArgName::DRAW) {
            image_builder::build_provinces_image(&provinces, &dist_folder("2__provinces.png"));
        }

        // make realms
        let mut realms: Vec<Realm> =
            voronoi_builder::build_realms_and_generate_sites(&planet_settings);
        voronoi_builder::assign_provinces_to_realms(provinces, &mut realms);

        println!("3. Realms at {}", get_elapsed_time(&time_now));

        if cache_contains_args(&ArgName::DRAW) {
            image_builder::build_realms_image(&realms, &dist_folder("3__realms.png"));
        }

        // make continents and apply realm to them based off of distance
        continents = voronoi_builder::build_continents_with_site(&planet_settings);
        voronoi_builder::assign_realms_to_continents(realms, &mut continents, &planet_settings);
        gamescript::bin_read_write::write(&continents, &dist_folder("continents.bin"));

        println!("4. Continents at {}", get_elapsed_time(&time_now));
    } else {
        continents = gamescript::bin_read_write::deserialize_bin(&dist_folder("continents.bin"));
    }

    let continents_tuple = continent_builder::merge_continents(&mut continents);

    // save planet for futher use
    let mut planet = Planet {
        edge_continents: continents_tuple.0,
        continents: continents_tuple.1,
    };
    println!("Merge continents -> {}", get_elapsed_time(&time_now));

    if cache_contains_args(&ArgName::DRAW) {
        image_builder::debug_planet_image(
            &planet,
            &planet_settings,
            &dist_folder("debug__continets.png"),
            false,
        );
    }

    continent_builder::assign_continent_gradient_to_pixels(&mut planet, &planet_settings);
    println!("Finished planet -> {}", get_elapsed_time(&time_now));

    continent_builder::move_continents_pixels_towards_edge(&mut planet, &planet_settings);
    println!("Move continents pixels -> {}", get_elapsed_time(&time_now));

    if cache_contains_args(&ArgName::DRAW) {
        image_builder::debug_planet_image(
            &planet,
            &planet_settings,
            &dist_folder("debug_moved__continets.png"),
            true,
        );
    }

    gamescript::bin_read_write::write(&planet, &dist_folder("planet.bin"));
    gamescript::json_read_write::write(&planet_settings, &dist_folder("planet_settings.json"));

    println!("{}", get_elapsed_time(&time_now));
}

fn create_planet_settings(
    width: u16,
    height: u16,
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

    PlanetSettings {
        final_img_size: final_img_size,
        final_continent_grid_size,
        continent_cell_size
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
