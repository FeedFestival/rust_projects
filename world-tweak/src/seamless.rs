use gamescript::models::continent::Planet;
use image::{ImageBuffer, Rgb};

pub fn make(planet: &Planet) {
    let mut img_buf: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::new(planet.img_size.width as u32, planet.img_size.height as u32);

    for x in 0..planet.grid_size.width {
        for y in 0..planet.grid_size.height {
            let is_margin = x == 0
                || y == 0
                || x == planet.grid_size.width - 1
                || y == planet.grid_size.height - 1;
            if is_margin {
                continue;
            }
            // let is_centered =
            //     x > 2 && y > 2 && x < planet.grid_size.width - 2 && y < planet.grid_size.height - 2;
            // if is_centered == false {
            //     continue;
            // }

            let continent = planet.continents.get(&(x, y)).unwrap();

            for rlm in &continent.realms {
                for pv in &rlm.provinces {
                    for rg in &pv.regions {
                        for px in &rg.pixels {
                            // let exists = modified_pixels.contains_key(&(px.0, px.1));
                            // if exists {
                            //     break;
                            // }
                            let pixel = img_buf.get_pixel_mut(px.0 as u32, px.1 as u32);
                            *pixel = Rgb([rg.grey_value, rg.grey_value, rg.grey_value]);
                        }
                    }
                }
            }
        }
    }

    img_buf.save("centered.png").unwrap();
}
