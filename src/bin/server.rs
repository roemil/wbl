use std::{collections::HashMap, io};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream}, time::{self, sleep},
};
use wbl::{
    calc_wb::WeightAndBalance,
    iterate_maps, parse_name_from_input, parse_values_from_input,
    planes::{Input, ParsedInput, PlaneData, PlaneProperties},
    read_plane_config_from_json, FailReason, WeightLever,
};

fn calc(input: Input) -> Result<(), FailReason> {
    println!("Got data: {:?}", input);
    let planes = read_plane_config_from_json("./src/input/config.json");

    let parsed_input = ParsedInput {
        name: parse_name_from_input(&input),
        values: parse_values_from_input(&input),
    };

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

    plane_config.is_weight_and_balance_ok(&plane_properties)
}

async fn process(mut socket: TcpStream) -> io::Result<()> {
    let mut buf = Vec::with_capacity(1024);
    let n = socket.read_buf(&mut buf).await?;
    if n == 0 {
        println!("Client disconnected");
        return Ok(());
    }
    if let Ok(str) = String::from_utf8(buf[0..n].to_vec()) {
        let json = serde_json::from_str::<Input>(&str);
        match json {
            Ok(data) => {
                let wb = calc(data);
                if wb.is_ok() {
                    println!("OK");
                    let res = socket.write_all(b"W&B is OK").await;
                    println!("Write success={:?}", res.is_ok());
                } else {
                    let _ = socket.write_all(b"W&B is NOT OK").await;
                }
            }
            Err(e) => println!("Failed to parsed json: {}", e),
        }
    } else {
        println!("Failed to parse json");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Could not create listener");

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await?;
        process(socket).await?;
    }
}
