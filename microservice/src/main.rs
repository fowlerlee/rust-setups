extern crate futures;
extern crate hyper;
extern crate net2;
extern crate num_cpus;
extern crate tokio_core;

use futures::future::FutureResult;
use futures::Stream;
use hyper::header::{ContentLength, Protocol};
use hyper::server::{Http, Request, Response, Service};
use hyper::{Get, StatusCode};
use net2::unix::UnixTcpBuilderExt;
use std::iter::Product;
use std::net::SocketAddr;
use std::sync::Arc;
use std::{thread, time};
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;

fn heavy_work() -> String {
    let duration = time::Duration::from_millis(100);
    thread::sleep(duration);
    "done".to_string()
}

#[derive(Clone, Copy)]
struct Echo;

impl Service for Echo {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;

    fn call(&self, req: Request) -> Self::Future {
        futures::future::ok(match (req.method(), req.path()) {
            (&Get, "/data") => {
                let b = heavy_work().into_bytes();
                Response::new()
                    .with_header(ContentLength(b.len() as u64))
                    .with_body(b)
            }
            _ => Response::new().with_status(StatusCode::NotFound),
        })
    }
}

fn serve(addr: &SocketAddr, protocol: &Http) {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let listener = net2::TcpBuilder::new_v4()
        .unwrap()
        .reuse_port(true)
        .unwrap()
        .bind(addr)
        .unwrap()
        .listen(128)
        .unwrap();
    let listener = TcpListener::from_listener(listener, addr, &handle).unwrap();
    core.run(listener.incoming().for_each(|(socket, addr)| {
        protocol.bind_connection(&handle, socket, addr, Echo);
        Ok(())
    }))
    .unwrap();
}

fn start_server(num: usize, addr: &str) {
    let addr = addr.parse().unwrap();

    let protocol = Arc::new(Http::new());

    {
        for _ in 0..num - 1 {
            let protocol = Arc::clone(&protocol);
            thread::spawn(move || serve(&addr, &protocol));
        }
    }
    serve(&addr, &protocol);
}

// #![feature(plugin)]
// #![plugin(rocket_codegen)]

// extern crate rocket;

// #[get("/")]
// fn blast_off() -> &'static str {
//     "you reached the route - Meep"
// }



fn main() {
    start_server(num_cpus::get(), "0.0.0.0:3000");
    // rocket::ignite().mount("/", routes![blast_off]).launch();
}
