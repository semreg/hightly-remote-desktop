extern crate env_logger;
extern crate ws;
extern crate serde_json;
extern crate serde;
extern crate enigo;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;

use enigo::*;

#[derive(Serialize, Deserialize)]
struct Command {
    action: String,
    payload: CommandPayload
}

#[derive(Serialize, Deserialize)]
struct CommandPayload {
    coords: [i32; 2],
    deltaY: i32,
    key: String
}

custom_derive! {
    #[derive(Debug, EnumFromStr)]
    enum Action {
        MouseMove,
        Wheel,
        MouseDown,
        MouseUp,
        MouseRight,
        KeyDown,
        KeyUp
    }
}

fn main() {
    const PORT: i32 = 3012;

    env_logger::init();

    let addr = format!("192.168.0.105:{}", PORT);

    println!("\nListening on wss://{}\n", &addr);
    
    // Listen on an address and call the closure for each connection
    if let Err(error) = ws::listen(addr, |out| {
        // The handler needs to take ownership of out, so we use move
        move |msg| {
            println!("{}", msg);
            handle_msg(&msg);

            let msg_to_send: ws::Message = ws::Message::Text(String::from("OK"));

            out.send(msg_to_send)
        }
    }) {
        // Inform the user of failure
        println!("Failed to create WebSocket due to {:?}", error);
    }
}

fn handle_msg (msg: &ws::Message) {
    let mut enigo = Enigo::new();

    let msg_to_str = msg.as_text().unwrap();
    let data: Command = serde_json::from_str(msg_to_str).unwrap();

    // Convert data.action to `Action` enum
    let action: Action = data.action
        .to_string()
        .parse()
        .unwrap();

    match action {
        Action::MouseDown => {
            enigo.mouse_move_to(data.payload.coords[0], data.payload.coords[1]);
            enigo.mouse_down(MouseButton::Left);
        },
        Action::MouseUp => {
            enigo.mouse_move_to(data.payload.coords[0], data.payload.coords[1]);
            enigo.mouse_up(MouseButton::Left);
        },
        Action::MouseRight => {
            enigo.mouse_move_to(data.payload.coords[0], data.payload.coords[1]);
            enigo.mouse_click(MouseButton::Right);
        },
        Action::MouseMove => {
            enigo.mouse_move_to(data.payload.coords[0], data.payload.coords[1]);
        },
        Action::Wheel => {
            enigo.mouse_scroll_y(data.payload.deltaY);
        },
        Action::KeyDown => {
            let key_string = data.payload.key.to_string();

            simulate_key_event(&mut enigo, key_string, "DOWN")
        },
        Action::KeyUp => {
            let key_string = data.payload.key.to_string();

            simulate_key_event(&mut enigo, key_string, "UP")            
        },
        _ => ()
    }
}

fn simulate_key_event (enigo: &mut Enigo, key_string: String, action: &str) {
    let key: Key = match key_string.as_ref() {
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

    match action {
        "DOWN" => enigo.key_down(key),
        "UP" => enigo.key_up(key),
        _ => ()
    }
}