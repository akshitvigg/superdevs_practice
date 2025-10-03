use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]

pub struct CreateOrderOutput {
    pub order_no: String,
}
