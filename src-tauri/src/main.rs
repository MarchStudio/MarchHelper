// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json::json;
use std::net::UdpSocket;
use std::collections::HashSet;
use std::time::{Duration, Instant};

#[tauri::command]
async fn listen_udp() -> Option<String> {
    let socket = UdpSocket::bind("0.0.0.0:12345").unwrap();

    let timeout_secs = 1;
    let start_time = Instant::now();
    let mut client_addresses = HashSet::new();


    while start_time.elapsed() < Duration::from_secs(timeout_secs) {
        let mut buf = [0; 1024];
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                let data = &buf[..amt];
                let data_str = std::str::from_utf8(data).expect("Invalid UTF-8 data");
                if data_str == "Hi" {
                    client_addresses.insert(src.ip()); 
                } else {

                }
            }
            Err(_) => {
                // Handle timeout or other errors
            }
        }
    }

    let unique_clients: Vec<String> = client_addresses.iter().map(|ip| ip.to_string()).collect();
    let json_msg = json!({
        "unique_clients": unique_clients.len(),
        "client_addresses": unique_clients
    });
    Some(format!("{}", json_msg))
    
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![listen_udp])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
