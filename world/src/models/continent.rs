use super::point::Point16;

pub struct Continent {
    pub grid_coord: Point16,
    pub site_point: Point16,
    pub plate_movement_direction: u8,
    // VoronoiEdge edge { get; set; }
    pub elevation: f32,
    // Vector2Int[] gradientSquarePixelCoords { get; set; }
    pub realms: Vec<Realm>,
}

impl Continent {
    // Custom constructor function that takes the index and initializes other fields.
    pub fn default() -> Continent {
        Continent {
            grid_coord: Point16::default(),
            site_point: Point16::default(),
            plate_movement_direction: 0,
            elevation: 0.0,
            realms: Vec::new()
        }
    }
}

pub struct Realm {
    pub grid_coord: Point16,
    pub site_point: Point16,
    pub provinces: Vec<Province>,
}

impl Realm {
    pub fn new(grid_coord: Point16, site_point: Point16) -> Realm {
        Realm {
            grid_coord,
            site_point,
            provinces: Vec::new()
        }
    }
}

pub struct Province {
    pub grid_coord: Point16,
    pub site_point: Point16,
    pub regions: Vec<Region>,
}

impl Province {
    pub fn new(grid_coord: Point16, site_point: Point16) -> Province {
        Province {
            grid_coord,
            site_point,
            regions: Vec::new()
        }
    }
}

pub struct Region {
    pub site_point: Point16,
    pub pixels: Vec<(u16, u16)>,
}

impl Region {
    pub fn new(site_point: Point16) -> Region {
        Region {
            site_point,
            pixels: Vec::new()
        }
    }
}
