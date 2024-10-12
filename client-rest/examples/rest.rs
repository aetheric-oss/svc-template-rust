//! Example communication with this service

use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::{
    body::{Bytes, Incoming},
    Error, Method, Request, Response, StatusCode,
};
use hyper_util::rt::TokioIo;
use lib_common::grpc::get_endpoint_from_env;
use lib_common::time::Utc;
use std::convert::Infallible;
use svc_template_rust_client_rest::types::*;
use tokio::net::TcpStream;

fn evaluate(resp: Result<Response<Incoming>, Error>, expected_code: StatusCode) -> (bool, String) {
    let mut ok = true;
    let result_str: String = match resp {
        Ok(r) => {
            let tmp = r.status() == expected_code;
            ok &= tmp;
            println!("{:?}", r.body());

            r.status().to_string()
        }
        Err(e) => {
            ok = false;
            e.to_string()
        }
    };

    (ok, result_str)
}
fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, Infallible> {
    Full::new(chunk.into()).boxed()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("NOTE: Ensure the server is running, or this example will fail.");

    let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_REST");
    let addr = format!("{}:{}", host, port);

    println!("Rest endpoint set to [{}].", addr);

    let stream = TcpStream::connect(addr.clone()).await?;
    let io = TokioIo::new(stream);

    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let mut ok = true;

    // POST /template/example
    {
        let data = ExampleRequest {
            id: "abcdef12".to_string(),
            timestamp: Utc::now(),
        };

        let data_str = serde_json::to_string(&data).unwrap();
        let uri = format!("http://{}/template/example", addr);
        let req = Request::builder()
            .method(Method::POST)
            .uri(uri.clone())
            .header("content-type", "application/json")
            .body(full(data_str))
            .unwrap();

        let res = sender.send_request(req).await;
        let (success, result_str) = evaluate(res, StatusCode::OK);
        ok &= success;

        println!("{}: {}", uri, result_str);
    }

    if ok {
        println!("\u{1F9c1} All endpoints responded!");
    } else {
        eprintln!("\u{2620} Errors");
    }

    Ok(())
}
