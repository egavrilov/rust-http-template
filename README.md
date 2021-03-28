# rust-http-template

An OpenFaaS of-watchdog template written in Rust.

This repository contains two Rust templates for OpenFaaS, one of which gives additional control over the HTTP request and response. They will both handle higher throughput than the classic watchdog due to the process being kept warm.

```sh
$ faas template pull https://github.com/egavrilov/rust-http-template
$ faas new --list
Languages available as templates:
- rust-http
```

## rust-http-template/rust-http

This template gives you more control over handling function input and output.

```Rust
use hyper::{Body, Request, Response};

const PHRASE: &str = "Hello, World!";

pub fn handle(_req: Request<Body>) -> Response<Body> {
    Response::new(Body::from(PHRASE))
}

// Returns:
// Hello, World!
```

You can return custom errors using `hyper::Response`.

```Rust
use hyper::{Body, Request, Response, StatusCode};

pub fn handle(_req: Request<Body>) -> Response<Body> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from("my error"))
        .unwrap()
}

// Returns:
// my error
```
