use super::point::Point16;

pub struct Continent {
    pub grid_coord: Point16,
    pub site_point: Point16,
    pub top_right: Point16,
    pub bottom_left: Point16,
    pub plate_movement_direction: u8,
    // VoronoiEdge edge { get; set; }
    pub elevation: f32,
    // Vector2Int[] gradientSquarePixelCoords { get; set; }
    pub realms: Vec<Realm>,
}

impl Continent {
    pub fn default() -> Continent {
        Continent {
            grid_coord: Point16::default(),
            site_point: Point16::default(),
            top_right: Point16::new(u16::MIN, u16::MIN),
            bottom_left: Point16::new(u16::MAX, u16::MAX),
            plate_movement_direction: 0,
            elevation: 0.0,
            realms: Vec::new()
        }
    }
    pub fn new(grid_coord: Point16, site_point: Point16, plate_movement_direction: u8, elevation: f32) -> Continent {
        Continent {
            grid_coord,
            site_point,
            top_right: Point16::new(u16::MIN, u16::MIN),
            bottom_left: Point16::new(u16::MAX, u16::MAX),
            plate_movement_direction,
            elevation,
            realms: Vec::new()
        }
    }
}

pub struct Realm {
    pub grid_coord: Point16,
    pub site_point: Point16,
    pub top_right: Point16,
    pub bottom_left: Point16,
    pub provinces: Vec<Province>,
}

impl Realm {
    pub fn new(grid_coord: Point16, site_point: Point16) -> Realm {
        Realm {
            grid_coord,
            site_point,
            top_right: Point16::new(u16::MIN, u16::MIN),
            bottom_left: Point16::new(u16::MAX, u16::MAX),
            provinces: Vec::new()
        }
    }
}

pub struct Province {
    pub grid_coord: Point16,
    pub site_point: Point16,
    pub top_right: Point16,
    pub bottom_left: Point16,
    pub regions: Vec<Region>,
}

impl Province {
    pub fn new(grid_coord: Point16, site_point: Point16) -> Province {
        Province {
            grid_coord,
            site_point,
            top_right: Point16::new(u16::MIN, u16::MIN),
            bottom_left: Point16::new(u16::MAX, u16::MAX),
            regions: Vec::new()
        }
    }
}

pub struct Region {
    pub site_point: Point16,
    pub top_right: Point16,
    pub bottom_left: Point16,
    pub pixels: Vec<(u16, u16)>,
}

impl Region {
    pub fn new(site_point: Point16) -> Region {
        Region {
            site_point,
            top_right: Point16::default(),
            bottom_left: Point16::default(),
            pixels: Vec::new()
        }
    }
}
