use std::net::TcpStream;
use std::str;
use std::io::{self, BufRead, BufReader, Write};

fn main() {
    
    let mut stream = TcpStream::connect("127.0.0.1:8888")
                    .expect("couldnt connect to server");
    loop{
        let mut input = String::new();
        let mut buffer : Vec<u8> = Vec::new();
        io::stdin().read_line(&mut input).expect("failed to read from stdin");
        stream.write(input.as_bytes()).expect("failed to write to server");
        let mut reader = BufReader::new(&stream);
        reader.read_until(b'\n', &mut buffer).expect("Could not read into buffer");
        print!("{}", str::from_utf8(&buffer).expect("could not write buffer to string"));
    }
}
