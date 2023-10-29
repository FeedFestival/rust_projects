use super::point::PointU16;

pub struct Continent {
    pub index: u16,
    pub grid_position: PointU16,
    pub site: PointU16,
    pub plate_movement_direction: u8,
    // VoronoiEdge edge { get; set; }
    pub elevation: f32,
    // Vector2Int[] gradientSquarePixelCoords { get; set; }
    // Dictionary<int, Dictionary<int, IRegionPoint>> RegionPoints { get; set; }
    pub pixels_size: Option<PointU16>, // TODO: use this for gradient or remove
    pub pixels: Option<Vec<(u16, u16)>>,
}

impl Continent {
    // Custom constructor function that takes the index and initializes other fields.
    pub fn default() -> Continent {
        Continent {
            index: 0,
            grid_position: PointU16::default(),
            site: PointU16::default(),
            plate_movement_direction: 0,
            elevation: 0.0,
            pixels_size: None,
            pixels: None,
        }
    }
}
