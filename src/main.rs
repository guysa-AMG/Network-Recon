use clap::Parser;
use std::net::{Ipv4Addr, ToSocketAddrs};
use std::thread;
use tokio::task::JoinHandle;

mod arp;
mod scanner;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
enum Commands {
    Scan {
        #[arg(short, long)]
        target: String,
        #[arg(short, long, default_value = "1-1024")]
        ports: String,
    },
    Arp {
        #[arg(short, long)]
        interface: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Scan {
            target,
            ports,
        } => {
            let ports = parse_ports(ports);
            let socket_addrs = (target.as_str(), 0)
                .to_socket_addrs()
                .expect("Failed to resolve domain");

            for socket_addr in socket_addrs {
                let mut tasks: Vec<JoinHandle<Option<u16>>> = vec![];
                for port in ports.clone() {
                    let task = tokio::spawn(async move {
                        if scanner::scan_port(socket_addr, port).await {
                            Some(port)
                        } else {
                            None
                        }
                    });
                    tasks.push(task);
                }

                let mut open_ports = vec![];
                for task in tasks {
                    if let Some(port) = task.await.unwrap() {
                        open_ports.push(port);
                    }
                }

                println!("Open ports on {}: {:?}", socket_addr.ip(), open_ports);
            }
        }
        Commands::Arp { interface } => {
            let interface = arp::get_interface(interface).expect("Interface not found");
            let source_ip = interface
                .ips
                .iter()
                .find(|ip| ip.is_ipv4())
                .map(|ip| match ip.ip() {
                    std::net::IpAddr::V4(ip) => ip,
                    _ => unreachable!(),
                })
                .unwrap();
            let source_mac = interface.mac.unwrap().into();

            let network_ips: Vec<Ipv4Addr> = interface
                .ips
                .iter()
                .find(|ip| ip.is_ipv4())
                .unwrap()
                .iter()
                .filter_map(|ip| match ip {
                    std::net::IpAddr::V4(ipv4) => Some(ipv4),
                    _ => None,
                })
                .collect();

            let (tx, rx) = std::sync::mpsc::channel();

            let sender_interface = interface.clone();
            let sender_thread = thread::spawn(move || {
                for ip in network_ips {
                    arp::send_arp_request(&sender_interface, source_ip, source_mac, ip);
                }
                tx.send(()).unwrap();
            });

            let receiver_interface = interface.clone();
            let receiver_thread = thread::spawn(move || {
                arp::receive_arp_responses(&receiver_interface);
            });

            rx.recv().unwrap();
            sender_thread.join().unwrap();
            receiver_thread.join().unwrap();
        }
    }
}

fn parse_ports(ports_str: &str) -> Vec<u16> {
    let mut ports = vec![];
    if ports_str.contains('-') {
        let parts: Vec<&str> = ports_str.split('-').collect();
        let start = parts[0].parse::<u16>().unwrap();
        let end = parts[1].parse::<u16>().unwrap();
        for port in start..=end {
            ports.push(port);
        }
    } else {
        ports.push(ports_str.parse::<u16>().unwrap());
    }
    ports
}

