extern crate env_logger;
extern crate ws;
extern crate web_view;
extern crate webbrowser;
extern crate enigo;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;

mod input;

use input::Input;

use web_view::*;

fn main() {
    const PORT: i32 = 6061;
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

    run_view(include_str!("../static/layouts/body.html"));
}

fn run_view (render_body: &str) {
    let html = format!(r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <meta http-equiv="X-UA-Compatible" content="ie=edge">
            <title>Document</title>
            <style>{bootstrap}</style>
            <style>{mdb}</style>
            <style>{css}</style>
        </head>
        <body>
            {body}
        </body>
        <script>{script}</script>
        </html>
    "#,
    body = render_body,
    css = include_str!("../static/styles/main.css"),
    script = include_str!("../static/javascripts/main.js"),
    mdb = include_str!("../static/bootstrap/mdb.min.css"),
    bootstrap = include_str!("../static/bootstrap/bootstrap.min.css"));

    WebViewBuilder::new()
        .title("Higtlly Remote Desktop")
        .content(Content::Html(html))
        .size(420, 300)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|webview, arg| {
            match arg {
                "open" => {
                    webbrowser::open("https://hightly.semreg.me/stream").unwrap();
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
