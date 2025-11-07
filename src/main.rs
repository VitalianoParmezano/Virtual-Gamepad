use std::string::String;
use std::thread;
use crate::websocketserver::websocketserver::WebsocketServer;
use futures_util::{SinkExt, StreamExt};
use tokio::io;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc;
use crate::agent::Agent;
use crate::virtual_gamepad::virtual_gamepad::VirtualGamepad;
use std::net::UdpSocket;

mod virtual_gamepad;
mod agent;
mod websocketserver;

#[tokio::main]
async fn main() {

    let agent = Agent::new();
    let gamepad_receiver = agent.subscribe();

    let ip = get_local_ip().unwrap_or("127.0.0.1".to_string());
    let address = format!("{}:9001", ip);
    let mut server = WebsocketServer::new(&address, agent).await;
    println!("✅ WebSocket server started on {}", address);


    let mut gamepad = VirtualGamepad::new(gamepad_receiver);

    thread::spawn(move || {
        let mut gamepad = gamepad;
        tokio::runtime::Runtime::new().unwrap().block_on(async move {
            gamepad.start_receiving().await;
        });
    });




    server.init().await;

}


fn get_local_ip() -> Option<String> {
    // Створюємо UDP сокет і "підключаємось" до публічної адреси (без фактичного відправлення)
    if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
        if socket.connect("8.8.8.8:80").is_ok() {
            if let Ok(local_addr) = socket.local_addr() {
                return Some(local_addr.ip().to_string());
            }
        }
    }
    None
}

// pub async fn read_std_in() -> String {
//     let stdin = io::stdin();                // отримуємо stdin
//     let mut reader = BufReader::new(stdin); // обгортаємо у буфер
//     let mut line = String::new();           // сюди буде записаний рядок
//
//     // чекаємо асинхронно на введення
//     match reader.read_line(&mut line).await {
//         Ok(_) => line.trim_end().to_string(), // прибираємо кінцевий \n
//         Err(_) => String::new(),             // при помилці повертаємо порожній рядок
//     };
//
//     println!("{line}");
//
//     line
// }