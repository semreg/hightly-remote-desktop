extern crate env_logger;
extern crate ws;

extern crate enigo;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;

mod input;

use input::Input;

fn main() {
    const PORT: i32 = 3012;
    const IP: &str = "0.0.0.0";

    env_logger::init();

    let addr = format!("{}:{}", IP, PORT);

    println!("\nListening on wss://{}\n", &addr);
    
    // Listen on an address and call the closure for each connection
    if let Err(error) = ws::listen(addr, |out| {
        // The handler needs to take ownership of out, so we use move
        move |msg| {
            println!("{}", msg);
            
            let msg_to_send: ws::Message = ws::Message::Text(String::from("OK"));

            let new_input = Input::new(&msg).unwrap();
            
            new_input.simulate();            

            out.send(msg_to_send)
        }
    }) {
        // Inform the user of failure
        println!("Failed to create WebSocket due to {:?}", error);
    }
}
