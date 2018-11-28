use std::net::TcpStream;
use std::thread;
use std::io::prelude::*;

fn main() {
    let stream_reader =
        TcpStream::connect("127.0.0.1:44321").expect("Failed to connect to TCP socket");
    let stream_writer = stream_reader
        .try_clone()
        .expect("Failed to clone TCP socket");

    let read = exchange(stream_reader, std::io::stdout());
    let write = exchange(std::io::stdin(), stream_writer);

    read.join().expect("Failed to join reader thread");
    write.join().expect("Failed to join writer thread");
}

fn exchange<R, W>(mut reader: R, mut writer: W) -> thread::JoinHandle<()>
where
    R: Read + Send + 'static,
    W: Write + Send + 'static,
{
    thread::spawn(move || {
        let mut buf = vec![0u8; 255];
        let mut buffer = buf.as_mut_slice();

        loop {
            if let Ok(_) = reader.read(&mut buffer) {
                writer.write(buffer).expect("failed to write to stdout");
            }
            clear(buffer);
        }
    })
}

fn clear(buf: &mut [u8]) {
    for i in buf.iter_mut() {
        *i = 0;
    }
}
