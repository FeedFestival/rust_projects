use voronoice::Point;

use super::point::PointU16;

pub struct Continent {
    pub index: u16,
    pub grid_coord: PointU16,
    pub grid_position: PointU16,
    pub site_point: PointU16,
    pub plate_movement_direction: u8,
    // VoronoiEdge edge { get; set; }
    pub elevation: f32,
    // Vector2Int[] gradientSquarePixelCoords { get; set; }
    pub regions: Vec<Region>,
    pub pixels_size: Option<PointU16>, // TODO: use this for gradient or remove
    pub pixels: Option<Vec<(u16, u16)>>,
}

impl Continent {
    // Custom constructor function that takes the index and initializes other fields.
    pub fn default() -> Continent {
        Continent {
            index: 0,
            grid_coord: PointU16::default(),
            grid_position: PointU16::default(),
            site_point: PointU16::default(),
            plate_movement_direction: 0,
            elevation: 0.0,
            regions: Vec::new(),
            pixels_size: None,
            pixels: None,
        }
    }
}

pub struct Region {
    pub x: u16,
    pub y: u16,
    pub grid_position: PointU16,
    pub site_point: PointU16,
    pub pixels: Vec<(u16, u16)>,
}

// TODO: I think we can just do Region... 
pub struct RegionSite {
    pub index: u16,
    pub x: u16,
    pub y: u16,
    pub grid_position: PointU16,
    pub point: Point,
    pub pixels: Vec<(u16, u16)>
}

impl RegionSite {
    pub fn new(index: u16, x: u16, y: u16, grid_position: PointU16, point: Point) -> RegionSite {
        RegionSite {
            index,
            x,
            y,
            grid_position,
            point,
            pixels: Vec::new()
        }
    }
}