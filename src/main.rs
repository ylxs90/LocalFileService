mod core;
mod server;

use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use clap::Parser;
use colored::{Color, Colorize};
use futures::StreamExt;
use get_if_addrs::get_if_addrs;
use std::collections::HashMap;
use std::io::{stdout, Write};
use std::net::IpAddr;
use std::str::FromStr;

use crate::Mode::{Download, Upload};

const DEFAULT_IP_IDX: usize = 1;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Params {
    #[clap(index = 1, default_value = ".")]
    path: String,
    /// if port < 1024, use random port
    #[arg(short, long, default_value = "8080")]
    port: u16,
    /// download, upload
    #[arg(short, long, default_value = "download")]
    mode: Mode,
    /// bind the IP address which you choose, not bind 0.0.0.0
    #[arg(short, long, default_value = "false")]
    bind_choose_ip: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Download,
    Upload,
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "download" => Ok(Download),
            "upload" => Ok(Upload),
            _ => Err(format!("error to parse {} for Mode", s)),
        }
    }
}

#[actix_web::main]
async fn main() -> Result<()> {
    let param = Params::parse();
    if param.path == "." {
        println!("start local server for current dir");
    }
    println!("{:?}", param);

    let mut ips = HashMap::new();
    let mut ip_index = 0;
    if let Ok(interfaces) = get_if_addrs() {
        for interface in interfaces.iter() {
            ips.insert(ip_index, interface.ip().to_string());
            ip_index += 1;
        }
    } else {
        eprintln!("Failed to get network interfaces");
    }

    let mut bind_ip = String::new();
    loop {
        println!("IP list:");
        for n in 0..ip_index {
            println!("[{n:0>2}] {}", ips.get(&n).unwrap());
        }
        
        print!(
            "choose your access IP address:{}{}{}",
            "[".bold(),
            DEFAULT_IP_IDX,
            "]".bold()
        );
        stdout().flush()?;

        let mut buf = String::new();
        if let Ok(s) = std::io::stdin().read_line(&mut buf) {
            buf = buf.trim().to_string();
            if buf.is_empty() {
                bind_ip = ips.get(&DEFAULT_IP_IDX).unwrap().to_string();
                break;
            }
            if let Ok(no) = buf.parse::<usize>() {
                if ips.contains_key(&no) {
                    bind_ip = ips.get(&no).unwrap().to_string();
                    break;
                }
            }
        }
        print!("\x1B[2J\x1B[1;1H");
        stdout().flush().unwrap();
        eprintln!("only allows 0~{}, but got input {buf}", ip_index - 1);
    }

    let access_base_uri = if bind_ip.parse::<IpAddr>()?.is_ipv6() {
        format!("[{bind_ip}]:{}", param.port)
    } else {
        format!("{bind_ip}:{}", param.port)
    };
    println!(
        "uri: {}",
        format!("http://{}", access_base_uri).color(Color::BrightBlue)
    );
    qr2term::print_qr(format!("http://{access_base_uri}")).unwrap();

    std::fs::create_dir_all("./uploads")?;
    HttpServer::new(move || {
        let app = App::new().route("/", web::get().to(index));
        match param.mode {
            Upload => app.route("/upload", web::post().to(upload)),
            Download => {
                app.service(actix_files::Files::new("/files", "./uploads").show_files_listing())
            }
        }
    })
    .bind((bind_ip.as_str(), param.port))?
    .run()
    .await?;

    Ok(())
}

async fn index() -> impl Responder {
    NamedFile::open("static/index.html")
}

async fn upload(mut payload: Multipart) -> impl Responder {
    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();
        let content_disposition = field.content_disposition();
        let filename = content_disposition.get_filename().unwrap();
        let filepath = format!("./uploads/{}", sanitize_filename::sanitize(&filename));

        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap()
            .unwrap();
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f = web::block(move || f.write_all(&data).map(|_| f))
                .await
                .unwrap()
                .unwrap();
        }
    }
    HttpResponse::Ok().body("File uploaded successfully")
}
