use actix_web::{App, HttpResponse, HttpServer, Responder, delete, get, post, web::Json};
use serde::{Deserialize, Serialize};
#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(move || {
        App::new()
            .service(create_order)
            .service(delete_order)
            .service(get_depth)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[derive(Serialize, Deserialize, Debug)]
struct CreateOrderSchema {
    price: u32,
    quantity: u32,
    user_id: u32,
    side: Side,
}

#[derive(Serialize, Deserialize, Debug)]
enum Side {
    Buy,
    Sell,
}

#[post("/order")]
async fn create_order(body: Json<CreateOrderSchema>) -> impl Responder {
    println!("{:?}", body);
    // let b = CreateOrderSchema {
    //     price: 12,
    //     quantity: 20,
    //     user_id: 3,
    //     side: Side::Buy,
    // };
    return HttpResponse::Ok().json("hello u got this ");
}

#[delete("/order")]
async fn delete_order() -> impl Responder {
    "delete order"
}

#[get("/depth")]
async fn get_depth() -> impl Responder {
    "get order"
}
