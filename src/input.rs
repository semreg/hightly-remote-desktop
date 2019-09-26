extern crate ws;
extern crate enigo;
extern crate serde_json;
extern crate serde;

use enigo::{Enigo, Key, MouseButton, MouseControllable, KeyboardControllable};

#[derive(Serialize, Deserialize)]
pub struct Input {
    action_type: String, // to InputType
    payload: InputPayload
}

custom_derive! {
    #[derive(Debug, EnumFromStr)]
    enum InputActionType {
        MouseMove,
        Wheel,
        MouseDown,
        MouseUp,
        MouseRight,
        KeyDown,
        KeyUp
    }
}

#[derive(Serialize, Deserialize)]
struct InputPayload {
    coords: (i32, i32),
    deltaY: i32,
    key: String
}

impl Input {
    pub fn new (msg: &ws::Message) -> Input {
        let msg_to_str = msg.as_text().unwrap();
        serde_json::from_str(msg_to_str).unwrap()
    }
}

impl Input {
    pub fn simulate (&self) {
        let action_type: InputActionType = self.action_type
            .to_string()
            .parse()
            .unwrap();

        let mut enigo = Enigo::new();

        let InputPayload { 
            deltaY: delta_y,
            coords: (x, y), 
            key 
        } = &self.payload;

        let key_string = key.to_string();

        match action_type {
            InputActionType::MouseDown => {
                enigo.mouse_move_to(*x, *y);
                enigo.mouse_down(MouseButton::Left);
            },
            InputActionType::MouseUp => {
                enigo.mouse_move_to(*x, *y);
                enigo.mouse_up(MouseButton::Left);
            },
            InputActionType::MouseRight => {
                enigo.mouse_move_to(*x, *y);
                enigo.mouse_click(MouseButton::Right);
            },
            InputActionType::MouseMove => {
                enigo.mouse_move_to(*x, *y);
            },
            InputActionType::Wheel => {
                enigo.mouse_scroll_y(*delta_y);
            },
            InputActionType::KeyDown => {
                Input::simulate_key_event(&mut enigo, key_string, "DOWN")
            },
            InputActionType::KeyUp => {
                Input::simulate_key_event(&mut enigo, key_string, "UP")            
            },
            _ => ()
        }
    }
    fn simulate_key_event (enigo: &mut Enigo, key_string: String, action_type: &str) {
        let key: Key = match key_string.as_ref() {
            "F12" => Key::F12,
            "F11" => Key::F11,
            "F10" => Key::F10,
            "F9" => Key::F9,
            "F8" => Key::F8,
            "F7" => Key::F7,
            "F6" => Key::F6,
            "F5" => Key::F5,
            "F4" => Key::F4,
            "F3" => Key::F3,
            "F2" => Key::F2,
            "F1" => Key::F1,
            "Delete" => Key::Delete,
            "PageUp" => Key::PageUp,
            "ArrowRight" => Key::RightArrow,
            "ArrowLeft" => Key::LeftArrow,
            "ArrowDown" => Key::DownArrow,
            "ArrowUp" => Key::UpArrow,
            "Enter" => Key::Return,
            "Alt" => Key::Alt,
            "OS" | "Command" | "Windows" => Key::Meta,
            "CapsLock" => Key::CapsLock,
            "Tab" => Key::Tab,
            "Control" => Key::Control,
            "Shift" => Key::Shift,
            "Backspace" => Key::Backspace,
            _ => {
                if key_string.len() == 1 {
                    let key_vec: Vec<char> = key_string.chars().collect();
                    Key::Layout(key_vec[0])
                } else {
                    Key::Meta
                }
            }
        };

        match action_type {
            "DOWN" => enigo.key_down(key),
            "UP" => enigo.key_up(key),
            _ => ()
        }
    }
}
