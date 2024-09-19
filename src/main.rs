use clap::Parser;
use core::fmt;
use std::error::Error;
use std::hash::Hash;
use std::io::BufReader;
use std::str::FromStr;
use std::{collections::HashMap, fs::File};
use wbl::calc_wb::WeightAndBalance;
use wbl::planes::{Input, ParsedInput, PlaneData, PlaneProperties};
use wbl::{Kind, WeightLever};

pub fn iterate_maps<'a: 'b, 'b, K: Eq + Hash + fmt::Debug, V>(
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

fn read_plane_config_from_json(path: &str) -> Vec<PlaneData> {
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

fn parse_name_from_input(input: &Input) -> String {
    input.name.to_string().trim_matches('\"').to_string()
}

fn parse_values_from_input(input: &Input) -> HashMap<Kind, f32> {
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

fn parse_input_file(path: &str) -> ParsedInput {
    let file = File::open(path).expect("Input file not found");
    let reader = BufReader::new(file);
    let input: Input = serde_json::from_reader(reader).expect("Invalid format of input file");

    ParsedInput {
        name: parse_name_from_input(&input),
        values: parse_values_from_input(&input),
    }
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let planes = read_plane_config_from_json("./src/input/config.json");
    let parsed_input = parse_input_file(&args.path);
    let plane_config: &PlaneData = &planes[planes
        .iter()
        .position(|plane| plane.name == parsed_input.name)
        .expect("Plane missing in config")];
    let plane_levers = plane_config.to_lever_map();
    let plane_properties =
        PlaneProperties::new(iterate_maps(&plane_levers, &parsed_input.values).fold(
            HashMap::new(),
            move |mut properties, (k, a, w)| {
                properties.insert(*k, WeightLever::new(*w, *a));
                properties
            },
        ));
    println!(
        "Plane: {} has W&B that is ok: {}",
        parsed_input.name,
        plane_config.is_weight_and_balance_ok(&plane_properties)
    );
    println!(
        "Plane: {} has W&B point at: {:?}",
        parsed_input.name,
        plane_config.calc_weight_and_balance(&plane_properties)
    );

    println!(
        "Plane: {} has landing W&B that is ok: {}",
        parsed_input.name,
        plane_config.is_landing_weight_and_balance_ok(&plane_properties)
    );

    println!(
        "Plane: {} has a landing W&B point at: {:?}",
        parsed_input.name,
        plane_config.calc_landing_weight_and_balance(&plane_properties)
    );

    // // /*
    // // TODO:
    // // 1. Front end TUI
    // // 2. clean up
    // //  */
    Ok(())
}
