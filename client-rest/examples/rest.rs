//! Example communication with this service

use chrono::Utc;
use hyper::{Body, Client, Method, Request, Response};
use hyper::{Error, StatusCode};
use std::env;
use svc_template_rust_client_rest::types::*;

/// Provide endpoint url to use
pub fn get_rest_endpoint() -> String {
    //parse socket address from env variable or take default value
    let address = match env::var("SERVER_HOSTNAME") {
        Ok(val) => val,
        Err(_) => "localhost".to_string(), // default value
    };

    let port = match env::var("SERVER_PORT_REST") {
        Ok(val) => val,
        Err(_) => "8080".to_string(), // default value
    };

    format!("http://{}:{}", address, port)
}

fn evaluate(resp: Result<Response<Body>, Error>, expected_code: StatusCode) -> (bool, String) {
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("NOTE: Ensure the server is running, or this example will fail.");
    let rest_endpoint = get_rest_endpoint();

    let mut ok = true;
    let client = Client::builder()
        .pool_idle_timeout(std::time::Duration::from_secs(10))
        .build_http();

    // POST /template/example
    {
        let data = ExampleRequest {
            id: "abcdef12".to_string(),
            timestamp: Utc::now(),
        };

        let data_str = serde_json::to_string(&data).unwrap();
        let uri = format!("{}/template/example", rest_endpoint);
        let req = Request::builder()
            .method(Method::POST)
            .uri(uri.clone())
            .header("content-type", "application/json")
            .body(Body::from(data_str))
            .unwrap();

        let resp = client.request(req).await;
        let (success, result_str) = evaluate(resp, StatusCode::OK);
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
