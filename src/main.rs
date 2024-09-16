use std::hash::Hash;
use std::io::BufReader;
use std::{collections::HashMap, fs::File};
use wbl::{
    calc_wb::CalcWeightAndBalance, moa::Moa, Kind, ViktArm,
};
use wbl::{update_weight, MoaConfig, PlaneConfigs};

fn get_moa_weights() -> MoaConfig {
    let config = HashMap::from([
        (Kind::Base, 453.5),
        (Kind::Fuel, 85.0),
        (Kind::BagageBack, 0.0),
        (Kind::BagageFront, 1.0),
        (Kind::BagageWings, 2.0),
        (Kind::Pilot, 70.0),
        (Kind::CoPilot, 0.0),
    ]);
    let mut moaconfig = MoaConfig::new();
    moaconfig.config = config;
    moaconfig
}

pub fn iterate_maps<'a: 'b, 'b, K: Eq + Hash, V>(
    m1: &'a HashMap<K, V>,
    m2: &'b HashMap<K, V>,
) -> impl Iterator<Item = (&'a K, &'a V, &'b V)> {
    m1.iter().map(move |(k, v1)| (k, v1, m2.get(k).unwrap()))
}

fn read_from_json_file() -> Result<PlaneConfigs, String> {
    let file = File::open("./src/config.json").expect("File not found");
    let reader = BufReader::new(file);
    let jsons = serde_json::Deserializer::from_reader(reader).into_iter::<PlaneConfigs>();
    for json in jsons {
        println!("json: {:?}", json);
        if let Ok(planes_json) = json {
            return Ok(planes_json);
        }
    }
    Err("No plane config found".to_string())
}

fn main() -> Result<(), String> {
    let planes = read_from_json_file()?;
    let arm = MoaConfig::from(planes.moa_json);
    // Weights are inputs from user
    let weights = get_moa_weights();

    let moa_properties = iterate_maps(&arm.config, &weights.config).fold(
        HashMap::new(),
        move |mut props, (k, a, w)| {
            props.insert(*k, ViktArm::new(*w, *a));
            props
        },
    );

    /*
    TODO:
    1. Front end TUI
    2. clean up
     */

    let mut moa = Moa::new(moa_properties, arm.vortices);
    println!("Is MOA config ok? {}", moa.is_weight_and_balance_ok());
    println!("Point: {:?}", moa.calc_weight_and_balance());

    let _ = update_weight(&mut moa.properties, Kind::CoPilot, 500.0);
    println!("Is MOA config ok? {}", moa.is_weight_and_balance_ok());
    println!("Point: {:?}", moa.calc_weight_and_balance());
    Ok(())
}
