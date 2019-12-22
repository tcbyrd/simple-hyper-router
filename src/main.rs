use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

// Test router that prints out all headers and values,
// the request URI, and any query arguments
async fn request_router(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response_body = String::from("");
    for header in _req.headers().keys() {
        let header_name = header.as_str();
        let header_value = _req.headers().get(header).unwrap();
        let header_string = format!("{}: {:#?}\n", header_name, header_value);
        response_body.push_str(&header_string);
    }

    let path = _req.uri().path();
    let path_string = format!("Path: {} \n", &path);
    response_body.push_str(&path_string);
    
    let query = _req.uri().query().unwrap_or_default();
    let query_string = format!("Query: {} \n", &query);
    response_body.push_str(&query_string);

    Ok(Response::new(response_body.into()))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let router_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(request_router))
    });

    let server = Server::bind(&addr).serve(router_svc);

    println!("Running server on http://{}", addr);
    
    server.await?;

    Ok(())
}
