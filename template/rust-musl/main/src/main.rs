use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;

use handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

  let make_svc = make_service_fn(|_| async {
    Ok::<_, Infallible>(service_fn(handler::handle))
  });

  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  let server = Server::bind(&addr).serve(make_svc);

  println!("Listening on http://{}", addr);

  if let Err(e) = server.await {
    eprintln!("server error: {}", e);
  }

  Ok(())
}
