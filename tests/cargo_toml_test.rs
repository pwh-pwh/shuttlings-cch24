use cargo_manifest::Manifest;
use std::str::FromStr;

#[test]
fn test_toml_cargo() {
    let t = r#"
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
    let manifest = Manifest::from_str(t).unwrap();
    println!("{:?}", manifest);
}
