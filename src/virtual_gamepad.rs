pub mod virtual_gamepad {
    use std::any::Any;
    use std::sync::Arc;
    use std::sync::mpsc::Receiver;
    use serde::Deserialize;
    use vigem_client::{Client, Xbox360Wired, XButtons};

    #[derive(Deserialize, Debug)]
    struct GamepadState {
        buttons: Vec<String>,
        left_trigger: u8,
        right_trigger: u8,
        thumb_lx: i16,
        thumb_ly: i16,
        thumb_rx: i16,
        thumb_ry: i16,
    }

    pub struct VirtualGamepad {
        connector: Xbox360Wired<Client>,
        gamepad: vigem_client::XGamepad,
        receiver: tokio::sync::broadcast::Receiver<String>,
    }

    impl VirtualGamepad {
        pub fn new(receiver: tokio::sync::broadcast::Receiver<String>) -> Self {


            let id = vigem_client::TargetId::XBOX360_WIRED;

            let client = Client::connect().expect("Failed to connect to ViGEmBus");
            let mut connector = Xbox360Wired::new(client, id);

            println!("{:?}",connector.plugin());

            println!("{:?}",connector.wait_ready());


            let gamepad = vigem_client::XGamepad{
                buttons: vigem_client::XButtons!(UP),
                .. Default::default()
            };


            Self {  connector,gamepad, receiver }
        }

        pub fn press_button_a(&mut self) {
            self.gamepad.buttons = XButtons!(A | X);
            self.connector.update(&self.gamepad);
            println!("a pressed");
        }

        pub async fn start_receiving(&mut self) {
            println!("Gamepad is ready to receive");
            loop {
                let data = self.receiver.recv().await.unwrap();
                self.use_controller(data);
            }
        }

        fn use_controller(&mut self, data: String) {
            // парсимо JSON
            let state: GamepadState = match serde_json::from_str(&data) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Failed to parse JSON: {:?}", e);
                    return;
                }
            };

            // скидаємо всі кнопки
            self.gamepad.buttons.raw = 0;

            // додаємо кнопки, які прийшли у JSON
            for btn in state.buttons {
                match btn.as_str() {
                    "A" => self.gamepad.buttons.raw |= XButtons::A,
                    "B" => self.gamepad.buttons.raw |= XButtons::B,
                    "X" => self.gamepad.buttons.raw |= XButtons::X,
                    "Y" => self.gamepad.buttons.raw |= XButtons::Y,
                    "UP" => self.gamepad.buttons.raw |= XButtons::UP,
                    "DOWN" => self.gamepad.buttons.raw |= XButtons::DOWN,
                    "LEFT" => self.gamepad.buttons.raw |= XButtons::LEFT,
                    "RIGHT" => self.gamepad.buttons.raw |= XButtons::RIGHT,
                    "LB" => self.gamepad.buttons.raw |= XButtons::LB,
                    "RB" => self.gamepad.buttons.raw |= XButtons::RB,
                    "START" => self.gamepad.buttons.raw |= XButtons::START,
                    "BACK" => self.gamepad.buttons.raw |= XButtons::BACK,
                    _ => {}
                }
            }

            // оновлюємо тригери та стики
            self.gamepad.left_trigger = state.left_trigger;
            self.gamepad.right_trigger = state.right_trigger;
            self.gamepad.thumb_lx = state.thumb_lx;
            self.gamepad.thumb_ly = state.thumb_ly;
            self.gamepad.thumb_rx = state.thumb_rx;
            self.gamepad.thumb_ry = state.thumb_ry;

            // відправляємо стан геймпада у ViGEm
            self.connector.update(&self.gamepad);

            println!("Updated gamepad: {:?}", self.gamepad);
        }

    }

}