use std::collections::HashMap;

use gamescript::models::continent::Continent;


pub fn remove_pixels_from_continent(continent: &mut Continent, pixels: &mut HashMap<(u16, u16), bool>, insert: bool) {
    println!("\n\n\n START extract_pixels_and_clean_extra_pixels \n");
    let mut to_remove_realms: Vec<u16> = Vec::new();
    let mut rlm_i = 0;

    for rlm in &mut continent.realms {
        let mut to_remove_provinces: Vec<u16> = Vec::new();
        let mut pv_i = 0;

        for pv in &mut rlm.provinces {
            let mut to_remove_regions: Vec<u16> = Vec::new();
            let mut r_i = 0;

            for rg in &mut pv.regions {

                let mut to_remove_pixels: Vec<u16> = Vec::new();
                let mut i: u16 = 0;

                for px in &mut rg.pixels {
                    let exists = pixels.contains_key(px);
                    if exists {
                        to_remove_pixels.push(i);
                    } else {
                        if insert {
                            pixels.insert(*px, false);
                        }
                    }

                    i += 1;
                }

                let delete_region = &to_remove_pixels.len() == &rg.pixels.len();
                if delete_region {
                    to_remove_regions.push(r_i);
                } else if to_remove_pixels.len() > 0 {
                    println!("--- removing {} pixels", to_remove_pixels.len());
                    to_remove_pixels.sort();
                    let px_length = to_remove_pixels.len();
                    for i in (0..px_length).rev() {
                        rg.pixels.remove(to_remove_pixels[i] as usize);
                    }
                }

                r_i += 1;
            }

            let delete_provinces = &to_remove_regions.len() == &pv.regions.len();
            if delete_provinces {
                to_remove_provinces.push(pv_i);
            } else if to_remove_regions.len() > 0 {
                println!("-- removing {} regions", to_remove_regions.len());
                to_remove_regions.sort();
                let px_length = to_remove_regions.len();
                for i in (0..px_length).rev() {
                    pv.regions.remove(to_remove_regions[i] as usize);
                }
            }

            pv_i += 1;
        }

        let delete_realms = &to_remove_provinces.len() == &rlm.provinces.len();
        if delete_realms {
            to_remove_realms.push(rlm_i);
        } else if to_remove_provinces.len() > 0 {
            println!("- removing {} provinces", to_remove_provinces.len());
            to_remove_provinces.sort();
            let px_length = to_remove_provinces.len();
            for i in (0..px_length).rev() {
                rlm.provinces.remove(to_remove_provinces[i] as usize);
            }
        }

        rlm_i += 1;
    }

    to_remove_realms.sort();
    remove_extra_realms(continent, to_remove_realms);

    println!("\n FINISHED extract_pixels_and_clean_extra_pixels \n\n\n");
}

pub fn remove_extra_realms(continent: &mut Continent, to_remove_realms: Vec<u16>) {
    let length = to_remove_realms.len();
    for i in (0..length).rev() {
        continent.realms.remove(to_remove_realms[i] as usize);
    }
    // println!("- removed {} realms from 00 after merge", to_remove_realms.len());
}
