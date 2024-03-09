mod server;

use std::collections::HashMap;
use std::io::{stdout, Write};
use std::str::FromStr;

use anyhow::Result;
use clap::Parser;
use colored::{Color, Colorize};
use get_if_addrs::get_if_addrs;

use crate::Mode::{Download, Upload};

const DEFAULT_IP_IDX: usize = 1;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Params {
    #[clap(index = 1, default_value = ".")]
    path: String,
    /// if port < 1024, use random port
    #[arg(short, long, default_value = "0")]
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
        return match s.to_lowercase().as_str() {
            "download" => {
                Ok(Download)
            }
            "upload" => {
                Ok(Upload)
            }
            _ => { Err(format!("error to parse {} for Mode", s)) }
        };
    }
}

fn main() -> Result<()> {
    let param = Params::parse();
    if param.path == "." {
        println!("start local server for current dir", );
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
        // println!("{:#?}", ips);
        for n in 0..ip_index {
            println!("[{n:0>2}] {}", ips.get(&n).unwrap());
        }
        print!("choose your access IP address:{}{}{}", "[".bold(), DEFAULT_IP_IDX, "]".bold());
        stdout().flush().unwrap();

        let mut buf = String::new();
        if let Ok(s) = std::io::stdin().read_line(&mut buf) {
            buf = buf.trim().to_string();
            if buf.is_empty() {
                // using default (key = 1) ip address
                // because key = 0 usually is 127.0.0.1, this program was supposed to provide a LAN file service
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
    let access_base_uri = format!("{bind_ip}:{}", param.port);
    println!("uri: {}", format!("http://{}", access_base_uri).color(Color::BrightBlue));
    qr2term::print_qr(format!("http://{access_base_uri}")).unwrap();


    Ok(())
}