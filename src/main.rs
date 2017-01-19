//#![feature(io)]
use std::io;
use std::io::Read;
use std::str::FromStr;
use std::thread;
use std::time;
//use std::sync::mpsc;
use std::sync::mpsc::{Sender, channel};

mod reversal;
//pub use reversal::Reversal;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();
    let input = String::from_str(input.trim()).unwrap();

    let (tx, rx) = channel();
    //let (tx, _) = channel();

    let mut rev = reversal::Reversal::new(input, tx);
    print!("{}", rev);

    let chile = thread::spawn(move || { rev.build_states(); });
    while true {
        match rx.recv() {
            Ok(r) => {
                print!("{}\n", r);
                thread::sleep(time::Duration::from_millis(100));
            }
            Err(e) => break,
        }
    }
}
