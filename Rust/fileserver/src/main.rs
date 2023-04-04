use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Deserialize, Debug)]
struct Config {
    address: String,
    port: String,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized = serde_json::from_str::<Point>(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
    println!("x: {}, y: {}", point.x, point.y);

    let config = toml::from_str::<Config>(
        r#"
    address = '127.0.0.1'
    port = '1234'
    "#,
    )
    .unwrap();

    println!("address = {}\nport = {}", config.address, config.port);

    let test = ron::from_str::<Config>(
        r#"
        (
            address = '127.0.0.1',
            port = '1234',
        )
    "#,
    )
    .unwrap();

    println!("{}{}", test.address, test.port);
}
