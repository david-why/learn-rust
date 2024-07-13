use std::thread;

use reqwest::Error;

use super::*;

fn setup<F>(port: u16, path: &'static str, handler: F)
where
    F: Fn(HttpRequest) -> HttpResponse + Send + Sync + 'static,
{
    let mut server = Server::bind(&format!("127.0.0.1:{port}")).unwrap();

    server.register(path, Box::new(handler));

    thread::spawn(move || server.run());
}

#[test]
fn get_root() -> Result<(), Error> {
    setup(10061, "/", |_req| "Hello, World!".into());

    let body = reqwest::blocking::get("http://localhost:10061")?.text()?;

    assert_eq!(body, "Hello, World!");

    Ok(())
}

#[test]
fn get_root_header() -> Result<(), Error> {
    setup(10062, "/", |req| {
        req.get_header("x-test").unwrap().clone().into()
    });

    let client = reqwest::blocking::Client::new();
    let body = client
        .get("http://localhost:10062")
        .header("X-tEsT", "some value")
        .send()?
        .text()?;

    assert_eq!(body, "some value");

    Ok(())
}

#[test]
fn post_root() -> Result<(), Error> {
    setup(10063, "/", |req| req.text().unwrap().into());

    let client = reqwest::blocking::Client::new();
    let body = client
        .post("http://localhost:10063")
        .body("This is a body")
        .send()?
        .text()?;

    assert_eq!(body, "This is a body");

    Ok(())
}

#[test]
fn response_headers() -> Result<(), Error> {
    setup(10064, "/", |_req| {
        HttpResponse::builder()
            .header(String::from("x-test"), String::from("some value"))
            .build()
    });

    let client = reqwest::blocking::Client::new();
    let response = client.get("http://localhost:10064").send()?;
    let value = response.headers().get("x-test").unwrap();

    assert_eq!(value, "some value");

    Ok(())
}
