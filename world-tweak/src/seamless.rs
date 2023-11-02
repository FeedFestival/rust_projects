use gamescript::models::continent::{Continent, Planet, PlanetSettings};
use image::{ImageBuffer, Rgb};

pub fn make(planet: &Planet, planet_settings: &PlanetSettings) {
    let mut img_buf: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::new(planet.img_size.width as u32, planet.img_size.height as u32);

    println!("{:?}", planet_settings.continent_grid_size);
    println!("{:?}", planet_settings.continent_cell_size);

    let mut muvable_left: Vec<&Continent> = Vec::new();

    for x in 0..planet_settings.continent_grid_size.width {
        for y in 0..planet_settings.continent_grid_size.height {
            let is_margin = x == 0
                || y == 0
                || x == planet_settings.continent_grid_size.width - 1
                || y == planet_settings.continent_grid_size.height - 1;
            if is_margin {
                continue;
            }

            let continent = planet.continents.get(&(x, y)).unwrap();

            let is_movable_left =
                x == 1 && y >= 1 && y <= (planet_settings.continent_grid_size.height - 2);

            if is_movable_left {
                muvable_left.push(continent);
                // continue;
            }

            for rlm in &continent.realms {
                for pv in &rlm.provinces {
                    for rg in &pv.regions {
                        let color = if is_movable_left == true {
                            Rgb([204, 255, 102])
                        } else {
                            Rgb([rg.grey_value, rg.grey_value, rg.grey_value])
                        };

                        for px in &rg.pixels {
                            let pixel = img_buf.get_pixel_mut(px.0 as u32, px.1 as u32);
                            *pixel = color;
                        }
                    }
                }
            }
        }
    }

    img_buf.save("centered.png").unwrap();

    let mut i = 0;
    for x in 0..planet_settings.continent_grid_size.width {
        for y in 0..planet_settings.continent_grid_size.height {
            let is_margin = x == 0
                || y == 0
                || x == planet_settings.continent_grid_size.width - 1
                || y == planet_settings.continent_grid_size.height - 1;
            if is_margin {
                continue;
            }

            let mut continent = planet.continents.get(&(x, y)).unwrap();

            let is_replaceable_right = x == (planet_settings.continent_grid_size.width - 2)
                && y >= 1
                && y <= (planet_settings.continent_grid_size.height - 2);
            let mut pixel_distance: (u32, u32) = (0, 0);

            if is_replaceable_right {
                continent = muvable_left.get(i).unwrap();
                // muvable_left.push(continent);

                pixel_distance = (9 * planet_settings.continent_cell_size.width as u32, 0);

                i += 1;
                // continue;
            }

            for rlm in &continent.realms {
                for pv in &rlm.provinces {
                    for rg in &pv.regions {
                        let color = if is_replaceable_right == true {
                            Rgb([204, 255, 102])
                        } else {
                            Rgb([rg.grey_value, rg.grey_value, rg.grey_value])
                        };

                        for px in &rg.pixels {
                            let pixel = img_buf.get_pixel_mut(
                                px.0 as u32 + pixel_distance.0,
                                px.1 as u32 + pixel_distance.1,
                            );
                            *pixel = color;
                        }
                    }
                }
            }
        }
    }

    img_buf.save("final_centered.png").unwrap();
}
