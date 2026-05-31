use std::env;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: port-scanner <ip> <max-port>");
        println!("Example: port-scanner 127.0.0.1 1000");
        return;
    }

    let ip = &args[1];
    let max_port: u16 = args[2].parse().unwrap_or(1024);

    println!("[*] Asynchronous Multi-threaded Port Scanner targeting: {}", ip);
    let mut threads = vec![];

    for port in 1..=max_port {
        let ip_clone = ip.clone();
        let handle = thread::spawn(move || {
            let addr = format!("{}:{}", ip_clone, port);
            if let Ok(socket_addr) = addr.parse::<SocketAddr>() {
                if TcpStream::connect_timeout(&socket_addr, Duration::from_millis(800)).is_ok() {
                    println!("[+] Port {} is OPEN", port);
                }
            }
        });
        threads.push(handle);
    }

    for t in threads {
        let _ = t.join();
    }
    println!("[+] Port scan completed.");
}