use voronoice::Point;

use super::point::PointU16;

pub struct Continent {
    pub grid_coord: PointU16,
    pub site_point: PointU16,
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
            grid_coord: PointU16::default(),
            site_point: PointU16::default(),
            plate_movement_direction: 0,
            elevation: 0.0,
            regions: Vec::new()
        }
    }
}

pub struct Region {
    pub x: u16,
    pub y: u16,
    pub site_point: PointU16,
    pub pixels: Vec<(u16, u16)>,
}

// TODO: I think we can just do Region... 
pub struct RegionSite {
    pub x: u16,
    pub y: u16,
    pub site_point: Point,
    pub pixels: Vec<(u16, u16)>
}

impl RegionSite {
    pub fn new(x: u16, y: u16, point: Point) -> RegionSite {
        RegionSite {
            x,
            y,
            site_point: point,
            pixels: Vec::new()
        }
    }
}