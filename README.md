# rust-http-template

An OpenFaaS of-watchdog template written in Rust.

This repository contains two Rust templates for OpenFaaS, one of which gives additional control over the HTTP request and response. They will both handle higher throughput than the classic watchdog due to the process being kept warm.

```sh
$ faas template pull https://github.com/egavrilov/rust-musl-template
$ faas new --list
Languages available as templates:
- rust-musl
```

## rust-http-template/rust-musl

This template gives you more control over handling function input and output.

```Rust
use std::convert::Infallible;
use hyper::{Body, Request, Response};

const PHRASE: &str = "Hello, World!";

pub async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from(PHRASE)))
}

// Returns:
// Hello, World!
```

You can return custom errors using `hyper::Response`.

```Rust
use std::convert::Infallible;
use hyper::{Body, Request, Response, StatusCode};

const PHRASE: &str = "Hello, World!";

pub async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from("my error"))
        .unwrap())
}

// Returns:
// my error
```
