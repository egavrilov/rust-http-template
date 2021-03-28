use std::convert::Infallible;
use hyper::{Body, Request, Response};

const PHRASE: &str = "Hello, World!";

pub async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from(PHRASE)))
}
