// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

include!(env!("SLINT_INCLUDE_GENERATED"));

use core::fmt;
use std::error::Error;
use std::hash::Hash;
use std::io::BufReader;
use std::str::FromStr;
use std::{collections::HashMap, fs::File};
use clap::Parser;
use wbl::calc_wb::WeightAndBalance;
use wbl::planes::{PlaneData, PlaneProperties};
use wbl::{Kind, ViktArm};

pub fn iterate_maps<'a: 'b, 'b, K: Eq + Hash + fmt::Debug, V>(
    m1: &'a HashMap<K, V>,
    m2: &'b HashMap<K, V>,
) -> impl Iterator<Item = (&'a K, &'a V, &'b V)> {
    m1.iter().map(move |(k, v1)| {
        (
            k,
            v1,
            m2.get(k)
                .expect(&format!("Expected key: {:?}", &k).to_string()),
        )
    })
}

fn read_plane_config_from_json(path : &str) -> Vec<PlaneData> {
    let file = File::open(path).expect("Config not found");
    let reader = BufReader::new(file);
    let jsons = serde_json::Deserializer::from_reader(reader).into_iter::<Vec<PlaneData>>();
    let mut planes_vec: Vec<PlaneData> = Vec::new();
    for json in jsons {
        if let Ok(planes) = json {
            for plane in planes {
                planes_vec.push(plane);
            }
        }
    }

    planes_vec
}

fn parse_input_file(path :&str) -> (String, HashMap<Kind, f32>) {
    let file = File::open(path).expect("Input file not found");
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
                        (object
                            .1.as_f64().expect("Expected float")) as f32,
                    );
                }
            }
        }
    }
    return (name, weights);
}

#[derive(Parser, Debug)]
struct Args{
    #[arg(short, long)]
    path : String
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();


    let planes = read_plane_config_from_json("./src/input/config.json");
    let (name, weights) = parse_input_file(&args.path);
    let plane_config: &PlaneData =
        &planes[planes.iter().position(|plane| plane.name == name).unwrap()];
    let plane_limits = plane_config.to_lever_map();
    let plane_properties = PlaneProperties::new(iterate_maps(&plane_limits, &weights).fold(
        HashMap::new(),
        move |mut props, (k, a, w)| {
            props.insert(*k, ViktArm::new(*w, *a));
            props
        },
    ));
    println!(
        "Plane: {} has W&B that is ok: {}",
        name,
        plane_config.is_weight_and_balance_ok(&plane_properties)
    );
    println!(
        "Plane: {} has W&B point at: {:?}",
        name,
        plane_config.calc_weight_and_balance(&plane_properties)
    );

    // // /*
    // // TODO:
    // // 1. Front end TUI
    // // 2. clean up
    // //  */

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
