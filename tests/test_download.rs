#[cfg(test)]
mod test_download {
    use std::convert::Infallible;
    use std::net::SocketAddr;
    use std::sync::{Arc, Mutex};
    use bytes::Bytes;
    use http_body_util::Full;
    use hyper::{Request, Response, StatusCode};
    use hyper::body::Incoming;
    use hyper::server::conn::http1;
    use hyper::service::service_fn;
    use hyper_util::rt::TokioIo;
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn test() -> anyhow::Result<()> {
        let addr: SocketAddr = "0.0.0.0:1337".parse().unwrap();


        let file = Arc::new(Mutex::new("hello world".to_string()));
        loop {
            let listener = TcpListener::bind(&addr).await?;
            let (stream, _) = listener.accept().await.unwrap();
            let stream = TokioIo::new(stream);

            tokio::spawn(
                async move {
                    http1::Builder::new().serve_connection(stream, service_fn( ))
                });
        }


        Ok(())
    }

    async fn handle_download(req: Request<Incoming>, file: Arc<Mutex<String>>) -> Result<Response<Full<Bytes>>, Infallible> {
        download_page(req, file).await
    }

    async fn download_page(req: Request<hyper::body::Incoming>, file: Arc<Mutex<String>>) -> Result<Response<Full<Bytes>>, Infallible> {
        let data = file.lock();
        let response = Response::builder()
            .header("Content-Type", "text/html")
            .status(StatusCode::OK)
            .body(Full::new(Bytes::from(format!("<h1>{}</h1>", data.unwrap().to_string())))).unwrap();

        Ok(response)
    }
}