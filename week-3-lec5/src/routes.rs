use actix_web::{HttpResponse, Responder, delete, get, post, web::Json};

use crate::{inputs::CreateOrderSchema, outputs::CreateOrderOutput};

#[post("/order")]
pub async fn create_order(body: Json<CreateOrderSchema>) -> impl Responder {
    println!("{:?}", body);
    return HttpResponse::Ok().json(CreateOrderOutput {
        order_no: String::from("this is your order number : 1"),
    });
}

#[delete("/order")]
pub async fn delete_order() -> impl Responder {
    "delete order"
}

#[get("/depth")]
pub async fn get_depth() -> impl Responder {
    "get order"
}
