extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_stdin_stdout;

use std::net::ToSocketAddrs;

use futures::Future;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;
use tokio_io::AsyncRead;

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let addr = std::env::args()
        .nth(1)
        .expect("You must provide an address in the form \"host:port\"")
        .to_socket_addrs()
        .expect("Failed to convert to socket address")
        .next()
        .expect("Failed to convert to socket address");

    let socket = TcpStream::connect(&addr, &handle);

    core.run(socket.and_then(|socket| {
        let (socket_read, socket_write) = socket.split();

        tokio_io::io::copy(socket_read, tokio_stdin_stdout::stdout(0)).join(tokio_io::io::copy(
            tokio_stdin_stdout::stdin(0),
            socket_write,
        ))
    })).expect("Failed to run loop");
}
