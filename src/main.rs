mod task;

#[macro_use]
extern crate serde_derive;

use actix_web::{web, App, HttpServer, Responder, get, post, put, delete};
use serde_json;

#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String
}

#[get("/api/v1/user/{name}")]
async fn get_method(name: web::Path<String>) -> impl Responder {
    let user_str = User{
        id: Option::Some(27),
        name: name.to_string()
    };

    return serde_json::to_string(&user_str).unwrap();
}

#[post("/api/v1/user")]
async fn post_method(data: web::Json<User>) -> impl Responder {
    format!("Created user with id {} and name {}", data.id.unwrap_or_default(), data.name)
}

#[put("/api/v1/user")]
async fn update_method(data: web::Json<User>) -> impl Responder {
    let id = data.id.unwrap_or_default();

    if id == 0 {
        return "Unable to update user, id is not present into request.".to_string();
    }

    format!("Updated structure with id {} and name {}", id, data.name)
}

#[delete("/api/v1/user/{id}")]
async fn delete_method(id: web::Path<i32>) -> impl Responder {
    let id = id.into_inner();

    if id == 0 {
        return "Unable to update user, id is not valid identifier. Must be greater than 0.".to_string();
    }

    format!("Removed structure with id {}", id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_method)
            .service(post_method)
            .service(update_method)
            .service(delete_method)
    })
        .bind(("127.0.0.1", 8088))?
        .run()
        .await
}
