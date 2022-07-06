use std::net::TcpStream;
use std::str;
use std::io::{self, BufRead, BufReader, Write};

fn main() {
    
    let mut stream = TcpStream::connect("127.0.0.1:8888")
                    .expect("couldnt connect to server");
    loop{
        let mut input = String::new();
        let mut buffer : Vec<u8> = Vec::new();
        io::stdin().read_line(&mut input).expect("failed to write to server");

    }

}
