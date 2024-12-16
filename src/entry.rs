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

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    #[serde(default)]
    pub orders: Vec<Order>,
}

#[serde_with::serde_as]
#[derive(Serialize, Deserialize)]
pub struct Order {
    pub item: String,
    #[serde_as(deserialize_as = "serde_with::DefaultOnError")]
    #[serde(default)]
    pub quantity: Option<u32>,
}
