use std::thread;

use webserver::{HttpResponse, Server};

fn main() {
    let mut server = Server::bind("127.0.0.1:8088").unwrap();
    server.register(
        "/",
        Box::new(|req| HttpResponse::builder().body(req.body().clone()).build()),
    );

    thread::spawn(move || server.run());

    let client = reqwest::blocking::Client::new();
    let response = client
        .post("http://127.0.0.1:8088")
        .body("HELLO WORLD LOL")
        .send()
        .unwrap();
    let body = response.text().unwrap();

    println!("{body}");
}
