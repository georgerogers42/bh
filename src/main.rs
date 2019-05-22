use std::io;
use std::io::prelude::*;

fn drain<R: Read>(r: R) -> io::Result<()> {
    for b in r.bytes() {
        b?;
    }
    Ok(())
}

fn main() {
    drain(io::stdin().lock()).unwrap();
}
