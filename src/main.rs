use hyper::{
    header::HeaderValue,
    service::{make_service_fn, service_fn},
    Body, Client, Request, Response, Server,
};
use hyperlocal::{UnixClientExt, Uri};
use std::{error::Error, net::SocketAddr, path::Path};

async fn handle_request(
    _req: Request<Body>,
) -> Result<Response<Body>, Box<dyn Error + Send + Sync>> {
    let path = Path::new("/var/run/docker.sock");

    if !path.exists() {
        return Ok(Response::new(Body::from(
            "Docker daemon not available - is it running?",
        )));
    }

    let req_path = _req.uri().path();
    let uri = Uri::new("/var/run/docker.sock", req_path).into();

    let client = Client::unix();
    let resp = client.get(uri).await?;

    println!("Response from {}: {}", req_path, resp.status());

    let (parts, body) = resp.into_parts();
    let mut response = Response::from_parts(parts, body);

    response
        .headers_mut()
        .insert("Access-Control-Allow-Origin", HeaderValue::from_static("*"));

    Ok(response)
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to Ctrl+C out of server");
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Box<dyn Error + Send + Sync>>(service_fn(handle_request))
    });

    let server = Server::bind(&addr).serve(make_svc);
    println!("Server listening on http://{}", addr);

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}
