#![feature(is_some_and)]

use futures_util::future;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};

use dw_server::redis_conn::RedisConn;
use metrics_types::MetricsAlarmType;

async fn handle_json_body(data: json::JsonValue, redis_conn: Arc<Mutex<RedisConn>>) {
    let tasks: Vec<_> = data
        .members()
        .filter(|&obj| obj.has_key("alarm_type"))
        .map(|obj| async {
            if let Ok(key) = MetricsAlarmType::from_str(&obj["alarm_type"].to_string()) {
                let mut lock = redis_conn.lock().await;
                lock.list_push(&key, obj.dump()).unwrap_or_else(|_err| {
                    // todo add log.
                    println!("handle data error {}", _err.to_string());
                });
            }
        })
        .collect();
    future::join_all(tasks).await;
}

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
async fn handle(
    req: Request<Body>,
    addr: SocketAddr,
    redis_conn: Arc<Mutex<RedisConn>>,
) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => Ok(Response::new(Body::from("DW Server Proxy"))),

        // get public ip
        (&Method::GET, "/api/ip") => {
            let ip = addr.ip().to_string();
            Ok(Response::new(Body::from(ip)))
        }

        (&Method::POST, "/api/alarm") => {
            // println!("header: {:?}", req.headers());
            if !req.headers().contains_key("content-type")
                || req.headers().get("content-type").is_some_and(|value| {
                    value
                        .to_str()
                        .is_ok_and(|str| !str.to_lowercase().contains("application/json"))
                })
            {
                return Ok(unprocessable_entity().unwrap());
            }

            // println!("body: {:?}", req.body());
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;
            let body_str = std::str::from_utf8(whole_body.as_ref()).unwrap_or("");
            let json_body = json::parse(body_str).unwrap_or(json::JsonValue::new_object());
            if json_body.is_empty() {
                // debug log : println origin body
                println!("json parse error or {:?}", whole_body);
                return Ok(unprocessable_entity().unwrap());
            }
            // println!("body content: {:?}", json_body);
            handle_json_body(json_body, redis_conn).await;

            Ok(Response::new(Body::from("ok")))
        }

        // Return the 404 Not Found for other routes.
        _ => Ok(not_found().unwrap()),
    }
}

#[inline]
fn not_found() -> hyper::http::Result<Response<Body>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("404 NOT FOUND"))
}

#[inline]
fn unprocessable_entity() -> hyper::http::Result<Response<Body>> {
    Response::builder()
        .status(StatusCode::UNPROCESSABLE_ENTITY)
        .body(Body::from("Unprocessable Data"))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // let client = redis::Client::open("redis://127.0.0.1/")?;
    // let mut con = client.get_connection()?;

    // let addr = ([127, 0, 0, 1], 3000).into();
    let addr = ([0, 0, 0, 0], 3000).into(); // Todo port args config.

    let rc = RedisConn::new().expect("Create redis connnection error");
    let redis_conn = Arc::new(Mutex::new(rc));

    let make_service = make_service_fn(move |conn: &AddrStream| {
        let addr = conn.remote_addr();
        let redis_conn = Arc::clone(&redis_conn);
        async move {
            let addr = addr.clone();
            let redis_conn = Arc::clone(&redis_conn);
            Ok::<_, hyper::Error>(service_fn(move |req| {
                handle(req, addr.clone(), Arc::clone(&redis_conn))
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_service);

    println!("Proxy Listening on http://{}", addr);

    server.await?;

    Ok(())
}
