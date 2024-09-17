use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PlaneWeights {
    name: String,
    base: f32,
    fuel: f32,
    bagage: Option<f32>,
    bagage_back: Option<f32>,
    bagage_front: Option<f32>,
    bagage_wings: Option<f32>,
    pilot: f32,
    co_pilot: f32,
    passenger_left: Option<f32>,
    passenger_right: Option<f32>
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PlaneConfigs {
    pub moa_json: MoaJson,
    pub ken_json: KenJson,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MoaJson {
    pub name: String,
    pub base: f32,
    pub fuel: f32,
    pub bagage_back: f32,
    pub bagage_front: f32,
    pub bagage_wings: f32,
    pub pilot: f32,
    pub co_pilot: f32,
    pub vortices: [[f32; 2]; 6],
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct KenJson {
    pub name: String,
    pub base: f32,
    pub fuel: f32,
    pub bagage: f32,
    pub pilot: f32,
    pub co_pilot: f32,
    pub passenger_left: f32,
    pub passenger_right: f32,
    pub vortices: [[f32; 2]; 6]
}