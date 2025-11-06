use tokio::sync::broadcast;

#[derive(Debug)]
pub struct Agent {
    tx: broadcast::Sender<String>,
}

impl Agent {
    pub fn new() -> Agent {
        let (tx, _rx) = broadcast::channel::<String>(16);
        Agent { tx }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<String> {
        self.tx.subscribe()
    }

    pub fn send(&self, s: String) {
        // send повертає Result<usize, SendError<T>>
        if let Err(e) = self.tx.send(s) {
            eprintln!("Send error: {}", e);
        }
    }
}
