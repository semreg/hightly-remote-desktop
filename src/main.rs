extern crate env_logger;
extern crate ws;
extern crate web_view;
extern crate webbrowser;
extern crate enigo;
extern crate base64;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;

mod input;

use input::Input;
use web_view::*;
use base64::{encode, decode};

fn main() {
    // TODO: Generate random port and test if it is open
    const PORT: u16 = 6061;
    const IP: &str = "0.0.0.0";

    env_logger::init();

    let addr = format!("{}:{}", IP, PORT);

    println!("\nListening on wss://{}\n", &addr);

    std::thread::spawn(move || {
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
    });

    run_view(PORT)
}

fn run_view (port: u16) {
    let encoded_port = encode(&port.to_string());

    let html = format!(r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <meta http-equiv="X-UA-Compatible" content="ie=edge">
            <title>Document</title>
            <style>{fontawesome}</style>
            <style>{bootstrap}</style>
            <style>{mdb}</style>
            <style>{css}</style>
        </head>
        <body>
            {nav}
            <div class="container mt-4">
                <h2 class="card-title h2 text-center">You are ready to go!</h2>
                <p class="card-text text-center">Remote desktop functionality is now ready to use! Click "Open App" to start streaming!</p>
                <p class='mt-4 blue-text text-center font-weight-bold mb-0'>Grab this link or click the button!</p>
                <div class='md-form container' style="text-align: left; color: #777">
                    <input readonly type='text' id="inputIconEx2" class='form-control' value="https://hightly.semreg.me/stream/{encoded_port}"/>
                </div>
                {buttons}
            </div>
        </body>
        <script>{script}</script>
        </html>
    "#,
    // port = port,
    encoded_port = encoded_port,
    nav = include_str!("../static/components/nav.html"),
    buttons = include_str!("../static/components/buttons.html"),
    css = include_str!("../static/styles/main.css"),
    script = include_str!("../static/javascripts/main.js"),
    mdb = include_str!("../static/bootstrap/mdb.min.css"),
    fontawesome = include_str!("../static/fontawesome/fontawesome.min.css"),
    bootstrap = include_str!("../static/bootstrap/bootstrap.min.css"));

    WebViewBuilder::new()
        .title("Higtlly Remote Desktop")
        .content(Content::Html(html))
        .size(420, 380)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg| {
            match arg {
                "open" => {
                    webbrowser::open(format!("https://hightly.semreg.me/stream/{}", encoded_port).as_ref()).unwrap();
                },
                "exit" => {
                    webview.terminate();
                }
                _ => unimplemented!(),
            };
            Ok(())
        })
        .run()
        .unwrap();
}
