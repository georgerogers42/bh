use std::io;
use std::io::prelude::*;

fn bh(r: &mut BufRead) -> io::Result<()> {
    for b in r.bytes() {
        b?;
    }
    Ok(())
}

fn main() {
    bh(&mut io::stdin().lock()).unwrap();
}
