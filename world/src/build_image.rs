use rand::Rng;
use std::collections::HashMap;
use voronoice::{Point, Voronoi};

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

pub fn make_image(imgx: u32, imgy: u32, voronoi: Voronoi, image_name: &str) {
    let mut imgbuf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        image::ImageBuffer::new(imgx, imgy);
    let mut voronoi_colors: HashMap<usize, Color> = HashMap::new();
    let mut index = 0;
    for cell in voronoi.iter_cells() {
        let mut rng = rand::thread_rng();
        let color = Color {
            r: rng.gen_range(0..=255),
            g: rng.gen_range(0..=255),
            b: rng.gen_range(0..=255),
        };
        voronoi_colors.insert(index, color);

        index = index + 1;
    }

    let grey_value = 255 / 2;
    let grey_color: Color = Color {
        r: grey_value,
        g: grey_value,
        b: grey_value,
    };
    let white_color: Color = Color {
        r: 255,
        g: 255,
        b: 255,
    };

    let width = imgx as usize;
    let height = imgy as usize;

    let mut last_site = 0;

    for x in 0..width - 1 {
        for y in 0..height - 1 {
            let x = x as u32;
            let y = y as u32;

            // get site/voronoi cell for which pixel belongs to
            let site = get_cell(&voronoi, last_site, x, y);
            last_site = site;

            // accumulate color per cell
            let pixel = imgbuf.get_pixel_mut(x, y);

            let color: &Color;
            let exists = voronoi_colors.contains_key(&site);
            if exists == true {
                match voronoi_colors.get(&site) {
                    Some(c) => {
                        color = c;
                    }
                    None => color = &white_color,
                };
            } else {
                color = &grey_color;
            }

            *pixel = image::Rgb([color.r, color.g, color.b]);
        }
    }
    imgbuf.save(image_name).unwrap();
}

fn get_cell(voronoi: &Voronoi, current_site: usize, x: u32, y: u32) -> usize {
    let p = Point {
        x: x as f64,
        y: y as f64,
    };
    voronoi
        .cell(current_site)
        .iter_path(p)
        .last()
        .expect("Expected to find site that contains point")
}
