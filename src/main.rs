mod api;
mod models;
mod repository;

use actix_cors::Cors;
//modify imports below
use actix_web::{web::Data, App, HttpServer};
use api::user_api::{create_user, delete_user, get_all_users, get_user, update_user}; //import the handler here
use repository::mongodb_repo::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["POST", "PUT", "PATCH", "GET", "OPTIONS", "HEAD"]);
        App::new()
            .app_data(db_data.clone())
            .wrap(cors)
            .service(create_user)
            .service(get_user)
            .service(update_user)
            .service(delete_user)
            .service(get_all_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
