#[cfg(test)]
mod test_server {
    use std::convert::Infallible;
    use std::future::Future;
    use std::net::SocketAddr;
    use http_body_util::combinators::BoxBody;
    use http_body_util::{BodyExt, Full, StreamBody};
    use hyper::{Method, Request, Response, StatusCode};
    use hyper::body::{Bytes, Frame};
    use hyper::server::conn::{http1, http2};
    use hyper::service::service_fn;
    use hyper_util::rt::TokioIo;
    use tokio::fs::File;
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn test_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let addr: SocketAddr = "0.0.0.0:1337".parse().unwrap();

        // We create a TcpListener and bind it to 127.0.0.1:3000
        let listener = TcpListener::bind(addr).await?;

        // We start a loop to continuously accept incoming connections
        loop {
            let (stream, _) = listener.accept().await?;

            // Use an adapter to access something implementing `tokio::io` traits as if they implement
            // `hyper::rt` IO traits.
            let io = TokioIo::new(stream);

            // Spawn a tokio task to serve multiple connections concurrently
            tokio::task::spawn(async move {
                // Finally, we bind the incoming connection to our `hello` service

                if let Err(err) = http1::Builder::new()
                    // `service_fn` converts our function in a `Service`
                    .serve_connection(io, service_fn(handle_upload))
                    .await
                {
                    println!("Error serving connection: {:?}", err);
                }
            });
        }
    }

    async fn handle_upload(req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
        match (req.method(), req.uri().path()) {
            (&Method::POST, "/upload") => {
                // Get the filename from the Content-Disposition header
                let filename = req.headers()
                    .get("Content-Disposition")
                    .and_then(|header| {
                        header.to_str().ok()
                            .and_then(|header_str| {
                                let filename_start = header_str.find("filename=\"")? + 10;
                                let filename_end = header_str.rfind("\"")?;
                                Some(&header_str[filename_start..filename_end])
                            })
                    })
                    .unwrap_or("uploaded_file.txt");

                req.headers().iter().for_each(|(n,v)| {
                    println!("{}  -->  {:?}", n, v);
                });


                // Get the body of the request
                let mut x = req.boxed();
                println!("------------------------------------", );
                println!("{:?}", x);
                

                // Write the body to a file with the extracted filename
                // let mut file = File::create(filename)?;
                // file.write_all(&full_body)?;

                // Ok(Response::new(Body::from("File uploaded successfully")))
            }
            _ => {
                // Ok(Response::builder()
                //     .status(404)
                //     .body(Body::from("Not Found"))
                //     .unwrap())
            }
        }
        return upload_html();
    }

    fn upload_html() -> Result<Response<Full<Bytes>>, Infallible> {
        let response = Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "text/html")
            .body(Full::new(Bytes::from(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>upload</title>
</head>
<body>

    <form action="/upload" method="post" enctype="multipart/form-data">
        <input type="file" name="upload_file">
        <input type="submit">
    </form>

</body>
</html>"#)))
            .unwrap();
        return Ok(response);
    }
}