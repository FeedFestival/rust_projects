use gamescript::models::{
    continent::Continent,
    point::{
        center_of_two_points,
        try_map_points_min_max_points_by_points, Point16
    },
};
use std::collections::HashMap;

use crate::cache::builder_settings;

pub fn merge_continents(
    continents: &mut HashMap<(u16, u16), Continent>,
) -> (
    HashMap<(u16, u16), Continent>,
    HashMap<(u16, u16), Continent>,
) {
    let settings = builder_settings();
    let mut edge_continents: HashMap<(u16, u16), Continent> = HashMap::new();
    let mut new_continents: HashMap<(u16, u16), Continent> = HashMap::new();

    // println!("{:?}", continents.len());

    // remove first column
    // println!(" \n- START remove first column -> \n \n ");
    for y in 0..settings.continent_grid_size.height {
        let x = 0;
        continents.remove(&(x, y));
    }
    // println!(" \n \nFINISHED continents.len(): {:?}", continents.len());

    // remove first row
    // println!(" \n- START remove first row -> \n \n ");
    for x in 1..settings.continent_grid_size.width {
        let y = 0;
        continents.remove(&(x, y));
    }
    // println!(" \n \nFINISHED continents.len(): {:?}", continents.len());

    // create the 4 cubed
    // println!(" \n- START create the 4 sized -> \n \n ");
    let insert_at = (0, 0);
    let moved_coord = &(2, 2);
    let mut continent = continents.remove(&moved_coord).unwrap();
    let top_c = continents.remove(&(2, 1)).unwrap();
    try_map_points_min_max_points_by_points(
        &mut continent.bottom_left,
        &mut continent.top_right,
        &top_c.bottom_left,
        &top_c.top_right,
    );
    for realm in top_c.realms {
        continent.realms.push(realm);
    }
    let left_c = continents.remove(&(1, 2)).unwrap();
    try_map_points_min_max_points_by_points(
        &mut continent.bottom_left,
        &mut continent.top_right,
        &left_c.bottom_left,
        &left_c.top_right,
    );
    for realm in left_c.realms {
        continent.realms.push(realm);
    }
    let top_left_c = continents.remove(&(1, 1)).unwrap();
    try_map_points_min_max_points_by_points(
        &mut continent.bottom_left,
        &mut continent.top_right,
        &top_left_c.bottom_left,
        &top_left_c.top_right,
    );
    for realm in top_left_c.realms {
        continent.realms.push(realm);
    }

    continent.grid_coord = Point16::new(insert_at.0, insert_at.1);
    continent.site_point = center_of_two_points(&continent.site_point, &top_left_c.site_point);

    // println!("into: {:?} <- INSERT {:?}", insert_at, moved_coord);
    // println!("into: {:?} <- INSERT {:?} + realms", insert_at, (2, 1));
    // println!("into: {:?} <- INSERT {:?} + realms", insert_at, (1, 2));
    // println!("into: {:?} <- INSERT {:?} + realms", insert_at, (2, 2));
    edge_continents.insert(insert_at, continent);

    // println!(
    //     " \n \nFINISHED continents.len(): {:?}, edge_continents.len(): {:?}",
    //     continents.len(),
    //     edge_continents.len()
    // );

    // create the 2 2 vertical
    // println!(" \n- START create the 2 2 vertical -> \n \n ");
    for y in 3..settings.continent_grid_size.height {
        let x = 2;

        let insert_at = (x - 2, y - 2);
        let moved_coord = (x, y);
        let to_remove = &(x - 1, y);
        let mut continent = continents.remove(&moved_coord).unwrap();
        let left_c = continents.remove(to_remove).unwrap();

        try_map_points_min_max_points_by_points(
            &mut continent.bottom_left,
            &mut continent.top_right,
            &left_c.bottom_left,
            &left_c.top_right,
        );
        continent.grid_coord = Point16::new(insert_at.0, insert_at.1);
        continent.site_point = center_of_two_points(&continent.site_point, &left_c.site_point);

        for realm in left_c.realms {
            continent.realms.push(realm);
        }

        // println!(
        //     "into: {:?} <- INSERT {:?} + realms of {:?}",
        //     insert_at, moved_coord, to_remove
        // );
        edge_continents.insert(insert_at, continent);
    }

    // println!(
    //     " \n \nFINISHED continents.len(): {:?}, new_continents.len(): {:?}",
    //     continents.len(),
    //     new_continents.len()
    // );

    // create the 2 2 horizontal
    // println!(" \n- START create the 2 2 horizontal -> \n \n ");
    for x in 3..settings.continent_grid_size.width {
        let y = 2;

        let insert_at = (x - 2, y - 2);
        let moved_coord = (x, y);
        let to_remove = &(x, y - 1);
        let mut continent = continents.remove(&moved_coord).unwrap();
        let top_c = continents.remove(to_remove).unwrap();
        try_map_points_min_max_points_by_points(
            &mut continent.bottom_left,
            &mut continent.top_right,
            &top_c.bottom_left,
            &top_c.top_right,
        );
        continent.grid_coord = Point16::new(insert_at.0, insert_at.1);
        continent.site_point = center_of_two_points(&continent.site_point, &top_c.site_point);

        for realm in top_c.realms {
            continent.realms.push(realm);
        }

        // println!(
        //     "into: {:?} <- INSERT {:?} + realms of {:?}",
        //     insert_at, moved_coord, to_remove
        // );
        edge_continents.insert(insert_at, continent);
    }

    // println!(
    //     " \n \nFINISHED continents.len(): {:?}, new_continents.len(): {:?}",
    //     continents.len(),
    //     new_continents.len()
    // );

    // move the rest one index less
    for x in 3..settings.continent_grid_size.width {
        for y in 3..settings.continent_grid_size.height {
            let insert_at = (x - 2, y - 2);
            let to_remove = &(x, y);
            let mut continent = continents.remove(to_remove).unwrap();

            continent.grid_coord = Point16::new(insert_at.0, insert_at.1);

            // println!("into: {:?} <- INSERT {:?}", insert_at, to_remove);
            new_continents.insert(insert_at, continent);
        }
    }

    // println!("\n\n FINISHED move the rest one index less -> continents.len(): {:?}, new_continents.len(): {:?} \n\n", continents.len(), new_continents.len());

    (edge_continents, new_continents)
}
