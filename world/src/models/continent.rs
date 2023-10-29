use super::point::Point16;

pub struct Continent {
    pub grid_coord: Point16,
    pub site_point: Point16,
    pub plate_movement_direction: u8,
    // VoronoiEdge edge { get; set; }
    pub elevation: f32,
    // Vector2Int[] gradientSquarePixelCoords { get; set; }
    pub regions: Vec<Region>,
}

impl Continent {
    // Custom constructor function that takes the index and initializes other fields.
    pub fn default() -> Continent {
        Continent {
            grid_coord: Point16::default(),
            site_point: Point16::default(),
            plate_movement_direction: 0,
            elevation: 0.0,
            regions: Vec::new()
        }
    }
}

pub struct Region {
    pub grid_coord: Point16,
    pub site_point: Point16,
    pub provinces: Vec<Province>,
}

impl Region {
    pub fn new(grid_coord: Point16, site_point: Point16) -> Region {
        Region {
            grid_coord,
            site_point,
            provinces: Vec::new()
        }
    }
}

pub struct Province {
    pub site_point: Point16,
    pub pixels: Vec<(u16, u16)>,
}

impl Province {
    pub fn new(site_point: Point16) -> Province {
        Province {
            site_point,
            pixels: Vec::new()
        }
    }
}