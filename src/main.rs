use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use tokio::net::TcpListener;
use hyper_util::rt::TokioIo;


async fn handle_redirect(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Starting proxy server");
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    let Ok(listener) = TcpListener::bind(&addr).await else {
        eprintln!("Failed to bind to address: {}", addr);
        return Err("Failed to bind to address".into());
    };

    loop {
        let (stream, _) = listener.accept().await?;

        // use an adapter to access something implementing `tokio::io` traits
        let io = TokioIo::new(stream);

        // multiple connections concurrently
        tokio::task::spawn(async move {
            // Finally, we bind the incoming connection to our `hello` service
            if let Err(err) = http1::Builder::new()
            // `service_fn` converts our function in a `Service`
                .serve_connection(io, service_fn(redirect))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
    
}
