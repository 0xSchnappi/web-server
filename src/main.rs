use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:80").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
        .lines() // Lines方法创建一个迭代器，对内容流按行迭代，要使用Lines，必须先引入std::io::BufRead
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect(); // collect 消费掉迭代器

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}
