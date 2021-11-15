mod cognite;
mod graph;
mod ssm;

#[macro_use]
extern crate juniper;

use crate::graph::*;
use actix_web::http::HeaderMap;
use juniper_actix::{graphql_handler, playground_handler};
use lambda_web::actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use lambda_web::{is_running_on_lambda, run_actix_on_lambda, LambdaError};
use std::env;
use std::str::FromStr;

#[actix_web::main]
async fn main() -> Result<(), LambdaError> {
    ssm::load_env().await;

    let app = move || {
        let schema = create_schema();

        // TODO: data -> app_dataに移行する
        // ApiGatewayにドメインを設定しないのでステージ名がパスに入る
        App::new()
            .data(schema)
            .service(
                web::resource("/default/graphql")
                    .route(web::post().to(graphql_route))
                    .route(web::get().to(graphql_route)),
            )
            .service(web::resource("/default/playground").route(web::get().to(playground_route)))
            .service(
                web::resource("/default/health_check").route(web::get().to(health_check_route)),
            )
    };

    if is_running_on_lambda() {
        run_actix_on_lambda(app).await?;
    } else {
        let port = env::var("PORT").unwrap_or("3000".to_string());
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
    playground_handler("/default/graphql", None).await
}

async fn graphql_route(
    req: HttpRequest,
    payload: web::Payload,
    schema: web::Data<Schema>,
) -> actix_web::Result<HttpResponse> {
    let authenticate_id = authenticate(&req).await;
    let context = Context::new(authenticate_id).await;
    graphql_handler(&schema, &context, req, payload).await
}

async fn authenticate(req: &HttpRequest) -> Option<String> {
    if let Some(id) = get_into(req.headers(), "x-user-id") {
        return Some(id);
    }

    let token: &str = get(req.headers(), "authorization")?;
    if token.len() < 7 {
        return None;
    }

    let result = cognite::verify_token(&token[7..]).await;
    if let Err(_err) = result {
        return None;
    }

    Some(result.ok().unwrap().unwrap())
}

fn get_into<T>(headers: &HeaderMap, key: &str) -> Option<T>
where
    T: FromStr,
{
    headers.get(key)?.to_str().ok()?.parse::<T>().ok()
}

fn get<'a, 'b>(headers: &'a HeaderMap, key: &'b str) -> Option<&'a str> {
    headers.get(key)?.to_str().ok()
}
