extern crate futures;

use std::io::{self, BufRead, Error};
use std::thread;

use futures::{Future, Sink, Stream, Poll};
use futures::stream::BoxStream;
use futures::sync::mpsc::channel;
use futures::future::Map;

fn add_echo<F>(future: F) -> Map<F, fn(String) -> String>
    where F: Future<Item=String>
{
    fn add(s: String) -> String { format!("echo: {:?}",s) }
    future.map(add)
}

fn stdin() -> BoxStream<String, io::Error> {
    let (mut tx, rx) = channel(1);
    thread::spawn(move || {
        let input = io::stdin();
        for line in input.lock().lines() {
//            let line = add_echo(line);
            match tx.send(line).wait() {
                Ok(s) => tx = s,
                Err(_) => break,
            }
        }
    });

    return rx.then(|e| e.unwrap()).boxed();
}

fn main() {
    stdin()
        .for_each(|string| {
            println!("{}", string);
            Ok(())
        })
        .wait()
        .unwrap();
}
