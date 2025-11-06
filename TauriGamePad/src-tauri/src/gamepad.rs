use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Stick {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Buttons {
    pub a: bool,
    pub b: bool,
    pub x: bool,
    pub y: bool,
    pub lb: bool,
    pub rb: bool,
    pub select: bool,
    pub start: bool,
    pub back: bool,
    pub home: bool,
    pub dpad_up: bool,
    pub dpad_down: bool,
    pub dpad_left: bool,
    pub dpad_right: bool,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Triggers {
    pub lt: u8,
    pub rt: u8,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GamepadState {
    pub left_stick: Stick,
    pub right_stick: Stick,
    pub buttons: Buttons,
    pub triggers: Triggers,
}

impl GamepadState {
    //updating btns
    pub fn update_button(&mut self, btn: &str, status: bool) {
        match btn {
            "A" => self.buttons.a = status,
            "B" => self.buttons.b = status,
            "X" => self.buttons.x = status,
            "Y" => self.buttons.y = status,
            "LB" => self.buttons.lb = status,
            "RB" => self.buttons.rb = status,
            "SELECT" => self.buttons.select = status,
            "START" => self.buttons.start = status,
            "BACK" => self.buttons.back = status,

            "HOME" => self.buttons.home = status,
            "UP" => self.buttons.dpad_up = status,
            "DOWN" => self.buttons.dpad_down = status,
            "LEFT" => self.buttons.dpad_left = status,
            "RIGHT" => self.buttons.dpad_right = status,
            _ => {}
        }
    }

    // Updating sticks
    pub fn update_stick(&mut self, stick: &str, x: i32, y: i32) {
        match stick {
            "stickLeft" => {
                self.left_stick.x = x;
                self.left_stick.y = y;
            }
            "stickRight" => {
                self.right_stick.x = x;
                self.right_stick.y = y;
            }
            _ => {}
        }
    }

    // Updating Triggers
    pub fn update_trigger(&mut self, trigger: &str, value: u8) {
        match trigger {
            "LT" => self.triggers.lt = value,
            "RT" => self.triggers.rt = value,
            _ => {}
        }
    }

    pub fn to_output_string(&self) -> String {
        let mut buttons_vec = Vec::new();
        
        if self.buttons.lb { buttons_vec.push("LB".to_string()); }
        if self.buttons.rb { buttons_vec.push("RB".to_string()); }
        if self.buttons.dpad_up { buttons_vec.push("UP".to_string()); }
        if self.buttons.dpad_down { buttons_vec.push("DOWN".to_string()); }
        if self.buttons.dpad_left { buttons_vec.push("LEFT".to_string()); }
        if self.buttons.dpad_right { buttons_vec.push("RIGHT".to_string()); }
        if self.buttons.y { buttons_vec.push("Y".to_string()); }
        if self.buttons.x { buttons_vec.push("X".to_string()); }
        if self.buttons.a { buttons_vec.push("A".to_string()); }
        if self.buttons.b { buttons_vec.push("B".to_string()); }
        if self.buttons.select { buttons_vec.push("SELECT".to_string()); }
        if self.buttons.home { buttons_vec.push("HOME".to_string()); }
        if self.buttons.back { buttons_vec.push("BACK".to_string()); }
        if self.buttons.start { buttons_vec.push("START".to_string()); }


        let output_json = serde_json::json!({
            "buttons": buttons_vec,
            "left_trigger": self.triggers.lt,
            "right_trigger": self.triggers.rt,
            "thumb_lx": self.left_stick.x,
            "thumb_ly": self.left_stick.y,
            "thumb_rx": self.right_stick.x,
            "thumb_ry": self.right_stick.y
        });

        output_json.to_string()
    }
}

pub fn new_shared_state() -> Arc<Mutex<GamepadState>> {
    Arc::new(Mutex::new(GamepadState::default()))
}