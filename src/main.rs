mod differ;

use core::convert::Infallible;

use differ::Differ;
use hyper::service::make_service_fn;
use hyper::service::service_fn;
use hyper::Body;
use hyper::Request;
use hyper::Response;
use hyper::Server;
use std::collections::HashMap;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(endpoint))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn endpoint(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let params: HashMap<String, String> = req
        .uri()
        .query()
        .map(|v| {
            url::form_urlencoded::parse(v.as_bytes())
                .into_owned()
                .collect()
        })
        .unwrap_or_else(HashMap::new);

    if !params.contains_key("a") || !params.contains_key("b") {
        // return 400 bad request
        return Ok(Response::builder()
            .status(400)
            .body(Body::from("missing a or b query parameters"))
            .unwrap());
    }

    // Check if diff mode is defined and valid.
    let diff_mode = if params.contains_key("mode") {
        let mode = params.get("mode").unwrap();
        match mode.as_str() {
            "line" => differ::DiffMode::Line,
            "word" => differ::DiffMode::Word,
            "char" => differ::DiffMode::Char,
            _ => {
                return Ok(Response::builder()
                    .status(400)
                    .body(Body::from("invalid mode"))
                    .unwrap())
            }
        }
    } else {
        differ::DiffMode::Line
    };

    let a_input = params.get("a").unwrap();
    let a = decode_param(a_input);
    let b_input = params.get("b").unwrap();
    let b = decode_param(b_input);

    let differ = Differ::new(diff_mode);
    let (different, output) = differ.generate(a, b);

    if different {
        return Ok(Response::builder()
            .status(200)
            .body(Body::from(output))
            .unwrap());
    }

    Ok(Response::builder()
        .status(200)
        .body(Body::from("input is the same"))
        .unwrap())
}

fn decode_param(param: &str) -> String {
    let bytes = base64::decode(param).unwrap();
    let ret = std::str::from_utf8(&bytes).unwrap();
    ret.to_string()
}
