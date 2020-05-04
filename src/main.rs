extern crate chrono;
extern crate hyper;
extern crate reqwest;
extern crate scraper;
extern crate serde_json;

use chrono::{Duration, Utc};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Method, Response, Server, StatusCode};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};

mod contest_service;

fn main() {
    let last_requested_time = Arc::new(Mutex::new(Utc::now()));
    let new_svc = move || {
        let last_requested_time = last_requested_time.clone();
        service_fn_ok(move |req| match (req.method(), req.uri().path()) {
            (&Method::GET, "/json") => {
                let last_time = { *last_requested_time.lock().unwrap() };
                let mut response_json: Map<String, Value> = Map::new();

                let duration = Utc::now() - last_time;
                if duration >= Duration::milliseconds(200) {
                    if let Some(query) = req.uri().query() {
                        let mut url_query_params: HashMap<&str, &str> = HashMap::new();
                        for pair in query.split('&') {
                            let tmp = pair.split('=').collect::<Vec<_>>();
                            if tmp.len() != 2 {
                                continue;
                            }
                            url_query_params.insert(tmp[0], tmp[1]);
                        }

                        println!(
                            "{} # url_query_params: {:?}",
                            chrono::Utc::now(),
                            url_query_params
                        );

                        for (service_name, handle) in url_query_params.into_iter() {
                            match contest_service::from_name(service_name) {
                                Some(service) => {
                                    let rating_opt = service.get_rating(handle);

                                    let mut content = Map::new();
                                    match rating_opt {
                                        Some(rating) => {
                                            content.insert(
                                                format!("status"),
                                                Value::String(format!("success")),
                                            );
                                            content.insert(
                                                format!("rating"),
                                                Value::Number(serde_json::Number::from(
                                                    rating.value,
                                                )),
                                            );
                                            content.insert(
                                                format!("color"),
                                                Value::String(rating.color.to_string()),
                                            );
                                        }
                                        None => {
                                            content.insert(
                                                format!("status"),
                                                Value::String(format!("error")),
                                            );
                                        }
                                    }
                                    response_json
                                        .insert(service.name().to_string(), Value::Object(content));
                                }
                                _ => {}
                            }
                        }
                    } else {
                        response_json
                            .insert(format!("error"), Value::String(format!("empty query")));
                    }
                } else {
                    response_json.insert(
                        format!("error"),
                        Value::String(format!(
                            "try again: latest request is {} ms ago",
                            duration.num_milliseconds()
                        )),
                    );
                }

                *last_requested_time.lock().unwrap() = Utc::now();

                Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "text/json")
                    .header("Access-Control-Allow-Origin", "*")
                    .body(Body::from(
                        serde_json::to_string_pretty(&Value::Object(response_json)).unwrap(),
                    ))
                    .unwrap()
            }
            (method, path) => {
                println!(
                    "{} # 404: method:{:?} path:{:?}",
                    chrono::Utc::now(),
                    method,
                    path
                );
                Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .header("Content-Type", "text/plain")
                    .header("Access-Control-Allow-Origin", "*")
                    .body(Body::from("Not found"))
                    .unwrap()
            }
        })
    };

    let port = env::var("PORT")
        .ok()
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(3000);
    let addr = ([0, 0, 0, 0], port).into();

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("{} # start serving !", chrono::Utc::now());
    hyper::rt::run(server);
}
