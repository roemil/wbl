// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

include!(env!("SLINT_INCLUDE_GENERATED"));

use std::error::Error;
use std::hash::Hash;
use std::io::BufReader;
use std::str::FromStr;
use std::{collections::HashMap, fs::File};
use serde_json::Value;
use wbl::ken::{Ken, KenConfig};
use wbl::{calc_wb::CalcWeightAndBalance, moa::Moa, Kind, ViktArm};
use wbl::{update_weight, MoaConfig};
use wbl::planes::PlaneConfigs;

fn get_ken_weights() -> KenConfig {
    let config = HashMap::from([
        (Kind::Base, 685.2),
        (Kind::Fuel, 129.0),
        (Kind::Bagage, 20.0),
        (Kind::Pilot, 70.0),
        (Kind::CoPilot, 0.0),
        (Kind::PaxLeftBack, 0.0),
        (Kind::PaxRightBack, 0.0),
    ]);
    let mut ken_config = KenConfig::new();
    ken_config.config = config;
    ken_config
}

pub fn iterate_maps<'a: 'b, 'b, K: Eq + Hash, V>(
    m1: &'a HashMap<K, V>,
    m2: &'b HashMap<K, V>,
) -> impl Iterator<Item = (&'a K, &'a V, &'b V)> {
    m1.iter().map(move |(k, v1)| (k, v1, m2.get(k).unwrap()))
}

fn json_object_to_hashmap(object : &serde_json::Map<String, Value>) -> HashMap<Kind, f32>{
    let mut data = HashMap::new();
    for (kind, value) in object {
        if kind != "vortices" {
            data.insert(Kind::from_str(kind).expect("Invalid type"), value.to_string().trim_matches('\"').parse::<f32>().expect("Expected float"));
        }
    }

    data
}

fn json_value_to_f32(value : &Value) -> f32{
    value.to_string().trim_matches('\"').parse::<f32>().expect("Expected float")
}

fn flatten_vortices(vortices : &Vec<serde_json::Value>) -> [ViktArm;6]{
    vortices
    .iter()
    .map(|vortex| ViktArm::new(json_value_to_f32(&vortex[0]),json_value_to_f32(& vortex[1])))
    .collect::<Vec<ViktArm>>()
    .try_into()
    .expect("Should be able to create array")
}

fn read_plane_config_from_json() -> (HashMap<String, HashMap<Kind, f32>>, HashMap<String, [ViktArm;6]>) {
    let file = File::open("./src/config.json").expect("File not found");
    let reader = BufReader::new(file);
    let jsons = serde_json::Deserializer::from_reader(reader).into_iter::<serde_json::Value>();
    let mut planes_map = HashMap::new();
    let mut vortices_map = HashMap::new();
    for json in jsons {
        if let Ok(planes_json) = json {
            if let Some(planes) = planes_json.as_object() {
                for (reg, data) in planes {
                    if let Some(data_points) = data.as_object() {
                        let data = json_object_to_hashmap(data_points);
                        let vortices = flatten_vortices(&data_points["vortices"].as_array().expect("Vortices must exist"));
                        let reg_str = reg.to_string().trim_matches('\"').to_string();
                        vortices_map.insert(reg_str.clone(), vortices);
                        planes_map.insert(reg_str, data);
                    }
                }
            }
        }
    }
    (planes_map, vortices_map)
}

fn parse_input_file() -> (String, HashMap<Kind, f32>) {
    let file = File::open("./src/input.json").expect("File not found");
    let reader = BufReader::new(file);
    let jsons = serde_json::Deserializer::from_reader(reader).into_iter::<serde_json::Value>();
    let mut weights = HashMap::new();
    let mut name = String::new();
    for json in jsons {
        if let Ok(input) = json {
            if let Some(tmp) = input.as_object() {
                for object in tmp {
                    if object.0 == "name" {
                        name = object.1.to_string().trim_matches('\"').to_string();
                        continue;
                    }
                    weights.insert(
                        Kind::from_str(object.0).unwrap(),
                        object.1.to_string().trim_matches('\"').parse::<f32>().unwrap(),
                    );
                }
            }
        }
    }
    //println!("Input file. Name: {}, objects: {:?}", name, weights);
    return (name, weights);
}

fn main() -> Result<(), Box<dyn Error>> {
    let (name, weights) = parse_input_file();
    let (planes, vortices) = read_plane_config_from_json();
    let plane_config = &planes[&name];
    let plane_properties = iterate_maps(&plane_config, &weights).fold(
                HashMap::new(),
                move |mut props, (k, a, w)| {
                    props.insert(*k, ViktArm::new(*w, *a));
                    props
                },
            );
    let moa = Moa::new(plane_properties, vortices[&name].clone());
    println!("Is MOA config ok? {}", moa.is_weight_and_balance_ok());
    // let moa_levers = MoaConfig::from(planes.moa_json);
    // if name == "MOA" {
    //     let moa_properties = iterate_maps(&moa_levers.config, &weights).fold(
    //         HashMap::new(),
    //         move |mut props, (k, a, w)| {
    //             props.insert(*k, ViktArm::new(*w, *a));
    //             props
    //         },
    //     );
    //     let mut moa = Moa::new(moa_properties, moa_levers.vortices);
    //     println!("Is MOA config ok? {}", moa.is_weight_and_balance_ok());
    //     println!("Point: {:?}", moa.calc_weight_and_balance());

    //     let _ = update_weight(&mut moa.properties, Kind::CoPilot, 500.0);
    //     println!("Is MOA config ok? {}", moa.is_weight_and_balance_ok());
    //     println!("Point: {:?}", moa.calc_weight_and_balance());
    // } else {
    //     println!("Name not valid?");
    // }

    // let ken_config = KenConfig::from(planes.ken_json);
    // let ken_weights = get_ken_weights();
    // let ken_properties = iterate_maps(&ken_config.config, &ken_weights.config).fold(
    //     HashMap::new(),
    //     move |mut props, (k, a, w)| {
    //         props.insert(*k, ViktArm::new(*w, *a));
    //         props
    //     },
    // );

    // // /*
    // // TODO:
    // // 1. Front end TUI
    // // 2. clean up
    // //  */

    // let mut ken = Ken::new(ken_properties, ken_config.vortices);
    // println!("Is KEN config ok? {}", ken.is_weight_and_balance_ok());
    // println!("Point: {:?}", ken.calc_weight_and_balance());

    // let _ = update_weight(&mut ken.properties, Kind::CoPilot, 500.0);
    // println!("Is KEN config ok? {}", ken.is_weight_and_balance_ok());
    // println!("Point: {:?}", ken.calc_weight_and_balance());

    // // let ui = AppWindow::new()?;

    // // // ui.on_request_increase_value({
    // // //     let ui_handle = ui.as_weak();
    // // //     move || {
    // // //         let ui = ui_handle.unwrap();
    // // //         ui.set_counter(ui.get_counter() + 1);
    // // //     }
    // // // });

    // // ui.run()?;

    Ok(())
}
