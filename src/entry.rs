use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Config {
    pub package: Package,
}

#[derive(Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub authors: Vec<String>,
    pub keywords: Vec<String>,
    pub metadata: Metadata,
}

#[derive(Serialize,Deserialize)]
pub struct Metadata {
    pub orders: Vec<Order>,
}

#[derive(Serialize,Deserialize)]
pub struct Order {
    pub item: String,
    pub quantity: u32,
}