mod equalize_realms;
mod seamless;

use gamescript::models::continent::Planet;
use image::Rgb;
use std::env;

fn main() {
    let planet = load_planet();

    println!("{:?}", planet.grid_size);

    seamless::make(&planet);

    let equalize_tuple = equalize_realms::equalize_light_realms(&planet, 95, 42);
    let modified_pixels = &equalize_tuple.0;
    let mut img_buf = equalize_tuple.1;

    for x in 0..planet.grid_size.width {
        for y in 0..planet.grid_size.height {
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

fn load_planet() -> Planet {
    let mut project_dir = env::current_dir().unwrap().to_string_lossy().to_string();
    project_dir = project_dir.replace("world-tweak", "world");
    let bin_path = format!("{}//{}", project_dir, "data.bin");
    println!("path: {}", bin_path);
    let planet = gamescript::bin_read_write::deserialize_bin(bin_path.as_str());
    planet
}
