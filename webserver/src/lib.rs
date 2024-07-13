use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader, BufWriter, Read, Write},
    net::TcpListener,
    string::FromUtf8Error,
};

pub type RequestHandler = dyn Fn(HttpRequest) -> HttpResponse + Send + Sync;

pub struct HttpRequest {
    method: String,
    path: String,
    body: Vec<u8>,
    headers: HashMap<String, String>,
}

impl HttpRequest {
    pub fn method(&self) -> &String {
        &self.method
    }

    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn get_header(&self, key: &str) -> Option<&String> {
        self.headers.get(&key.to_lowercase())
    }

    pub fn body(&self) -> &Vec<u8> {
        &self.body
    }

    pub fn text(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.body.clone())
    }
}

pub struct HttpResponse {
    body: Vec<u8>,
    headers: HashMap<String, String>,
}

impl HttpResponse {
    pub fn builder() -> HttpResponseBuilder {
        HttpResponseBuilder::new()
    }
}

impl From<&'static str> for HttpResponse {
    fn from(body: &'static str) -> HttpResponse {
        From::from(String::from(body))
    }
}

impl From<String> for HttpResponse {
    fn from(body: String) -> HttpResponse {
        HttpResponse {
            body: body.into_bytes(),
            headers: HashMap::new(),
        }
    }
}

pub struct HttpResponseBuilder {
    body: Option<Vec<u8>>,
    headers: HashMap<String, String>,
}

impl HttpResponseBuilder {
    pub fn new() -> HttpResponseBuilder {
        HttpResponseBuilder {
            body: None,
            headers: HashMap::new(),
        }
    }

    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    pub fn header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key.to_lowercase(), value);
        self
    }

    pub fn build(self) -> HttpResponse {
        HttpResponse {
            body: self.body.unwrap_or_else(|| Vec::new()),
            headers: self.headers,
        }
    }
}

pub struct Server {
    listener: TcpListener,
    handlers: HashMap<String, Box<RequestHandler>>,
}

impl Server {
    pub fn bind(bind: &str) -> Result<Server, io::Error> {
        Ok(Server {
            listener: TcpListener::bind(bind)?,
            handlers: HashMap::new(),
        })
    }

    pub fn run(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let mut reader = BufReader::new(stream.try_clone().unwrap());
                    let mut buf = String::new();
                    reader.read_line(&mut buf).unwrap();
                    let status_parts: Vec<_> = buf.split(" ").collect();
                    let method = String::from(status_parts[0]);
                    let path = String::from(status_parts[1]);
                    let http_ver = String::from(status_parts[2].trim());
                    self.handlers.get(&path).and_then(move |f| {
                        let mut headers = HashMap::new();
                        let mut buf = String::new();
                        loop {
                            buf.clear();
                            reader.read_line(&mut buf).unwrap();
                            if let Some(idx) = buf.find(":") {
                                let (key, mut value) = buf.split_at(idx);
                                value = value[1..].trim();
                                headers.insert(key.to_lowercase(), String::from(value));
                            } else {
                                break;
                            }
                        }
                        let content_length = headers.get("content-length");
                        let mut body = Vec::new();
                        if let Some(length) = content_length {
                            let length: usize = length.parse().unwrap();
                            body.reserve_exact(length);
                            reader
                                .by_ref()
                                .take(length as u64)
                                .read_to_end(&mut body)
                                .unwrap();
                        }
                        let request = HttpRequest {
                            method,
                            path,
                            headers,
                            body,
                        };
                        let mut response = f(request);
                        let mut writer = BufWriter::new(stream);
                        response
                            .headers
                            .insert(String::from("connection"), String::from("close"));
                        response.headers.insert(
                            String::from("content-length"),
                            response.body.len().to_string(),
                        );
                        writer
                            .write_all(format!("{} 200 OK\r\n", http_ver).as_bytes())
                            .unwrap();
                        for (key, value) in response.headers.iter() {
                            writer.write_all(key.as_bytes()).unwrap();
                            writer.write_all(": ".as_bytes()).unwrap();
                            writer.write_all(value.as_bytes()).unwrap();
                            writer.write_all("\r\n".as_bytes()).unwrap();
                        }
                        writer.write_all("\r\n".as_bytes()).unwrap();
                        writer.write_all(&response.body).unwrap();
                        Some(())
                    });
                }
                Err(_) => {}
            }
        }
    }

    pub fn register(&mut self, path: &str, handler: Box<RequestHandler>) {
        self.handlers.insert(String::from(path), handler);
    }
}

#[cfg(test)]
mod test;
