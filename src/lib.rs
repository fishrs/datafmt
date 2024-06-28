use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct LakeData{
    coords: Coords,
    pull: f32,
    label: Option<bool>
}

impl Default for LakeData {
    fn default() -> Self {
        Self { coords: Coords::default(), pull: 0f32, label: None }
    }
}

impl Default for Coords {
    fn default() -> Self {
        Self { lon: 0f32, lat: 0f32 }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Coords {
    lon: f32,
    lat: f32
}
