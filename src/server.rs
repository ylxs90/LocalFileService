use std::path::PathBuf;

pub mod server {
    use std::io::Write;
    use std::net::{SocketAddr, TcpListener};
    use std::path::PathBuf;
    use std::thread::sleep;
    use std::time::Duration;

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



}


macro_rules! file_to_html {
    ($fmt:expr, $path:expr) => {{
        // Hardcoded content; replace this with file reading or other logic if needed
        let path = $path as &PathBuf;
        let mut generated_html = String::new();
         match path.read_dir() {
            Ok(dir) => {
                dir.into_iter().for_each(|d| {
                    // println!("{:?}", d.unwrap());
                    let  data = format!("<td>{}</td>\n", d.unwrap().file_name().to_str().unwrap());
                    generated_html.push_str(&data);
                })
            }
            Err(_) => {
                println!("error", );
            }
        }



        // Replace the `{}` in the format string with the generated HTML
        format!($fmt, generated_html)
    }};
}
#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use std::path::PathBuf;
    use crate::Mode::Upload;
    use crate::server::server::{FileServer, Server};

    #[test]
    fn test_serve() {
        let addr: SocketAddr = "0.0.0.0:1337".parse().unwrap();
        let server: Server = (&Upload, &addr, &".".into()).into();
        println!("{:?}", server);
        server.serve();
    }

    #[test]
    fn test_marco() {
        // ll !/
        // aa
        // bb
        let mut path = PathBuf::from("./");

        // match path.read_dir() {
        //     Ok(dir) => {
        //         dir.into_iter().for_each(|d| {
        //             let entry = d.unwrap();
        //
        //             println!("{:?}", entry.path());
        //         })
        //     }
        //     Err(e) => {
        //         println!("{}", e);
        //     }
        // }


        let x = file_to_html!("<tr>\n{}</tr>\n", &path);



        println!("{}", x);
    }


}

