use std::{sync::Mutex, collections::HashMap, env};

use gamescript::{file_read_write, models::{point::Size16, continent::PlanetSettings}};
use once_cell::sync::OnceCell;
use world::LIB_NAME;

#[derive(PartialEq, Eq, Hash)]
pub enum ArgName {
    EMPTY, BUILD, LoadRegions, LOAD, DRAW
}

// #[derive(PartialEq, Eq, Hash)]
pub struct ContinentBuilderSettings {
    pub img_size: Size16,
    pub region_grid_size: Size16,
    pub province_grid_size: Size16,
    pub realm_grid_size: Size16,
    pub continent_grid_size: Size16,
    pub province_cell_size: Size16,
    pub realm_cell_size: Size16
}

pub struct GlobalCache {
    pub args: Mutex<HashMap<ArgName, bool>>,
    pub dist_folder: String,
    pub builder_settings: ContinentBuilderSettings
}

pub static GLOBAL_CACHE: OnceCell<GlobalCache> = OnceCell::new();

pub fn initialize_cache(
    planet_settings: &PlanetSettings,
    region_pref_width: u16,
    province_pref_width: u16, // 256
    realm_pref_width: u16,
) {

    let args: Vec<String> = env::args().collect();
    let dir_name: Option<String> = file_read_write::dir_name(LIB_NAME);

    // println!("\n");
    // for arg in &args {
    //     println!("arg: {:#?}", arg);
    // }
    // println!("\n");

    let mut cached_args: HashMap<ArgName, bool> = HashMap::new();
    cached_args.insert(ArgName::EMPTY, args.len() == 1);
    cached_args.insert(ArgName::BUILD, args.contains(&String::from("build")));
    cached_args.insert(ArgName::LoadRegions, args.contains(&String::from("load-rg")));
    cached_args.insert(ArgName::LOAD, args.contains(&String::from("load")));
    cached_args.insert(ArgName::DRAW, args.contains(&String::from("draw")));

    // Builder Settings
    let continent_multiplier = 2;

    let img_size = Size16::new(
        planet_settings.final_img_size.width + (planet_settings.continent_cell_size.width * continent_multiplier),
        planet_settings.final_img_size.height + (planet_settings.continent_cell_size.height * continent_multiplier),
    );
    let continent_grid_size = Size16::new(
        planet_settings.final_continent_grid_size.width + continent_multiplier,
        planet_settings.final_continent_grid_size.height + continent_multiplier,
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

    let builder_settings = ContinentBuilderSettings {
        img_size,
        region_grid_size,
        province_grid_size,
        realm_grid_size,
        continent_grid_size,
        province_cell_size,
        realm_cell_size,
    };

    let result = GLOBAL_CACHE.set(GlobalCache {
        args: Mutex::new(cached_args),
        dist_folder: format!("{}{}", dir_name.unwrap(), "__dist"),
        builder_settings: builder_settings
    });
    match result {
        Ok(_) => println!("GLOBAL_CACHE Stored"),
        Err(_) => println!("Couldn't store in GLOBAL_CACHE"),
    }
}

pub fn cache_contains_args(arg: &ArgName) -> bool {
    *GLOBAL_CACHE.get().unwrap().args.lock().unwrap().get(&arg).unwrap()
}

pub fn dist_folder(file_name: &str) -> String {
    println!("dist_folder: {}", file_name);
    format!("{}\\{}", GLOBAL_CACHE.get().unwrap().dist_folder, file_name)
}

pub fn builder_settings() -> &'static ContinentBuilderSettings {
    &GLOBAL_CACHE.get().unwrap().builder_settings
}
