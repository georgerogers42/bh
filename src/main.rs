use std::io;
use std::io::prelude::*;

fn bh<R: BufRead>(r: R) -> io::Result<()> {
    for b in r.bytes() {
        b?;
    }
    Ok(())
}

fn main() {
    bh(io::stdin().lock()).unwrap();
}
