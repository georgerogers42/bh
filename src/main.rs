use std::io;
use std::io::prelude::*;

fn sink<E, X, I: Iterator<Item=Result<E, X>>>(i: I) -> Result<(), X> {
    for e in i {
        e?;
    }
    Ok(())
}

fn main() {
    sink(io::stdin().lock().bytes()).unwrap();
}
