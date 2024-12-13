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
    #[serde(default)]
    orders: Vec<Order>,
}

#[serde_with::serde_as]
#[derive(Serialize,Deserialize)]
struct Order {
    item: String,
    #[serde_as(deserialize_as = "serde_with::DefaultOnError")]
    #[serde(default)]
    pub quantity: Option<u32>,
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
quantity = 1.5

[[package.metadata.orders]]
item = "Doll"
quantity = 2

[[package.metadata.orders]]
quantity = 5
item = "Cookie:::\n"

[[package.metadata.orders]]
item = "Thing"
count = 3

    "#;
    let config: Config = toml::from_str(toml_data).unwrap();
    let r = config.package.metadata.orders
        .iter().filter(|o| o.quantity.is_some())
        .map(|item| format!("{}: {}", item.item, item.quantity.unwrap()))
        .collect::<Vec<String>>().join("\n");
    println!("r: {}", r);
    assert_eq!(r, "Toy car: 2");
}