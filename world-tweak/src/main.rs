mod equalize_realms;
mod seamless;

use gamescript::{
    bin_read_write, file_read_write, json_read_write, models::continent::PlanetSettings,
};
use image::Rgb;
pub const LIB_NAME: &str = "world-tweak";

fn main() {
    let dir_name: Option<String> = file_read_write::dir_name(LIB_NAME);
    let dist_folder: &str = &format!("{}{}", dir_name.unwrap(), "__dist");
    let path = &format!("{}\\{}", dist_folder, "planet.bin");
    let planet = bin_read_write::deserialize_bin(path);
    let path: &String = &format!("{}\\{}", dist_folder, "planet_settings.json");
    let planet_settings: PlanetSettings = json_read_write::deserialize_json(path);

    // seamless::make(&planet, &planet_settings);

    // return;

    let equalize_tuple = equalize_realms::equalize_light_realms(&planet, &planet_settings, 95, 42);
    let modified_pixels = &equalize_tuple.0;
    let mut img_buf = equalize_tuple.1;

    // build image
    for x in 1..planet_settings.final_continent_grid_size.width {
        for y in 1..planet_settings.final_continent_grid_size.height {
            let continent = planet.continents.get(&(x, y)).unwrap();

            for rlm in &continent.realms {
                for pv in &rlm.provinces {
                    for rg in &pv.regions {
                        for px in &rg.pixels {
                            let exists = modified_pixels.contains_key(&(px.0, px.1));
                            if exists {
                                break;
                            }
                            let pixel = img_buf.get_pixel_mut(px.0 as u32, px.1 as u32);
                            *pixel = Rgb([rg.grey_value, rg.grey_value, rg.grey_value]);
                        }
                    }
                }
            }
        }
    }

    img_buf.save("final.png").unwrap();
}
