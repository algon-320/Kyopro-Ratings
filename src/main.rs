mod cache;
mod contest_service;
mod util;

use chrono::prelude::*;
use std::net::SocketAddr;

use hyper::{Body, Method, Request, Response, Server, StatusCode};
use std::convert::Infallible;

#[tokio::main]
async fn main() {
    let addr = {
        let port = std::env::var("PORT")
            .ok()
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(3000);
        SocketAddr::from(([0, 0, 0, 0], port))
    };

    let make_svc = hyper::service::make_service_fn(|_conn| async {
        Ok::<_, Infallible>(hyper::service::service_fn(router))
    });
    let server = Server::bind(&addr).serve(make_svc);

    println!("{} # start listening on {}", Utc::now(), &addr);
    if let Err(err) = server.await {
        eprintln!("server error: {err}");
    }
}

use serde_json::{Map, Value};

async fn router(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/json") if req.uri().query().is_some() => {
            let query_string = req.uri().query().unwrap();

            let mut response_json: Map<String, Value> = Map::new();
            for key_value in query_string.split('&') {
                let mut parts = key_value.split('=');
                if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                    process_query(key, value, &mut response_json).await;
                }
            }

            let body = serde_json::to_string_pretty(&Value::Object(response_json)).unwrap();

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/json")
                .header("Access-Control-Allow-Origin", "*")
                .body(Body::from(body))
                .unwrap())
        }

        (&Method::GET, "/") => Ok(Response::builder()
            .status(StatusCode::SEE_OTHER)
            .header("Location", "https://github.com/algon-320/Kyopro-Ratings")
            .body(Body::empty())
            .unwrap()),

        (method, path) => {
            println!(
                "{} # 404: method:{:?}, path:{:?}, query:{:?}",
                Utc::now(),
                method,
                path,
                req.uri().query()
            );

            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header("Content-Type", "text/plain")
                .header("Access-Control-Allow-Origin", "*")
                .body(Body::from("Not found"))
                .unwrap())
        }
    }
}

async fn process_query(
    service_name: &str,
    handle_name: &str,
    response_json: &mut Map<String, Value>,
) {
    let rating_opt = contest_service::get_rating(service_name, handle_name).await;

    let mut content = Map::new();
    match rating_opt {
        Some(rating) => {
            content.insert(format!("status"), Value::String(format!("success")));
            content.insert(
                format!("rating"),
                Value::Number(serde_json::Number::from(rating.value)),
            );
            content.insert(format!("color"), Value::String(rating.color.to_string()));
        }
        None => {
            content.insert(format!("status"), Value::String(format!("error")));
        }
    }
    response_json.insert(service_name.to_owned(), Value::Object(content));
}
