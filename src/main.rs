use clap::Parser;
use std::error::Error;
use std::io::BufReader;
use std::{collections::HashMap, fs::File};
use wbl::calc_wb::WeightAndBalance;
use wbl::planes::{Input, ParsedInput, PlaneData, PlaneProperties};
use wbl::{iterate_maps, parse_name_from_input, parse_values_from_input, read_plane_config_from_json, WeightLever};

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

    let wb = plane_config.is_weight_and_balance_ok(&plane_properties);
    if wb.is_ok() {
        println!("Plane: {} is approved for W&B", parsed_input.name);

    } else {
        println!("Plane: {} failed W&B for: {:?}", parsed_input.name, wb.unwrap_err());
    }
    println!(
        "Plane: {} has W&B point at: {:?}",
        parsed_input.name,
        plane_config.calc_weight_and_balance(&plane_properties)
    );

    let wb_landing = plane_config.is_weight_and_balance_ok(&plane_properties);
    if wb_landing.is_ok() {
        println!("Plane: {} is approved for W&B when landing", parsed_input.name);

    } else {
        println!("Plane: {} failed landing W&B for: {:?}", parsed_input.name, wb_landing.unwrap_err());
    }

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
