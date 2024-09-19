mod ffi;
mod poll;

use std::{
    collections::HashSet,
    env,
    io::{self, Read, Result, Write},
    net::TcpStream,
};

use ffi::Event;
use poll::Poll;

fn get_req(path: &str) -> String {
    format!(
        "GET {path} HTTP/1.1\r\n
            Host: localhost\r\n
        Connection: close\r\n\
        r\n"
    )
}

fn handle_events(
    events: &[Event],
    streams: &mut Vec<TcpStream>,
    handled_ids: &mut HashSet<usize>,
) -> Result<usize> {
    Ok(1)
}

fn main() -> Result<()> {
    let mut poll = Poll::new()?;

    let n_events = 5;
    let mut streams = vec![];

    let base_url = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("localhost"));

    let addr = format!("{}:8080", base_url);

    for i in 0..n_events {
        let delay = (n_events - i) * 1000;
        let url_path = format!("/{delay}/request-{i}");
        let request = get_req(&url_path);
        let mut stream = std::net::TcpStream::connect(addr)?;
        stream.set_nonblocking(true)?;
        stream.write_all(request.as_bytes())?;
        poll.registry()
            .register(&stream, i, ffi::EPOLLIN | ffi::EPOLLET)?;
        streams.push(stream);
    }

    let mut handled_ids = HashSet::new();

    let mut handled_events = 0;

    while handled_events < n_events {
        let mut events = Vec::with_capacity(10);
        poll.poll(&mut events, None)?;

        if events.is_empty() {
            println!("TIMEOUT (OR SPURIOUS EVENT NOTIFICATION)");
            continue;
        }

        handled_events += handle_events(&events, &mut streams, &mut handled_ids)?;
        println!("FINISHED");
    }
    Ok(())
}
