use std::net::{SocketAddr, TcpListener};
use std::path::PathBuf;

pub trait FileServer {
    fn serve(&self);
    fn socket(&self) -> SocketAddr;
    fn path(&self) -> PathBuf;
}

pub enum Server {
    UploadServer(SocketAddr, PathBuf),
    DownloadServer(SocketAddr, PathBuf),
}


fn upload_serve(socket_addr: SocketAddr, path: PathBuf) {
    let listener = TcpListener::bind(socket_addr).unwrap();




}

fn download_serve(socket_addr: SocketAddr, path: PathBuf) {}

impl FileServer for Server {
    fn serve(&self) {
        match self {
            Server::UploadServer(_, _) => upload_serve(self.socket(), self.path()),
            Server::DownloadServer(_, _) => download_serve(self.socket(), self.path()),
        }
    }

    fn socket(&self) -> SocketAddr {
        match self {
            Server::UploadServer(socket, _) => { socket.clone() }
            Server::DownloadServer(socket, _) => { socket.clone() }
        }
    }

    fn path(&self) -> PathBuf {
        match self {
            Server::UploadServer(_, path) => { path.clone() }
            Server::DownloadServer(_, path) => { path.clone() }
        }
    }
}




