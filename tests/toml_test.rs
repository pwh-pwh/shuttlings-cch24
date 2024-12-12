use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Config {
    package: Package,
}

#[derive(Serialize, Deserialize)]
struct Package {
    name: String,
    authors: Vec<String>,
    keywords: Vec<String>,
    metadata: Metadata,
}

#[derive(Serialize,Deserialize)]
struct Metadata {
    orders: Vec<Order>,
}

#[derive(Serialize,Deserialize)]
struct Order {
    item: String,
    quantity: u32,
}

#[test]
fn test_toml() {
    let toml_data = r#"
[package]
name = "not-a-gift-order"
authors = ["Not Santa"]
keywords = ["Christmas 2024"]

[[package.metadata.orders]]
item = "Toy car"
quantity = 2

[[package.metadata.orders]]
item = "Lego brick"
quantity = 230
    "#;

    let config: Config = toml::from_str(toml_data).unwrap();
    println!("{}", config.package.name);
}