use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread::{self, Thread},
    time::Duration,
};

use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    println!("Listener: {:?}", listener);

    let thread_pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread_pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    // let _http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        _ => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        }
    };

    let content = fs::read_to_string(filename)
        .unwrap_or_else(|e| format!("Not found: {} - {}", filename, e.to_string()));
    let length = content.len();

    let response = format!("{status_line}\r\nContent-Length:{length}\r\n\r\n{content}");

    // println!("{request_line}: \r\n {:?}", response);

    stream.write_all(response.as_bytes()).unwrap();
}
