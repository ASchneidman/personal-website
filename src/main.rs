use std::{
    env, fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Expects single argument containing a valid address and port to host on.");
        return;
    }
    match TcpListener::bind(args[1].as_str()) {
        Err(e) => println!("Failed to bind: {}", e),
        Ok(listener) => {
            for stream in listener.incoming() {
                match stream {
                    Err(e) => println!("Failed to receive stream: {}", e),
                    Ok(s) => handle_connection(s),
                }
            }
        }
    }

}

fn respond_to_request(request_line: &String, mut stream: &TcpStream) {
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "src/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "src/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    match stream.write_all(response.as_bytes()) {
        Ok(_) => return,
        Err(e) => {
            println!("Failed to write response: {}", e);
        }
    }

}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    match buf_reader.lines().next() {
        None => return,
        Some(t) => {
            match t {
                Err(e) => println!("Failed to read request: {}", e),
                Ok(t) => respond_to_request(&t, &stream),
            }
        }
    }
}
