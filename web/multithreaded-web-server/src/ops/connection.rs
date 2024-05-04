use std::{
    collections::HashMap,
    fs,
    io::{prelude::*, BufReader},
    net::TcpStream,
};

/// Main type to create a request -> response map
type Responses<'a> = HashMap<&'a str, (&'a str, String)>;

#[derive(Debug, Clone)]
pub struct Server {
    directory: String,
    port: u32,
}

impl Server {
    pub fn new(directory: String, port: u32) -> Self {
        Self { directory, port }
    }

    /// Creates the response to each request header the server may get
    /// and ads a default response `"404"`, to all other requests
    pub fn responses(&self) -> Responses<'static> {
        let mut responses = Responses::new();

        // add each response for different requests
        responses.insert(
            "GET / HTTP/1.1",
            ("HTTP/1.1 200 OK", format!("{}/index.html", self.directory)),
        );
        responses.insert(
            "404",
            (
                "HTTP/1.1 404 NOT FOUND",
                format!("{}/404.html", self.directory),
            ),
        );

        responses
    }

    pub fn port(&self) -> u32 {
        self.port
    }
}

/// Given the request, maps it to the corresponding response.
/// If the request does not exist in the response map, it
/// defaults to entry `"404"`.
pub fn create_response(responses: &Responses, request: String) -> String {
    let (mut status_line, mut filename) = responses.get("404").unwrap().clone();

    match responses.get(request.as_str()) {
        Some(r) => (status_line, filename) = r.clone(),
        None => (),
    }

    let contents = fs::read_to_string(filename).expect("Unable to read file");
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-length: {length}\r\n\r\n{contents}");

    response
}

/// Handles each connection, reading the header, obtaining the request
/// and sending the response.
pub fn handle_connection(responses: &Responses, mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let response = create_response(&responses, request_line);

    stream.write_all(response.as_bytes()).unwrap();
}
