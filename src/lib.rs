use core::fmt;
use std::{collections::HashMap, fs::File, io::BufReader, str::FromStr};

use num::complex::ComplexFloat;
use planes::{Input, PlaneData};
use serde::{Deserialize, Serialize};

pub mod calc_wb;
pub mod planes;

#[derive(
    Default, PartialEq, Eq, Hash, Debug, Clone, Copy, PartialOrd, Ord, Deserialize, Serialize,
)]
pub enum Kind {
    #[default]
    NoValue,
    Base,
    Fuel,
    Bagage,
    BagageFront,
    BagageBack,
    BagageWings,
    Pilot,
    CoPilot,
    PaxLeftBack,
    PaxRightBack,
    TripFuel,
}

#[derive(Debug, PartialEq)]
pub enum FailReason {
    Bagage,
    BagageFront,
    BagageBack,
    BagageWings,
    MaxTakeOffWeight,
    MaxWingLoad,
    Fuel,
    ZeroFuel,
    LandingFuel,
    TorqueOutOfBounds,
}

impl FromStr for Kind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NoValue" => Ok(Kind::NoValue),
            "base" => Ok(Kind::Base),
            "fuel" => Ok(Kind::Fuel),
            "bagage" => Ok(Kind::Bagage),
            "bagage_front" => Ok(Kind::BagageFront),
            "bagage_back" => Ok(Kind::BagageBack),
            "bagage_wings" => Ok(Kind::BagageWings),
            "pilot" => Ok(Kind::Pilot),
            "co_pilot" => Ok(Kind::CoPilot),
            "passenger_left" => Ok(Kind::PaxLeftBack),
            "passenger_right" => Ok(Kind::PaxRightBack),
            "trip_fuel" => Ok(Kind::TripFuel),
            _ => Err(format!("Invalid value of string: {}", s)),
        }
    }
}

fn is_value_within_weight_limit(
    properties: &std::collections::HashMap<Kind, WeightLever>,
    kind: Kind,
    limit: f32,
) -> bool {
    let mut is_item_within_limit = true;
    if let Some(item) = properties.get(&kind) {
        is_item_within_limit = item.weight <= limit;
    }
    is_item_within_limit
}

#[derive(PartialEq, PartialOrd, Debug, Clone, Deserialize, Serialize, Copy)]
pub struct WeightLever {
    pub weight: f32,
    pub lever: f32,
}

impl std::default::Default for WeightLever {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl WeightLever {
    pub fn new(weight: f32, lever: f32) -> Self {
        WeightLever { weight, lever }
    }

    pub fn torque(&self) -> f32 {
        self.lever * self.weight
    }
}

//ref: https://www.linkedin.com/pulse/short-formula-check-given-point-lies-inside-outside-polygon-ziemecki/
pub fn is_inside_polygon(
    point: WeightLever,
    vertices: &[WeightLever; 6],
    valid_border: bool,
) -> Result<(), FailReason> {
    let mut sum = num::complex::Complex::new(0.0, 0.0);

    for i in 1..vertices.len() + 1 {
        let v0 = &vertices[i - 1];
        let v1 = &vertices[i % vertices.len()];

        if is_point_in_segment(&point, v0, v1) {
            if valid_border {
                return Ok(());
            } else {
                return Err(FailReason::TorqueOutOfBounds);
            }
        }
        let v1_c = num::complex::Complex::new(v1.lever, v1.weight);
        let p_c = num::complex::Complex::new(point.lever, point.weight);
        let v0_c = num::complex::Complex::new(v0.lever, v0.weight);
        sum += num::complex::Complex::ln((v1_c - p_c) / (v0_c - p_c));
    }

    if sum.abs() <= 1.0 {
        return Err(FailReason::TorqueOutOfBounds);
    }
    Ok(())
}

fn is_point_in_segment(p: &WeightLever, p0: &WeightLever, p1: &WeightLever) -> bool {
    let p0 = WeightLever::new(p0.weight - p.weight, p0.lever - p.lever);
    let p1 = WeightLever::new(p1.weight - p.weight, p1.lever - p.lever);

    let det = p0.weight * p1.lever - p1.weight * p0.lever;
    let prod = p0.weight * p1.weight + p0.lever * p1.lever;

    (det == 0.0 && prod < 0.0)
        || (p0.weight == 0.0 && p0.lever == 0.0)
        || (p1.weight == 0.0 && p1.lever == 0.0)
}

pub fn read_plane_config_from_json(path: &str) -> Vec<PlaneData> {
    let file = File::open(path).expect("Config not found");
    let reader = BufReader::new(file);
    let jsons = serde_json::Deserializer::from_reader(reader)
        .into_iter::<Vec<PlaneData>>()
        .flatten();
    let mut planes_vec: Vec<PlaneData> = Vec::new();
    for planes in jsons {
        for plane in planes {
            planes_vec.push(plane);
        }
    }

    planes_vec
}

pub fn iterate_maps<'a: 'b, 'b, K: Eq + std::hash::Hash + fmt::Debug, V>(
    m1: &'a HashMap<K, V>,
    m2: &'b HashMap<K, V>,
) -> impl Iterator<Item = (&'a K, &'a V, &'b V)> {
    m1.iter().map(move |(k, v1)| {
        (
            k,
            v1,
            m2.get(k)
                .unwrap_or_else(|| panic!("{}", format!("Expected key: {:?}", &k))),
        )
    })
}

pub fn parse_name_from_input(input: &Input) -> String {
    input.name.to_string().trim_matches('\"').to_string()
}

pub fn parse_values_from_input(input: &Input) -> HashMap<Kind, f32> {
    let mut weights = HashMap::new();
    for key in input.values.keys() {
        let v = input.values.get(key).unwrap();
        weights.insert(
            Kind::from_str(key).expect("Incorrect format in json values"),
            v.as_f64().expect("Expected float") as f32,
        );
    }
    weights
}
