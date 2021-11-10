mod graph;

#[macro_use]
extern crate juniper;

use crate::graph::*;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use juniper_actix::{graphql_handler, playground_handler};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 3000;

    HttpServer::new(|| {
        let schema = create_schema();

        App::new()
            .data(schema)
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql_route))
                    .route(web::get().to(graphql_route)),
            )
            .service(web::resource("/playground").route(web::get().to(playground_route)))
            .service(web::resource("/health_check").route(web::get().to(health_check_route)))
    })
    .bind(format!("127.0.0.1:{}", port))
    .unwrap()
    .run()
    .await
}

async fn health_check_route() -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("ok"))
}

async fn playground_route() -> actix_web::Result<HttpResponse> {
    playground_handler("/graphql", None).await
}

async fn graphql_route(
    req: HttpRequest,
    payload: web::Payload,
    schema: web::Data<Schema>,
) -> actix_web::Result<HttpResponse> {
    let context = Context::new();
    graphql_handler(&schema, &context, req, payload).await
}
