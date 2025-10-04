use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]

pub struct CreateOrderOutput {
    pub order_no: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteOrderOutput {
    pub filled_qty: u32,
    pub average_price: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Depth {
    pub bids: Vec<[u32; 2]>,
    pub asks: Vec<[u32; 2]>,
    pub last_update_id: String,
}
