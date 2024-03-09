pub mod server {
    use std::convert::Infallible;
    use std::fmt::format;
    use std::io::Write;
    use std::net::{SocketAddr, TcpListener};
    use std::path::PathBuf;
    use std::thread::sleep;
    use std::time::Duration;
    use bytes::Bytes;
    use http_body_util::Full;
    use hyper::body::Body;
    use hyper::{Request, Response, StatusCode};

    use crate::Mode;
    use crate::Mode::Download;
    use crate::server::server::Server::{DownloadServer, UploadServer};



    pub trait FileServer {
        fn serve(&self) {
            let listener = TcpListener::bind(self.addr()).unwrap();
            let (mut stream, _) = listener.accept().unwrap();
            let x = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>upload</title>
</head>
<body>

    <table>
<tr>
<td><a href="./file">{}</a></td>
</tr>
</table>

</body>
</html>"#, self.path().file_name().unwrap().to_str().unwrap());
            stream.write_all(x.as_bytes()).unwrap()
        }

        fn addr(&self) -> &SocketAddr;

        fn path(&self) -> &PathBuf;
    }

    #[derive(Debug)]
    pub enum Server {
        UploadServer {
            addr: SocketAddr,
            path: PathBuf,
        },
        DownloadServer {
            addr: SocketAddr,
            path: PathBuf,
        },
    }

    impl From<(&Mode, &SocketAddr, &PathBuf)> for Server {
        fn from(value: (&Mode, &SocketAddr, &PathBuf)) -> Self {
            match value.0 {
                Mode::Download => {
                    DownloadServer { addr: value.1.clone(), path: value.2.clone() }
                }
                Mode::Upload => {
                    UploadServer { addr: value.1.clone(), path: value.2.clone() }
                }
            }
        }
    }

    impl From<(&SocketAddr, &PathBuf)> for Server {
        fn from(value: (&SocketAddr, &PathBuf)) -> Self {
            (&Download, value.0, value.1).into()
        }
    }

    impl FileServer for Server {
        fn serve(&self) {
            sleep(Duration::from_secs(600))
        }

        fn addr(&self) -> &SocketAddr {
            match self {
                UploadServer { addr, path: _path } => { addr }
                DownloadServer { addr, path: _path } => { addr }
            }
        }

        fn path(&self) -> &PathBuf {
            match self {
                UploadServer { addr: _addr, path } => { path }
                DownloadServer { addr: _addr, path } => { path }
            }
        }
    }


    async fn handle_upload(req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
        if req.uri().path() != "/file" {
            return download_page(req);
        }
        todo!()
    }

    async fn download_page(req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
        let response = Response::builder()
            .header("Content-Type", "text/html")
            .status(StatusCode::OK)
            .body(Full::new(Bytes::from(""))).unwrap();

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use std::thread::{sleep, spawn};
    use std::time::Duration;

    use crate::server::server::{FileServer, Server};

    #[test]
    fn test_serve() {
        let addr: SocketAddr = "0.0.0.0:1337".parse().unwrap();
        let server: Server = (&addr, &".".into()).into();
        println!("{:?}", server);
        server.serve();
    }
}

