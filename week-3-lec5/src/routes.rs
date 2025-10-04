use actix_web::{HttpResponse, Responder, delete, get, post, web::Json};

use crate::{
    inputs::{CreateOrderSchema, DeleteOrderSchema},
    outputs::{CreateOrderOutput, DeleteOrderOutput, Depth},
};

#[post("/order")]
pub async fn create_order(body: Json<CreateOrderSchema>) -> impl Responder {
    let price = body.0.price;
    let quantity = body.0.quantity;
    let user_id = body.0.user_id;
    let side = body.0.side;

    return HttpResponse::Ok().json(CreateOrderOutput {
        order_no: String::from("this is your order number : 1"),
    });
}

#[delete("/order")]
pub async fn delete_order(Json(body): Json<DeleteOrderSchema>) -> impl Responder {
    let orderid = body.order_id;

    HttpResponse::Ok().json(DeleteOrderOutput {
        filled_qty: 23,
        average_price: 333,
    })
}

#[get("/depth")]
pub async fn get_depth() -> impl Responder {
    HttpResponse::Ok().json(Depth {
        bids: vec![],
        asks: vec![],
        last_update_id: String::from("this id"),
    })
}
