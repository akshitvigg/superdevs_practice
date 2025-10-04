use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOrderSchema {
    pub price: u32,
    pub quantity: u32,
    pub user_id: u32,
    pub side: Side,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteOrderSchema {
    pub order_id: u32,
}
