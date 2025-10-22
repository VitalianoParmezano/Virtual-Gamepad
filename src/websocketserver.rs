// Only input data logic
// all received jsons sending to agent
pub mod websocketserver {
    use std::sync::mpsc::Sender;
    use tokio::net::{TcpListener, TcpStream};
    use tokio_tungstenite::{accept_async, WebSocketStream};
    use tokio_tungstenite::tungstenite::Message;
    use futures_util::{StreamExt, SinkExt};
    use futures_util::stream::SplitSink;
    use serde_json::Value;
    use crate::agent::Agent;

    #[derive(Debug)]
    pub struct WebsocketServer {
        listener: TcpListener,
        websocket_sender: Option<SplitSink<WebSocketStream<TcpStream>, Message>>,
        agent: Agent,
    }

    impl WebsocketServer {
        pub async fn new(addr: &str, agent: Agent) -> Self {
            let listener = TcpListener::bind(addr).await.unwrap();
            WebsocketServer { listener, websocket_sender: None, agent }
        }

        pub async fn init(&mut self) {
            while let Ok((stream, addr)) = self.listener.accept().await {
                println!("accepted a connection from {}", addr);

                let ws_stream = accept_async(stream).await.unwrap();
                println!("connection established with {:?}", addr);

                let (ws_sender, mut ws_receiver) = ws_stream.split();
                self.websocket_sender = Some(ws_sender); // зберігаємо відправник

                // слухаємо повідомлення
                while let Some(msg) = ws_receiver.next().await {
                    match msg {
                        Ok(Message::Text(text)) => {
                            println!("Received at websocket {}", text);
                            let text = text.trim();
                            self.send_to_agent(text).await;

                            // if is_json(text) {
                            // }

                        }
                        _ => {}
                    }
                }

                self.websocket_sender = None; // delete socket if sender offline
            }
        }
        pub async fn send_to_client(&mut self, message: &str) {
            if let Some(sender) = &mut self.websocket_sender {
                let _ = sender.send(Message::Text(message.into())).await;
            } else {
                println!("No client connected");
            }
        }
        async fn send_to_agent(&mut self,message : &str){
            println!("Sending {}", message);
            self.agent.send(message.into());
        }
    }

    pub fn is_json(s: &str) -> bool {
        serde_json::from_str::<Value>(s).is_ok()
    }
}
