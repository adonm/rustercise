extern crate nanomsg;
use nanomsg::{Socket, Protocol, Error};
use std::io::{Read, Write};
use std::env;
use std::process;

static socket_address: &'static str = "tcp://127.0.0.1:8080";

/// Creating a new `Pull` socket type. Pull sockets can only receive messages
/// from a `Push` socket type.
fn create_socket() -> Result<Socket, Error> {
    let mut socket = try!(Socket::new(Protocol::Pull));

    // Create a new endpoint bound to the following protocol string. This returns
    // a new `Endpoint` that lives at-most the lifetime of the original socket.
    let mut endpoint = try!(socket.bind(socket_address));

    Ok(socket)
}

fn watch_socket(mut socket:Socket) -> Result<(), Error> {
    let mut msg = String::new();
    loop {
        try!(socket.read_to_string(&mut msg));
        println!("We got a message: {}", &*msg);
        msg.clear();
    }
}

fn pusher() -> Result<(), Error> {
    let mut socket = try!(Socket::new(Protocol::Push));
    let mut endpoint = try!(socket.connect(socket_address));

    socket.write(b"message in a bottle");

    endpoint.shutdown();
    Ok(())
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 { 
        println!("Pass an action (server, client) please");
        process::exit(1);
    }
    match args[1].as_ref() {
        "server" => {
            let mut socket = create_socket().unwrap();
            watch_socket(socket);
        },
        "client" => {
            pusher();
        },
        _ => {
            println!("Pass an action (server, client) please");
            process::exit(1);
        }
    }
}

