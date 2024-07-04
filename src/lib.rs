use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LakeData{
    pub coords: Coords,
    pub pull: f64,
    label: Option<bool>
}

impl Default for LakeData {
    fn default() -> Self {
        Self { coords: Coords::default(), pull: 0f64, label: None }
    }
}

impl LakeData {
    pub fn new(lon: f64, lat: f64, pull: f64, label: bool) -> Self {
        Self {coords: Coords::new(lon, lat), pull, label: Some(label)}
    }
}

impl Default for Coords {
    fn default() -> Self {
        Self { lon: 0f64, lat: 0f64 }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Coords {
    lon: f64,
    lat: f64
}

impl Coords {
    pub fn new(lon: f64, lat: f64) -> Self { Self { lon, lat } }
    pub fn export(&self) -> (f64, f64) {
        (self.lon, self.lat)
    }
}
