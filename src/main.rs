extern crate hyper;
extern crate reqwest;
extern crate scraper;
extern crate serde_json;

use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Method, Response, Server, StatusCode};
use serde_json::{Map, Value};
use std::collections::HashMap;

mod contest_service;
use contest_service::ContestService;

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let new_svc = || {
        service_fn_ok(|_req| match (_req.method(), _req.uri().path()) {
            (&Method::GET, "/json") => {
                let mut response_json: Map<String, Value> = Map::new();

                if let Some(query) = _req.uri().query() {
                    let mut url_query_params: HashMap<&str, &str> = HashMap::new();
                    for pair in query.split('&') {
                        let tmp = pair.split('=').collect::<Vec<_>>();
                        if tmp.len() != 2 {
                            continue;
                        }
                        url_query_params.insert(tmp[0], tmp[1]);
                    }

                    println!("url_query_params: {:?}", url_query_params);

                    for (service_name, handle) in url_query_params.into_iter() {
                        match ContestService::from_name(service_name) {
                            Some(ref service) => {
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
                                            Value::Number(serde_json::Number::from(rating)),
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
                    response_json.insert(format!("error"), Value::String(format!("empty query")));
                }

                Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "text/json")
                    .body(Body::from(format!("{}", Value::Object(response_json))))
                    .unwrap()
            }
            (_, _) => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Not found"))
                .unwrap(),
        })
    };
    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("now serving !");
    hyper::rt::run(server);
}
