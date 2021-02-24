extern crate tetsy_nanomsg;

use tetsy_nanomsg::{Socket, Protocol};

use std::thread;

use std::io::Read;


fn puller(url: &str) {
    let mut input = Socket::new(Protocol::Pull).unwrap();
    let mut msg = String::new();

    input.bind(url).unwrap();
    println!("Puller listen on '{}'.", url);

    loop {
        match input.read_to_string(&mut msg) {
            Ok(_) => {
                println!("Puller received '{}'.", msg);

                thread::sleep_ms(1000); // fake some work ...
                msg.clear();
            },
            Err(err) => {
                println!("Puller failed '{}'.", err);
                break;
            }
        }
    }
}

fn usage() {
    println!("Usage: puller $url");
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        return usage()
    }

    puller(args[1].as_ref());
}