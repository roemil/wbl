use std::{
    io::{Read, Write},
    net::TcpStream
};

fn main() {
    let json_str = r#"
    {
    "name": "MOA",
    "values": {
        "base": 453.5,
        "fuel": 85.0,
        "trip_fuel": 35.0,
        "bagage_back": 0.0,
        "bagage_front": 1.0,
        "bagage_wings": 2.0,
        "pilot": 70.0,
        "co_pilot": 0.0
    }
}
    "#;

    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8080") {
        println!("created stream");
        let result = stream.write(json_str.as_bytes());
        println!("wrote to stream; success={:?}", result.is_ok());
        let mut buf: [u8; 50] = [0; 50];
        let n = stream.read(&mut buf).expect("Read must work lol");
        println!("Got n bytes: {}", n);
        println!("{:?}", String::from_utf8(buf[0..n].to_vec()));
    } else {
        println!("failed lol");
    }
}
