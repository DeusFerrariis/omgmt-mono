use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SKU(pub String);

#[derive(Serialize, Deserialize)]
pub struct Amount(u32);

#[derive(Serialize, Deserialize)]
pub enum ProductEvent {
    Created { sku: SKU, retail: Amount },
    PriceIncrease { amount: Amount, retail: Amount },
    PriceDecrease { amount: Amount, retail: Amount },
}
