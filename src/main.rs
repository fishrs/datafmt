use std::{fs::File, time::Instant};

use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

fn main() {
    let mut collection: Vec<LakeData> = vec![];
    let mut rng = thread_rng();
    for _ in 0..500 {
        let mut data = LakeData::default();

        data.pull = rng.gen_range(0f32..50f32);

        collection.push(data);

    }

    let file = File::create("lakedata.fson").unwrap();
    serde_json::to_writer(file, &collection).unwrap();
}

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
