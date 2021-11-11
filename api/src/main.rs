mod graph;

#[macro_use]
extern crate juniper;

use crate::graph::*;
use juniper_actix::{graphql_handler, playground_handler};
use lambda_web::actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use lambda_web::{is_running_on_lambda, run_actix_on_lambda, LambdaError};

#[actix_web::main]
async fn main() -> Result<(), LambdaError> {
    let app = move || {
        let schema = create_schema();

        // TODO: data -> app_dataに移行する
        App::new()
            .data(schema)
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql_route))
                    .route(web::get().to(graphql_route)),
            )
            .service(web::resource("/playground").route(web::get().to(playground_route)))
            .service(web::resource("/health_check").route(web::get().to(health_check_route)))
    };

    if is_running_on_lambda() {
        println!("running on lambda");
        run_actix_on_lambda(app).await.map_err(|e| {
            println!("got error when running lambda {:?}", e);
            e
        })?;
    } else {
        let port = 3000;
        println!("running on local, port={}", port);
        HttpServer::new(app)
            .bind(format!("127.0.0.1:{}", port))?
            .run()
            .await?;
    }

    Ok(())
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
    let context = Context::new().await;
    graphql_handler(&schema, &context, req, payload).await
}
